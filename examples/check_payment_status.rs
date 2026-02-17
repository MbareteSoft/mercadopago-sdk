use mercadopago_sdk::MercadoPagoClient;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Get access token from environment
    let access_token = env::var("MERCADO_PAGO_ACCESS_TOKEN")
        .expect("MERCADO_PAGO_ACCESS_TOKEN must be set in .env or environment");

    // Initialize client
    let client = MercadoPagoClient::builder(&access_token).build()?;

    // This is the external_reference you used when creating the QR
    // Tip: Use the one printed by 'full_qr_flow' or 'qr_payment'
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example check_payment_status <external_reference>");
        println!("Example: cargo run --example check_payment_status ORDER-1234");
        return Ok(());
    }
    let external_reference = &args[1];

    println!("--- Polling for Payment Status ---");
    println!("External Reference: {}", external_reference);
    println!(
        "Press Ctrl+C to stop.
"
    );

    loop {
        match client.search_payments(external_reference).await {
            Ok(search_res) => {
                if let Some(payment) = search_res.results.first() {
                    println!("Payment Found!");
                    println!("   ID: {}", payment.id);
                    println!("   Status: {}", payment.status);
                    println!("   Status Detail: {:?}", payment.status_detail);

                    if payment.status == "approved" {
                        println!(
                            "
✅ SUCCESS: The payment has been accredited!"
                        );
                        break;
                    } else if payment.status == "rejected" {
                        println!(
                            "
❌ REJECTED: The payment was not accepted."
                        );
                        break;
                    } else {
                        println!("   -> Status is '{}'. Still waiting...", payment.status);
                    }
                } else {
                    println!(
                        "No payment found yet for reference '{}'.",
                        external_reference
                    );
                }
            }
            Err(e) => eprintln!("Error searching payment: {}", e),
        }

        println!("Retrying in 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
