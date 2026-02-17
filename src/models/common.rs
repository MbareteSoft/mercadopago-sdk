//! Common types shared across API domains.
use serde::{Deserialize, Serialize};

/// Generic paginated response wrapper used by search and list endpoints.
///
/// The Mercado Pago API returns search results in this format across
/// multiple endpoints (payments, stores, POS, etc.).
///
/// # Example
///
/// ```
/// use mercadopago_sdk::models::common::{SearchResponse, Paging};
///
/// let response: SearchResponse<String> = SearchResponse {
///     paging: Paging { total: 1, offset: 0, limit: 10 },
///     results: vec!["item".to_string()],
/// };
/// assert_eq!(response.results.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse<T> {
    /// Pagination metadata.
    pub paging: Paging,
    /// List of results for the current page.
    pub results: Vec<T>,
}

/// Pagination metadata returned by search endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paging {
    /// Total number of results matching the query.
    pub total: u32,
    /// Offset of the current page.
    pub offset: u32,
    /// Maximum number of results per page.
    pub limit: u32,
}
