use mercadopago_sdk::models::payments::{PaymentRequest, PaymentResponse};
use serde_json::json;

#[test]
fn test_payment_request_serialization_full() {
    let request = PaymentRequest {
        transaction_amount: 150.5,
        payment_method_id: "visa".to_string(),
        description: Some("Comprehensive Test".to_string()),
        payer: mercadopago_sdk::models::payments::Payer {
            email: "test@example.com".to_string(),
            identification: Some(mercadopago_sdk::models::payments::Identification {
                id_type: "DNI".to_string(),
                number: "12345678".to_string(),
            }),
        },
        installments: Some(3),
        external_reference: Some("REF123".to_string()),
        ..Default::default()
    };

    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["transaction_amount"], 150.5);
    assert_eq!(serialized["payer"]["email"], "test@example.com");
    assert_eq!(serialized["payer"]["identification"]["type"], "DNI");
    assert_eq!(serialized["installments"], 3);
    assert_eq!(serialized["external_reference"], "REF123");
}

#[test]
fn test_payment_response_deserialization() {
    let response_json = json!({
        "id": 123456789,
        "status": "approved",
        "status_detail": "accredited",
        "transaction_amount": 100.0,
        "date_created": "2023-01-01T12:00:00.000-04:00"
    });

    let response: PaymentResponse = serde_json::from_value(response_json).unwrap();
    assert_eq!(response.id, 123456789);
    assert_eq!(response.status, "approved");
}

#[test]
fn test_preference_request_serialization() {
    let request = mercadopago_sdk::models::preferences::PreferenceRequest {
        items: vec![mercadopago_sdk::models::preferences::PreferenceItem {
            title: "Test Item".to_string(),
            quantity: 1,
            unit_price: 100.0,
            ..Default::default()
        }],
        external_reference: Some("REF-456".to_string()),
        ..Default::default()
    };

    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["items"][0]["title"], "Test Item");
    assert_eq!(serialized["external_reference"], "REF-456");
}

#[test]
fn test_qr_order_request_serialization() {
    let request = mercadopago_sdk::models::instore::QrOrderRequest {
        total_amount: 150.0,
        items: vec![mercadopago_sdk::models::instore::QrOrderItem {
            title: "QR Item".to_string(),
            unit_price: 150.0,
            quantity: 1,
            unit_measure: "unit".to_string(),
            total_amount: 150.0,
            ..Default::default()
        }],
        ..Default::default()
    };

    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["total_amount"], 150.0);
    assert_eq!(serialized["items"][0]["title"], "QR Item");
}

#[test]
fn test_store_request_serialization() {
    let request = mercadopago_sdk::models::instore::StoreRequest {
        name: "Test Store".to_string(),
        external_id: "EXT-STORE-1".to_string(),
        location: Default::default(),
    };

    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["name"], "Test Store");
    assert_eq!(serialized["external_id"], "EXT-STORE-1");
}

#[test]
fn test_pos_request_serialization() {
    let request = mercadopago_sdk::models::instore::PosRequest {
        name: "Test POS".to_string(),
        fixed_amount: false,
        store_id: "12345".to_string(),
        external_id: "EXT-POS-1".to_string(),
    };

    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["name"], "Test POS");
    assert_eq!(serialized["store_id"], "12345");
}

#[test]
fn test_refund_request_serialization() {
    // Partial refund
    let request = mercadopago_sdk::models::refunds::RefundRequest { amount: Some(50.0) };
    let serialized = serde_json::to_value(&request).unwrap();
    assert_eq!(serialized["amount"], 50.0);

    // Full refund (no amount)
    let full_refund = mercadopago_sdk::models::refunds::RefundRequest { amount: None };
    let serialized = serde_json::to_string(&full_refund).unwrap();
    assert!(!serialized.contains("amount"));
}

