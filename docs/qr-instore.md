# QR y Pagos Instore

Los pagos con QR dinámico son ideales para tiendas físicas. Requieren configurar un entorno (Sucursales y Cajas) antes de generar códigos QR.

## Jerarquía

1.  **Sucursal (Store):** Una ubicación física.
2.  **Punto de Venta (POS/Caja):** Un terminal o caja específica dentro de una sucursal.
3.  **Orden QR:** Una transacción específica asignada a un Punto de Venta.

## 1. Configuración del Entorno

### Crear una Sucursal
```rust
use mercadopago_sdk::models::instore::{StoreRequest, StoreLocation};

let store_req = StoreRequest {
    name: "Sucursal Principal".into(),
    external_id: "SUCURSAL_001".into(),
    location: StoreLocation {
        street_number: "123".into(),
        street_name: "Av. Corrientes".into(),
        city_name: "Buenos Aires".into(),
        state_name: "CABA".into(),
        latitude: -34.0,
        longitude: -58.0,
        ..Default::default()
    },
};

let store = client.create_store(user_id, store_req).await?;
```

### Crear un Punto de Venta
```rust
use mercadopago_sdk::models::instore::PosRequest;

let pos_req = PosRequest {
    name: "Caja 1".into(),
    fixed_amount: false,
    store_id: store.id, // ID de la sucursal creada arriba
    external_id: "CAJA_001".into(),
};

let pos = client.create_pos(pos_req).await?;
```

## 2. Generación de QR Dinámico

Para generar un código QR para un cliente, creás una "orden" y la asignás a un Punto de Venta.

```rust
use mercadopago_sdk::models::instore::{QrOrderRequest, QrOrderItem};

let order = QrOrderRequest {
    external_reference: Some("VENTA_999".into()),
    total_amount: 150.0,
    items: vec![QrOrderItem {
        title: "Producto A".into(),
        unit_price: 150.0,
        quantity: 1,
        unit_measure: "unit".into(),
        total_amount: 150.0,
        ..Default::default()
    }],
    ..Default::default()
};

// Usá el EXTERNAL_ID del POS para vincular la orden
let res = client.create_qr_order(user_id, "CAJA_001", order).await?;

// res.qr_data contiene el payload para renderizar como código QR
```

## 3. Descubrimiento

Podés listar todas las sucursales y cajas existentes para mapear correctamente tu sistema:

```rust
let stores = client.search_stores(user_id).await?;
let boxes = client.list_pos().await?;
```
