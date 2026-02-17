//! Models for the Preferences API (Checkout Pro).
use serde::{Deserialize, Serialize};

/// Represents a request to create a payment preference (Checkout Pro).
///
/// See the [official documentation](https://www.mercadopago.com.ar/developers/es/reference/preferences/_checkout_preferences/post) for more details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PreferenceRequest {
    /// List of items to be paid.
    pub items: Vec<PreferenceItem>,
    /// Information about the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<PreferencePayer>,
    /// URLs to redirect the user after the payment process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_urls: Option<BackUrls>,
    /// URL to receive notifications about the payment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    /// External reference for the preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    /// Expiration date of the preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_to: Option<String>,
}

/// Represents an item in a payment preference.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PreferenceItem {
    /// Title of the item.
    pub title: String,
    /// Description of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the item image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    /// Category of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// Quantity of the item.
    pub quantity: i32,
    /// Currency of the item price (e.g., ARS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Unit price of the item.
    pub unit_price: f64,
}

/// Information about the payer for a preference.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PreferencePayer {
    /// Name of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Surname of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// Email of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Phone>,
    /// Identification of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<Identification>,
}

/// Phone number of the payer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    /// Area code of the phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

/// Identification document of the payer.
// TODO(v0.2): unify with payments::Identification (this uses Option<String>, payments uses String)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Identification {
    /// Type of identification (e.g., DNI, CPF).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    /// Identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

/// Back URLs for redirecting the user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackUrls {
    /// URL to redirect after a successful payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// URL to redirect after a pending payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<String>,
    /// URL to redirect after a failed payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
}

/// Represents the response from creating a payment preference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreferenceResponse {
    /// Unique identifier of the preference.
    pub id: String,
    /// List of items.
    pub items: Vec<PreferenceItem>,
    /// URL to redirect the user to complete the payment (Checkout Pro).
    pub init_point: String,
    /// Sandbox URL to redirect the user for testing.
    pub sandbox_init_point: String,
    /// Date the preference was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
}
