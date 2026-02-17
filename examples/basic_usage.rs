use mercadopago_sdk::MercadoPagoClient;
use mercadopago_sdk::models::payments::{Payer, PaymentRequest};
use mercadopago_sdk::models::preferences::{PreferenceItem, PreferenceRequest};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Get access token from environment
    let access_token = env::var("MERCADO_PAGO_ACCESS_TOKEN")
        .expect("MERCADO_PAGO_ACCESS_TOKEN must be set in .env or environment");

    // Initialize client
    let client = MercadoPagoClient::builder(&access_token).build()?;

    println!("--- Creating a Payment Preference (Checkout Pro) ---");
    let pref_request = PreferenceRequest {
        items: vec![PreferenceItem {
            title: "Test Product".to_string(),
            quantity: 1,
            unit_price: 150.0,
            currency_id: Some("ARS".to_string()),
            ..Default::default()
        }],
        external_reference: Some("MY-EXTERNAL-REF-123".to_string()),
        ..Default::default()
    };

    match client.create_preference(pref_request).await {
        Ok(pref) => {
            println!("Preference created successfully!");
            println!("ID: {}", pref.id);
            println!("Init Point: {}", pref.init_point);
            println!("Sandbox Init Point: {}", pref.sandbox_init_point);
        }
        Err(e) => eprintln!("Error creating preference: {}", e),
    }

    println!("\n--- Creating a Simple Payment (Pix/Generic) ---");
    // Note: For real credit card payments, you need a card token from the frontend.
    // This example uses a generic request structure.
    let payment_request = PaymentRequest {
        transaction_amount: 100.0,
        payment_method_id: "pix".to_string(), // Using pix as an example of a simple method
        description: Some("Example Payment".to_string()),
        payer: Payer {
            email: "test_user_123@testuser.com".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    // We can add an idempotency key to prevent double charges
    let idempotency_key = "unique-key-for-this-transaction";

    match client
        .post("/v1/payments")
        .header("X-Idempotency-Key", idempotency_key)
        .json(&payment_request)
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                let payment: mercadopago_sdk::models::payments::PaymentResponse =
                    res.json().await?;
                println!("Payment created successfully!");
                println!("ID: {}", payment.id);
                println!("Status: {}", payment.status);
            } else {
                println!("Payment failed with status: {}", res.status());
            }
        }
        Err(e) => eprintln!("Error creating payment: {}", e),
    }

    Ok(())
}