#[test]
fn test_search_response_deserialization() {
    let json = serde_json::json!({
        "paging": { "total": 5, "offset": 0, "limit": 10 },
        "results": [
            { "id": 1, "status": "approved", "transaction_amount": 100.0 },
            { "id": 2, "status": "pending", "transaction_amount": 200.0 }
        ]
    });

    let response: mercadopago_sdk::models::instore::SearchResponse<
        mercadopago_sdk::models::payments::PaymentResponse,
    > = serde_json::from_value(json).unwrap();
    assert_eq!(response.paging.total, 5);
    assert_eq!(response.paging.limit, 10);
    assert_eq!(response.results.len(), 2);
    assert_eq!(response.results[0].id, 1);
    assert_eq!(response.results[1].status, "pending");
}

#[test]
fn test_payment_response_full() {
    let json = json!({
        "id": 987654321,
        "status": "approved",
        "status_detail": "accredited",
        "transaction_amount": 250.50,
        "date_created": "2026-02-16T12:00:00.000-03:00",
        "external_reference": "ORDER-999",
        "payment_method_id": "visa",
        "payment_type_id": "credit_card",
        "currency_id": "ARS",
        "description": "Test payment",
        "installments": 6,
        "net_received_amount": 230.25,
        "captured": true,
        "date_approved": "2026-02-16T12:00:05.000-03:00",
        "date_last_updated": "2026-02-16T12:00:05.000-03:00",
        "live_mode": false,
        "payer": {
            "id": 123456,
            "email": "buyer@example.com",
            "identification": { "type": "DNI", "number": "33445566" },
            "type": "registered"
        },
        "fee_details": [
            { "type": "mercadopago_fee", "amount": 20.25, "fee_payer": "collector" }
        ]
    });

    let response: PaymentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(response.id, 987654321);
    assert_eq!(response.payment_method_id.as_deref(), Some("visa"));
    assert_eq!(response.payment_type_id.as_deref(), Some("credit_card"));
    assert_eq!(response.currency_id.as_deref(), Some("ARS"));
    assert_eq!(response.description.as_deref(), Some("Test payment"));
    assert_eq!(response.installments, Some(6));
    assert_eq!(response.net_received_amount, Some(230.25));
    assert_eq!(response.captured, Some(true));
    assert_eq!(response.live_mode, Some(false));

    let payer = response.payer.unwrap();
    assert_eq!(payer.id, Some(123456));
    assert_eq!(payer.email.as_deref(), Some("buyer@example.com"));
    assert_eq!(payer.payer_type.as_deref(), Some("registered"));
    assert_eq!(payer.identification.unwrap().number, "33445566");

    let fees = response.fee_details.unwrap();
    assert_eq!(fees.len(), 1);
    assert_eq!(fees[0].fee_type.as_deref(), Some("mercadopago_fee"));
    assert_eq!(fees[0].amount, Some(20.25));
    assert_eq!(fees[0].fee_payer.as_deref(), Some("collector"));
}

#[test]
fn test_payment_response_backward_compat() {
    // Minimal JSON with only the original required fields
    let json = json!({
        "id": 1,
        "status": "pending",
        "transaction_amount": 100.0
    });

    let response: PaymentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(response.id, 1);
    assert_eq!(response.status, "pending");
    assert_eq!(response.transaction_amount, 100.0);

    // All new fields should be None
    assert!(response.payment_method_id.is_none());
    assert!(response.payment_type_id.is_none());
    assert!(response.currency_id.is_none());
    assert!(response.description.is_none());
    assert!(response.installments.is_none());
    assert!(response.net_received_amount.is_none());
    assert!(response.captured.is_none());
    assert!(response.date_approved.is_none());
    assert!(response.date_last_updated.is_none());
    assert!(response.live_mode.is_none());
    assert!(response.payer.is_none());
    assert!(response.fee_details.is_none());
}

#[test]
fn test_skip_serializing_none_fields() {
    let request = PaymentRequest {
        transaction_amount: 100.0,
        payment_method_id: "visa".to_string(),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(!serialized.contains("description"));
    assert!(!serialized.contains("token"));
    assert!(!serialized.contains("installments"));
    assert!(!serialized.contains("external_reference"));
    assert!(!serialized.contains("notification_url"));
}
