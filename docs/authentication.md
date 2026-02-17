# Autenticación y Configuración

El SDK requiere un Access Token válido de Mercado Pago para interactuar con las APIs.

## Obtener Credenciales

1. Ingresá al [Panel de Desarrolladores de Mercado Pago](https://www.mercadopago.com.ar/developers/panel).
2. Creá una Aplicación.
3. Encontrá tus credenciales de **Producción** o **Test**.
4. Almacená de forma segura tu `Access Token`.

## Inicialización del Cliente

La forma recomendada de inicializar el cliente es usando variables de entorno.

### Usando `.env`

Creá un archivo `.env` en la raíz de tu proyecto:
```env
MERCADO_PAGO_ACCESS_TOKEN=APP_USR-xxxx-xxxx-xxxx
```

### En Rust

```rust
use mercadopago_sdk::MercadoPagoClient;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let token = env::var("MERCADO_PAGO_ACCESS_TOKEN")?;
    let client = MercadoPagoClient::builder(&token)
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    Ok(())
}
```

## Buenas Prácticas de Seguridad

- **Nunca hardcodees tokens:** Usá siempre variables de entorno o gestores de secretos (AWS Secrets Manager, HashiCorp Vault).
- **Usá tokens separados:** Usá tokens de Test para desarrollo local/CI y tokens de Producción solo en ambientes productivos.
- **Idempotencia:** Al realizar operaciones de escritura (como `create_payment`), proporcioná siempre una clave de idempotencia para evitar cobros duplicados al usuario en caso de reintentos de red.

```rust
let response = client.post("/v1/payments")
    .header("X-Idempotency-Key", "uuid-unico-por-transaccion")
    .json(&request)
    .send()
    .await?;
```
