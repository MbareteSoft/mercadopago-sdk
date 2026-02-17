//! Models for the Refunds API.
use serde::{Deserialize, Serialize};

/// Request to create a refund.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RefundRequest {
    /// Amount to be refunded. If not provided, a full refund will be issued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

/// Response from creating a refund.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundResponse {
    /// Unique identifier of the refund.
    pub id: u64,
    /// ID of the payment being refunded.
    pub payment_id: u64,
    /// Amount refunded.
    pub amount: f64,
    /// Status of the refund (e.g., approved).
    pub status: String,
    /// Date the refund was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
}
