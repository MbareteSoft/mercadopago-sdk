# Mercado Pago Rust SDK

[![Licencia: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)

Un SDK de Rust de alto rendimiento, con tipado seguro e idiomático, para integrar pagos de Mercado Pago en tus servicios backend.

## Características

- **Base sólida:** Construido sobre `reqwest` y `tokio` para E/S asíncrona robusta.
- **API de Pagos:** Creá, consultá y buscá pagos con tipado seguro completo.
- **Checkout Pro:** Generá preferencias de pago para flujos de checkout web.
- **QR/Instore:** Gestión completa de Sucursales, Puntos de Venta (Cajas) y generación de QR dinámico.
- **Resiliencia:** Soporte integrado para `429 Too Many Requests` con manejo automático de `Retry-After`.
- **Observabilidad:** Instrumentación de trazas opcional mediante el feature flag `logging`.

## Instalación

Agregá esto a tu `Cargo.toml`:

```toml
[dependencies]
mercadopago-sdk = { path = "./" } # O desde el registro una vez publicado
tokio = { version = "1.0", features = ["full"] }
```

## Inicio Rápido

```rust
use mercadopago_sdk::MercadoPagoClient;
use mercadopago_sdk::models::payments::PaymentRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let access_token = "TU_ACCESS_TOKEN";
    let client = MercadoPagoClient::builder(access_token).build()?;

    // Crear un pago simple
    let request = PaymentRequest {
        transaction_amount: 100.0,
        payment_method_id: "pix".to_string(),
        ..Default::default()
    };

    let payment = client.create_payment(request).await?;
    println!("Pago creado con ID: {}", payment.id);

    Ok(())
}
```

## Documentación

La documentación técnica completa está disponible en el directorio [`docs/`](./docs):

- [Arquitectura y Diseño](./docs/architecture.md)
- [Autenticación y Configuración](./docs/authentication.md)
- [Pagos y Checkout Pro](./docs/payments-and-checkout.md)
- [QR e Instore](./docs/qr-instore.md)
- [Manejo de Errores](./docs/error-handling.md)

## Ejemplos

Consultá la carpeta `examples/` para patrones listos para producción:
- `basic_usage.rs`: Pagos y preferencias básicas.
- `full_qr_flow.rs`: Flujo QR de punta a punta incluyendo configuración de Sucursal/Caja y polling.
- `discovery.rs`: Explorá las sucursales y cajas de tu cuenta.
- `list_payments.rs`: Buscá y auditá el historial de transacciones.
- `list_store_payments.rs`: Filtrá pagos por sucursal y/o caja.
- `refund_payment.rs`: Realizá reembolsos totales o parciales.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT.
