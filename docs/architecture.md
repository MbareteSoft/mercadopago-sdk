# Arquitectura y Diseño

El SDK de Mercado Pago para Rust está diseñado con foco en tipado seguro, rendimiento asíncrono y ergonomía para el desarrollador.

## Componentes Principales

### `MercadoPagoClient`
El punto de entrada para todas las interacciones con la API. Está diseñado para ser:
- **Thread-Safe:** Implementa `Clone`, `Send` y `Sync`, permitiendo compartirlo de forma segura entre tareas o almacenarlo en estados de frameworks web (ej: Axum, Actix).
- **Personalizable:** Usa el patrón Builder para configurar timeouts, URLs base y autenticación.

### `MercadoPagoRequestBuilder`
Un wrapper sobre `reqwest::RequestBuilder` que provee:
- **Soporte de Idempotencia:** Adjunción sencilla de headers `X-Idempotency-Key`.
- **Query/JSON Genérico:** Serialización con tipado seguro de cuerpos de request y parámetros de consulta.
- **Intercepción de Errores:** Intercepta automáticamente códigos de estado HTTP no exitosos e intenta parsear respuestas `ApiError` estructuradas.

## Patrones de Resiliencia

### Reintentos Automáticos (Manejo de 429)
El SDK incluye lógica integrada para manejar rate limiting. Cuando la API retorna un estado `429 Too Many Requests`:
1. El cliente busca el header `Retry-After`.
2. Si está presente, espera la duración especificada.
3. Si no está presente, aplica un delay por defecto de 1 segundo.
4. El request se reintenta automáticamente.

### Timeouts
Cada instancia del cliente tiene timeouts configurables de conexión y request (por defecto 10s y 30s respectivamente), asegurando que tu aplicación no se quede colgada indefinidamente por problemas de red.

## Stack Tecnológico

- **Cliente HTTP:** `reqwest` (con `rustls-tls` para seguridad).
- **Runtime Asíncrono:** `tokio`.
- **Serialización:** `serde` y `serde_json`.
- **Manejo de Errores:** `thiserror` para errores estructurados de la librería y `anyhow` para flexibilidad a nivel aplicación.
- **Logging:** `tracing` (opcional, vía el feature `logging`).

## Feature Flags

| Feature | Descripción | Por defecto |
|---------|-------------|-------------|
| `logging` | Habilita instrumentación con `tracing` para requests y reintentos. | Deshabilitado |
