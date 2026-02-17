use mercadopago_sdk::MercadoPagoClient;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let access_token = env::var("MERCADO_PAGO_ACCESS_TOKEN")
        .expect("MERCADO_PAGO_ACCESS_TOKEN must be set in .env or environment");

    let client = MercadoPagoClient::builder(&access_token).build()?;

    // Configure these with your actual IDs
    let store_id = env::var("STORE_ID").unwrap_or_default();
    let pos_id = env::var("POS_ID").unwrap_or_default();

    let mut filters: Vec<(&str, &str)> = vec![
        ("sort", "date_created"),
        ("criteria", "desc"),
        ("limit", "20"),
    ];

    if !store_id.is_empty() {
        filters.push(("store_id", &store_id));
        println!("--- Pagos de Sucursal: {} ---", store_id);
    }

    if !pos_id.is_empty() {
        filters.push(("pos_id", &pos_id));
        println!("--- Pagos de POS: {} ---", pos_id);
    }

    if store_id.is_empty() && pos_id.is_empty() {
        println!("--- Todos los pagos recientes ---");
        println!("Tip: usa STORE_ID y/or POS_ID en .env para filtrar por sucursal/punto de venta");
    }

    match client.search_payments_generic(&filters).await {
        Ok(search_res) => {
            println!(
                "\nTotal: {} pagos encontrados. Mostrando {}:\n",
                search_res.paging.total,
                search_res.results.len()
            );

            println!(
                "{:<15} | {:<10} | {:>10} | {:<25} | {:<20}",
                "ID", "Estado", "Monto", "Fecha", "Referencia"
            );
            println!("{}", "-".repeat(90));

            for payment in &search_res.results {
                let ref_str = payment.external_reference.as_deref().unwrap_or("-");
                let date_str = payment
                    .date_created
                    .as_deref()
                    .map(|d| &d[..19])
                    .unwrap_or("-");

                println!(
                    "{:<15} | {:<10} | {:>10.2} | {:<25} | {:<20}",
                    payment.id, payment.status, payment.transaction_amount, date_str, ref_str
                );
            }

            if search_res.results.is_empty() {
                println!("  (sin resultados)");
            }
        }
        Err(e) => eprintln!("Error consultando pagos: {}", e),
    }

    Ok(())
}
