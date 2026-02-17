//! Models for the Instore/QR API.
use serde::{Deserialize, Serialize};

/// Represents a request to create a dynamic QR order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QrOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    pub total_amount: f64,
    pub items: Vec<QrOrderItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_out: Option<CashOut>,
}

/// Represents an item in a QR order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QrOrderItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: f64,
    pub quantity: i32,
    pub unit_measure: String,
    pub total_amount: f64,
}

/// Represents a cash-out operation within a QR order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CashOut {
    /// Amount to be cashed out.
    pub amount: f64,
}

/// Represents the response from creating a QR order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QrOrderResponse {
    pub qr_data: String,
    pub in_store_order_id: String,
}

// Re-exported from common module for backward compatibility.
pub use super::common::{Paging, SearchResponse};

/// Request to create a Store.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StoreRequest {
    pub name: String,
    pub external_id: String,
    pub location: StoreLocation,
}

/// Physical location of a store.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StoreLocation {
    /// Street number of the store.
    pub street_number: String,
    /// Street name of the store.
    pub street_name: String,
    /// City where the store is located.
    pub city_name: String,
    /// State or province where the store is located.
    pub state_name: String,
    /// Latitude coordinate.
    pub latitude: f64,
    /// Longitude coordinate.
    pub longitude: f64,
    /// Additional reference for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from the Store API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreResponse {
    /// Unique identifier of the store.
    pub id: String,
    /// Name of the store.
    pub name: String,
    /// External identifier of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Date when the store was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_creation: Option<String>,
    /// Location details of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}

/// Request to create a Point of Sale (POS).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PosRequest {
    pub name: String,
    pub fixed_amount: bool,
    pub store_id: String,
    pub external_id: String,
}

/// Response from the POS (Point of Sale) API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosResponse {
    /// Unique identifier of the POS.
    pub id: u64,
    /// Name of the POS.
    pub name: String,
    /// External identifier of the POS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Identifier of the store this POS belongs to.
    pub store_id: String,
    /// Date when the POS was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date when the POS was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_last_updated: Option<String>,
}
