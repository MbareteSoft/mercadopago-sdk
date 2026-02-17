use mercadopago_sdk::MercadoPagoClient;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_client_authentication_header() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";

    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(header("Authorization", &format!("Bearer {}", token)))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client.get("/test").send().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_client_idempotency_header() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let idempotency_key = "test-key-123";

    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("POST"))
        .and(path("/test"))
        .and(header("X-Idempotency-Key", idempotency_key))
        .respond_with(ResponseTemplate::new(201))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .post("/test")
        .header("X-Idempotency-Key", idempotency_key)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn test_client_retry_on_429() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";

    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    // First call returns 429, second call returns 200
    Mock::given(method("GET"))
        .and(path("/retry"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "1"))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/retry"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let res = client.get("/retry").send().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_client_retry_exhaustion() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";

    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .max_retries(2)
        .build()
        .unwrap();

    // Always return 429
    Mock::given(method("GET"))
        .and(path("/retry-exhaust"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
        .mount(&mock_server)
        .await;

    let res = client.get("/retry-exhaust").send().await;
    match res {
        Err(mercadopago_sdk::error::Error::ApiError { status, .. }) => {
            assert_eq!(status, 429);
        }
        _ => panic!("Expected ApiError with status 429 after retry exhaustion"),
    }
}

#[tokio::test]
async fn test_create_payment() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let payment_request = mercadopago_sdk::models::payments::PaymentRequest {
        transaction_amount: 100.0,
        payment_method_id: "pix".to_string(),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/v1/payments"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 123456,
            "status": "approved",
            "transaction_amount": 100.0
        })))
        .mount(&mock_server)
        .await;

    let payment = client.create_payment(payment_request).await.unwrap();
    assert_eq!(payment.id, 123456);
    assert_eq!(payment.status, "approved");
}

#[tokio::test]
async fn test_get_payment() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/v1/payments/123456"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123456,
            "status": "approved",
            "transaction_amount": 100.0
        })))
        .mount(&mock_server)
        .await;

    let payment = client.get_payment(123456).await.unwrap();
    assert_eq!(payment.id, 123456);
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/v1/payments/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "message": "Payment not found",
            "error": "not_found",
            "status": 404
        })))
        .mount(&mock_server)
        .await;

    let res = client.get_payment(999).await;
    match res {
        Err(mercadopago_sdk::error::Error::ApiError { status, .. }) => {
            assert_eq!(status, 404);
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn test_create_preference() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let preference_request = mercadopago_sdk::models::preferences::PreferenceRequest {
        items: vec![mercadopago_sdk::models::preferences::PreferenceItem {
            title: "Test Item".to_string(),
            quantity: 1,
            unit_price: 100.0,
            ..Default::default()
        }],
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/checkout/preferences"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "pref-123",
            "items": [],
            "init_point": "http://init.point",
            "sandbox_init_point": "http://sandbox.init.point"
        })))
        .mount(&mock_server)
        .await;

    let preference = client.create_preference(preference_request).await.unwrap();
    assert_eq!(preference.id, "pref-123");
    assert_eq!(preference.init_point, "http://init.point");
}

