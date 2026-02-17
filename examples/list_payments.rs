use mercadopago_sdk::MercadoPagoClient;
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

    println!("--- List Recent Payments ---");

    // Query parameters for the search
    // We sort by date_created in descending order to see the newest first
    let filters = [
        ("sort", "date_created"),
        ("criteria", "desc"),
        ("limit", "10"),
    ];

    match client.search_payments_generic(&filters).await {
        Ok(search_res) => {
            println!(
                "Found {} total payments. Showing last {}:",
                search_res.paging.total,
                search_res.results.len()
            );

            println!(
                "{:<15} | {:<10} | {:<10} | {:<12} | {:<6} | {:<25} | {:<20}",
                "ID", "Status", "Amount", "Method", "Inst.", "Date", "Reference"
            );
            println!("{}", "-".repeat(110));

            for payment in search_res.results {
                let ref_str = payment.external_reference.as_deref().unwrap_or("N/A");
                let date_str = payment.date_created.as_deref().unwrap_or("N/A");
                let method = payment.payment_method_id.as_deref().unwrap_or("-");
                let installments = payment
                    .installments
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "-".to_string());

                println!(
                    "{:<15} | {:<10} | {:<10.2} | {:<12} | {:<6} | {:<25} | {:<20}",
                    payment.id,
                    payment.status,
                    payment.transaction_amount,
                    method,
                    installments,
                    date_str,
                    ref_str
                );
            }
        }
        Err(e) => eprintln!("Error fetching payments: {}", e),
    }

    Ok(())
}
