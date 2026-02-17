use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents errors that can occur when using the Mercado Pago SDK.
#[derive(Debug, PartialEq, Error, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Error {
    /// Errors returned by the Mercado Pago API.
    #[error("API Error ({status}): {message}")]
    ApiError {
        message: String,
        #[serde(default)]
        error: String,
        #[serde(default)]
        status: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        cause: Option<Vec<Cause>>,
    },

    /// Errors that occur within the SDK or during network requests.
    #[error("Internal Error: {0}")]
    #[serde(skip)]
    Internal(String),

    /// Network-related errors.
    #[error("Network Error: {0}")]
    #[serde(skip)]
    Network(String),

    /// Serialization/Deserialization errors.
    #[error("Serialization Error: {0}")]
    #[serde(skip)]
    Serialization(String),
}

/// Represents a specific cause of an API error.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Cause {
    /// Error code returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Value>,
    /// Human-readable description of the error cause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional data associated with the error cause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_decode() {
            Error::Serialization(err.to_string())
        } else {
            Error::Network(err.to_string())
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}
