//! # Mercado Pago Rust SDK
//!
//! A type-safe and idiomatic Rust SDK for integrating with Mercado Pago APIs.
//!
//! ## Example
//!
//! ```no_run
//! use mercadopago_sdk::MercadoPagoClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = MercadoPagoClient::builder("YOUR_ACCESS_TOKEN").build()?;
//!     // ... use the client
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::MercadoPagoClient;
pub use error::Error;
