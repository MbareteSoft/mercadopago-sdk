# Pagos y Checkout Pro

El SDK provee métodos de alto nivel para operaciones comunes de pago.

## API de Pagos

La API de Pagos te permite crear y gestionar transacciones.

### Crear un Pago

```rust
use mercadopago_sdk::models::payments::{PaymentRequest, Payer};

let request = PaymentRequest {
    transaction_amount: 500.0,
    payment_method_id: "visa".to_string(),
    description: Some("Orden #123".to_string()),
    payer: Payer {
        email: "comprador@ejemplo.com".to_string(),
        ..Default::default()
    },
    ..Default::default()
};

let payment = client.create_payment(request).await?;
println!("Estado: {}", payment.status);
```

### Consultar y Buscar

```rust
// Obtener por ID
let payment = client.get_payment(123456789).await?;

// Buscar por Referencia Externa
let search = client.search_payments("ORDER-123").await?;
let payment = search.results.first();
```

## Checkout Pro (Preferencias)

Checkout Pro es la forma más fácil de aceptar pagos en un sitio web. Creás una "Preferencia" y redirigís al usuario a la URL generada.

```rust
use mercadopago_sdk::models::preferences::{PreferenceRequest, PreferenceItem};

let request = PreferenceRequest {
    items: vec![PreferenceItem {
        title: "Remera".to_string(),
        quantity: 2,
        unit_price: 250.0,
        ..Default::default()
    }],
    external_reference: Some("REF-999".to_string()),
    ..Default::default()
};

let pref = client.create_preference(request).await?;
println!("Redirigir al usuario a: {}", pref.init_point);
```

### URLs de Retorno
Podés configurar a dónde se redirige al usuario después del pago:

```rust
use mercadopago_sdk::models::preferences::BackUrls;

let request = PreferenceRequest {
    back_urls: Some(BackUrls {
        success: Some("https://miapp.com/exito".into()),
        failure: Some("https://miapp.com/error".into()),
        ..Default::default()
    }),
    ..Default::default()
};
```
