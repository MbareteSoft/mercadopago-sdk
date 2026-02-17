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

    // User ID (Collector ID) from environment
    let user_id: u64 = env::var("MERCADO_PAGO_USER_ID")
        .expect("MERCADO_PAGO_USER_ID must be set in .env or environment")
        .parse()
        .expect("MERCADO_PAGO_USER_ID must be a valid u64");

    println!("--- Account Discovery ---");

    // 1. Discover Stores
    println!("\n1. Listing Stores for user {}...", user_id);
    let stores_search = client.search_stores(user_id).await?;
    println!("Found {} stores:", stores_search.paging.total);

    for store in &stores_search.results {
        println!(
            "   - [{}] {} (External ID: {})",
            store.id,
            store.name,
            store.external_id.as_deref().unwrap_or("N/A")
        );
    }

    // 2. Discover all POS
    println!("\n2. Listing all Points of Sale (POS)...");
    let pos_search = client.list_pos().await?;
    println!("Found {} POS:", pos_search.paging.total);

    for pos in &pos_search.results {
        // Try to match POS with its store for clarity
        let store_name = stores_search
            .results
            .iter()
            .find(|s| s.id == pos.store_id)
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown Store");

        println!(
            "   - [{}] Name: '{}' | External ID: '{}' | Store: '{}'",
            pos.id,
            pos.name,
            pos.external_id.as_deref().unwrap_or("N/A"),
            store_name
        );
    }

    if pos_search.results.is_empty() {
        println!(
            "\nTIP: If you don't see any POS, run 'cargo run --example full_qr_flow' to create one."
        );
    }

    Ok(())
}