#[tokio::test]
async fn test_put_request() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("PUT"))
        .and(path("/v1/resource/123"))
        .and(header("Authorization", &format!("Bearer {}", token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"updated": true})))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .put("/v1/resource/123")
        .json(&json!({"name": "updated"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_delete_request() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path("/v1/resource/456"))
        .and(header("Authorization", &format!("Bearer {}", token)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client.delete("/v1/resource/456").send().await.unwrap();
    assert_eq!(res.status(), 204);
}

#[tokio::test]
async fn test_patch_request() {
    let mock_server = MockServer::start().await;
    let token = "TEST_TOKEN";
    let client = MercadoPagoClient::builder(token)
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("PATCH"))
        .and(path("/v1/resource/789"))
        .and(header("Authorization", &format!("Bearer {}", token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"patched": true})))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .patch("/v1/resource/789")
        .json(&json!({"field": "value"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_search_payments() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/v1/payments/search"))
        .and(query_param("external_reference", "REF-001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paging": { "total": 1, "offset": 0, "limit": 30 },
            "results": [{
                "id": 111,
                "status": "approved",
                "transaction_amount": 50.0
            }]
        })))
        .mount(&mock_server)
        .await;

    let response = client.search_payments("REF-001").await.unwrap();
    assert_eq!(response.paging.total, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, 111);
}

#[tokio::test]
async fn test_search_payments_generic() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/v1/payments/search"))
        .and(query_param("status", "approved"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paging": { "total": 2, "offset": 0, "limit": 30 },
            "results": [
                { "id": 1, "status": "approved", "transaction_amount": 10.0 },
                { "id": 2, "status": "approved", "transaction_amount": 20.0 }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client
        .search_payments_generic(&[("status", "approved")])
        .await
        .unwrap();
    assert_eq!(response.paging.total, 2);
    assert_eq!(response.results.len(), 2);
}

#[tokio::test]
async fn test_create_qr_order() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let request = mercadopago_sdk::models::instore::QrOrderRequest {
        total_amount: 100.0,
        items: vec![mercadopago_sdk::models::instore::QrOrderItem {
            title: "Item".to_string(),
            unit_price: 100.0,
            quantity: 1,
            unit_measure: "unit".to_string(),
            total_amount: 100.0,
            ..Default::default()
        }],
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path(
            "/instore/orders/qr/seller/collectors/123/pos/POS1/qrs",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "qr_data": "00020101021226940014br.gov.bcb.pix",
            "in_store_order_id": "order-abc"
        })))
        .mount(&mock_server)
        .await;

    let response = client.create_qr_order(123, "POS1", request).await.unwrap();
    assert_eq!(response.in_store_order_id, "order-abc");
}

#[tokio::test]
async fn test_create_store() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let request = mercadopago_sdk::models::instore::StoreRequest {
        name: "My Store".to_string(),
        external_id: "EXT-1".to_string(),
        location: Default::default(),
    };

    Mock::given(method("POST"))
        .and(path("/users/123/stores"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "store-1",
            "name": "My Store"
        })))
        .mount(&mock_server)
        .await;

    let response = client.create_store(123, request).await.unwrap();
    assert_eq!(response.id, "store-1");
    assert_eq!(response.name, "My Store");
}

#[tokio::test]
async fn test_create_pos() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let request = mercadopago_sdk::models::instore::PosRequest {
        name: "POS A".to_string(),
        fixed_amount: false,
        store_id: "store-1".to_string(),
        external_id: "EXT-POS-1".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/pos"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 42,
            "name": "POS A",
            "store_id": "store-1"
        })))
        .mount(&mock_server)
        .await;

    let response = client.create_pos(request).await.unwrap();
    assert_eq!(response.id, 42);
    assert_eq!(response.name, "POS A");
}

#[tokio::test]
async fn test_search_stores() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/users/123/stores/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paging": { "total": 1, "offset": 0, "limit": 30 },
            "results": [{
                "id": "store-1",
                "name": "My Store"
            }]
        })))
        .mount(&mock_server)
        .await;

    let response = client.search_stores(123).await.unwrap();
    assert_eq!(response.paging.total, 1);
    assert_eq!(response.results[0].name, "My Store");
}

#[tokio::test]
async fn test_list_pos() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/pos"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paging": { "total": 2, "offset": 0, "limit": 30 },
            "results": [
                { "id": 1, "name": "POS 1", "store_id": "s1" },
                { "id": 2, "name": "POS 2", "store_id": "s2" }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client.list_pos().await.unwrap();
    assert_eq!(response.paging.total, 2);
    assert_eq!(response.results.len(), 2);
}

#[tokio::test]
async fn test_create_refund() {
    let mock_server = MockServer::start().await;
    let client = MercadoPagoClient::builder("TEST_TOKEN")
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let request = mercadopago_sdk::models::refunds::RefundRequest { amount: Some(50.0) };

    Mock::given(method("POST"))
        .and(path("/v1/payments/123/refunds"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 999,
            "payment_id": 123,
            "amount": 50.0,
            "status": "approved"
        })))
        .mount(&mock_server)
        .await;

    let response = client.create_refund(123, request).await.unwrap();
    assert_eq!(response.id, 999);
    assert_eq!(response.payment_id, 123);
    assert_eq!(response.amount, 50.0);
    assert_eq!(response.status, "approved");
}
