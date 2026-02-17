use mercadopago_sdk::MercadoPagoClient;
use mercadopago_sdk::models::instore::{
    PosRequest, QrOrderItem, QrOrderRequest, StoreLocation, StoreRequest,
};
use qrcode::QrCode;
use rand::Rng;
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

    // User ID (Collector ID) from environment
    let user_id: u64 = env::var("MERCADO_PAGO_USER_ID")
        .expect("MERCADO_PAGO_USER_ID must be set in .env or environment")
        .parse()
        .expect("MERCADO_PAGO_USER_ID must be a valid u64");

    // Generate random suffix to avoid collisions
    let mut rng = rand::thread_rng();
    let suffix: u32 = rng.gen_range(1000..9999);

    let store_external_id = format!("STORERS{}", suffix);
    let pos_external_id = format!("POSRS{}", suffix);
    let external_reference = format!("ORDER-{}", suffix);

    println!("--- Starting Full QR Payment Flow ---");
    println!(
        "Generated External IDs: Store='{}', POS='{}', Order='{}'",
        store_external_id, pos_external_id, external_reference
    );

    // 1. Create a Store
    println!("\n1. Creating Store...");

    let store_request = StoreRequest {
        name: format!("Rust SDK Test Store {}", suffix),
        external_id: store_external_id.clone(),
        location: StoreLocation {
            street_number: "123".to_string(),
            street_name: "Calle Falsa".to_string(),
            city_name: "Belgrano".to_string(),
            state_name: "Capital Federal".to_string(),
            latitude: -34.562087,
            longitude: -58.456695,
            reference: Some("Esquina".to_string()),
        },
    };

    let store = client.create_store(user_id, store_request).await?;
    println!("   -> Store created! ID: {}", store.id);

    // 2. Create a POS linked to that Store
    println!("\n2. Creating POS...");

    let pos_request = PosRequest {
        name: format!("Caja {}", suffix),
        fixed_amount: false,
        store_id: store.id.clone(),
        external_id: pos_external_id.clone(),
    };

    let pos = client.create_pos(pos_request).await?;
    println!("   -> POS created! ID: {}", pos.id);

    // 3. Create QR Order
    println!("\n3. Requesting Dynamic QR Order...");
    let qr_request = QrOrderRequest {
        external_reference: Some(external_reference.clone()),
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

    let response = client
        .create_qr_order(user_id, &pos_external_id, qr_request)
        .await?;
    println!(
        "   -> QR Order created! In-store Order ID: {}",
        response.in_store_order_id
    );

    // 4. Render QR Code
    println!("\n--- QR CODE ---");
    let code = QrCode::new(&response.qr_data)?;
    let image = code.render().light_color(' ').dark_color('█').build();

    println!("{}", image);
    println!("----------------\n");
    println!("ESCANEA EL CÓDIGO CON TU APP DE MERCADO PAGO PARA PAGAR $150.0");
    println!("Esperando confirmación de pago (Polling)... Presiona Ctrl+C para cancelar.\n");

    // 5. Wait for payment (Polling loop)
    loop {
        match client.search_payments(&external_reference).await {
            Ok(search_res) => {
                if let Some(payment) = search_res.results.first() {
                    println!("\n¡PAGO DETECTADO!");
                    println!("   ID de Pago: {}", payment.id);
                    println!("   Estado: {}", payment.status);

                    if payment.status == "approved" {
                        println!(
                            "\n✅ ¡VENTA CONFIRMADA! El pago ha sido acreditado exitosamente."
                        );
                        println!("Ya puedes entregar el producto al cliente.");
                        break;
                    } else if payment.status == "rejected" {
                        println!("\n❌ PAGO RECHAZADO. Por favor, solicita otro medio de pago.");
                        break;
                    } else {
                        println!(
                            "   -> El estado es '{}'. Todavía procesando...",
                            payment.status
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error consultando estado: {}", e),
        }

        sleep(Duration::from_secs(3)).await;
    }

    Ok(())
}
