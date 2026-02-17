use mercadopago_sdk::MercadoPagoClient;
use mercadopago_sdk::models::instore::{QrOrderItem, QrOrderRequest};
use qrcode::QrCode;
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
    // External ID of the POS (Point of Sale). You should have created this in your Mercado Pago account.
    // For this example, we'll use a placeholder.
    let pos_id = "SUCURSAL001";

    println!("--- Requesting a Dynamic QR Order ---");
    let qr_request = QrOrderRequest {
        external_reference: Some("ORDER-QR-123".to_string()),
        title: Some("Venta en local".to_string()),
        description: Some("Cobro de producto via QR".to_string()),
        total_amount: 150.0,
        items: vec![QrOrderItem {
            title: "Producto Test".to_string(),
            unit_price: 150.0,
            quantity: 1,
            unit_measure: "unit".to_string(),
            total_amount: 150.0,
            ..Default::default()
        }],
        ..Default::default()
    };

    println!("Creating order for User: {}, POS: {}", user_id, pos_id);

    match client.create_qr_order(user_id, pos_id, qr_request).await {
        Ok(response) => {
            println!("QR Order created successfully!");
            println!("In-store Order ID: {}", response.in_store_order_id);
            println!(
                "
--- QR CODE ---"
            );

            // Generate the QR code from the qr_data
            let code = QrCode::new(&response.qr_data)?;
            let image = code.render().light_color(' ').dark_color('█').build();

            println!("{}", image);
            println!(
                "----------------
"
            );
            println!("Scan the QR code above to pay.");
        }
        Err(e) => {
            eprintln!("Error creating QR order: {}", e);
            eprintln!(
                "
Note: Make sure the POS_ID '{}' exists in your Mercado Pago account for the user {}.",
                pos_id, user_id
            );
        }
    }

    Ok(())
}
