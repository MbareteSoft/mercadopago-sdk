//! Models for the Payments API.
use serde::{Deserialize, Serialize};

/// Represents a request to create a payment.
///
/// See the [official documentation](https://www.mercadopago.com.ar/developers/es/reference/payments/_payments/post) for more details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PaymentRequest {
    /// Amount of the payment.
    pub transaction_amount: f64,
    /// Identifier of the payment method.
    pub payment_method_id: String,
    /// Description of the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Information about the payer.
    pub payer: Payer,
    /// Token for card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Number of installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<u32>,
    /// External reference for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    /// Email of the payer (if not in payer struct).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
}

/// Information about the payer of the payment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Payer {
    /// Email of the payer.
    pub email: String,
    /// Identification of the payer (DNI, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,
}

/// Identification information for a payer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Identification {
    /// Type of identification (e.g., DNI).
    #[serde(rename = "type")]
    pub id_type: String,
    /// Number of identification.
    pub number: String,
}

/// Represents the response from the Mercado Pago API after a payment operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentResponse {
    /// Unique identifier of the payment.
    pub id: u64,
    /// Status of the payment (e.g., approved, pending).
    pub status: String,
    /// Detailed status of the payment (e.g., accredited).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// Amount of the payment.
    pub transaction_amount: f64,
    /// Date the payment was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// External reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    /// Payment method identifier (e.g., "visa", "master", "pix").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Payment type (e.g., "credit_card", "debit_card", "account_money").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type_id: Option<String>,
    /// Currency of the payment (e.g., "ARS", "BRL").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Description of the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Number of installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<u32>,
    /// Net amount received after fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_received_amount: Option<f64>,
    /// Whether the payment has been captured (for two-step payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured: Option<bool>,
    /// Date the payment was approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_approved: Option<String>,
    /// Date the payment was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_last_updated: Option<String>,
    /// Whether the payment was made in live mode (vs sandbox).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_mode: Option<bool>,
    /// Information about the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<PayerResponse>,
    /// Fees charged on this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_details: Option<Vec<FeeDetail>>,
}

/// Payer information as returned in a payment response.
///
/// Separate from [`Payer`] (used in requests) because the response includes
/// additional fields like `id` and makes all fields optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PayerResponse {
    /// Unique identifier of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Email of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Identification of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,
    /// Type of payer (e.g., "guest", "registered").
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_type: Option<String>,
}

/// Fee charged on a payment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FeeDetail {
    /// Type of fee (e.g., "mercadopago_fee", "coupon_fee").
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
    /// Amount of the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Who pays the fee: "collector" or "payer".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
}
