# Manejo de Errores

El SDK provee un sistema robusto de manejo de errores para distinguir entre problemas de red, errores de serialización y rechazos reales de la API.

## El Enum `Error`

Todos los métodos retornan un `Result<T, mercadopago_sdk::Error>`.

```rust
pub enum Error {
    /// Errores devueltos por Mercado Pago (ej: 400 Bad Request, 401 Unauthorized)
    ApiError {
        message: String,
        error: String,
        status: u16,
        cause: Option<Vec<Cause>>,
    },
    /// Errores que ocurren durante el request/response de red
    Network(String),
    /// Errores durante el mapeo JSON
    Serialization(String),
    /// Errores de lógica interna del SDK
    Internal(String),
}
```

## Manejo de Errores de la API

Cuando la API de Mercado Pago retorna un estado no-2xx, el SDK parsea automáticamente el cuerpo del error.

```rust
match client.create_payment(request).await {
    Ok(payment) => println!("¡Éxito!"),
    Err(mercadopago_sdk::Error::ApiError { status, message, .. }) => {
        eprintln!("Mercado Pago rechazó el request ({}): {}", status, message);
    }
    Err(e) => {
        eprintln!("Error del sistema: {}", e);
    }
}
```

## Causas de Error

Algunos errores de la API incluyen un campo `cause` con detalles específicos (ej: qué campo falló la validación).

```rust
if let Error::ApiError { cause: Some(causes), .. } = err {
    for cause in causes {
        println!("Error {}: {}",
            cause.code.unwrap_or_default(),
            cause.description.unwrap_or_default()
        );
    }
}
```

## Logging y Depuración

Habilitá el feature `logging` en tu `Cargo.toml` para ver logs detallados de requests y reintentos:

```bash
# Ejecutar con salida de logs
RUST_LOG=debug cargo run
```
