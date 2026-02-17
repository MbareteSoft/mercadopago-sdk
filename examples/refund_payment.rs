use mercadopago_sdk::MercadoPagoClient;
use mercadopago_sdk::models::refunds::RefundRequest;
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

    // Get payment ID from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example refund_payment <payment_id> [amount]");
        println!("Example: cargo run --example refund_payment 123456789");
        return Ok(());
    }

    let payment_id: u64 = args[1].parse().expect("Invalid payment ID format");

    // Optional partial refund amount
    let amount = args.get(2).and_then(|a| a.parse::<f64>().ok());

    println!("--- Issuing Refund ---");
    println!("Payment ID: {}", payment_id);
    if let Some(a) = amount {
        println!("Amount: ${:.2}", a);
    } else {
        println!("Type: Full Refund");
    }

    let refund_request = RefundRequest { amount };

    match client.create_refund(payment_id, refund_request).await {
        Ok(res) => {
            println!(
                "
✅ SUCCESS: Refund issued successfully!"
            );
            println!("Refund ID: {}", res.id);
            println!("Status: {}", res.status);
            println!("Amount Refunded: ${:.2}", res.amount);
        }
        Err(e) => eprintln!("Error issuing refund: {}", e),
    }

    Ok(())
}
