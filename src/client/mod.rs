use crate::error::Error;
use crate::models::common::SearchResponse;
use crate::models::instore::{
    PosRequest, PosResponse, QrOrderRequest, QrOrderResponse, StoreRequest, StoreResponse,
};
use crate::models::payments::{PaymentRequest, PaymentResponse};
use crate::models::preferences::{PreferenceRequest, PreferenceResponse};
use crate::models::refunds::{RefundRequest, RefundResponse};
use reqwest::{Client, Method, RequestBuilder, Response};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

struct MercadoPagoClientInner {
    client: Client,
    access_token: String,
    base_url: String,
    max_retries: u32,
}

/// The main client for interacting with the Mercado Pago API.
///
/// Use the [`builder`](MercadoPagoClient::builder) method to create a new instance.
#[derive(Clone)]
pub struct MercadoPagoClient {
    inner: Arc<MercadoPagoClientInner>,
}

impl MercadoPagoClient {
    /// Returns a builder to configure and create a `MercadoPagoClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mercadopago_sdk::MercadoPagoClient;
    ///
    /// let client = MercadoPagoClient::builder("YOUR_ACCESS_TOKEN")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder(access_token: &str) -> MercadoPagoClientBuilder {
        MercadoPagoClientBuilder::new(access_token)
    }

    /// Performs a GET request on the specified path.
    pub fn get(&self, path: &str) -> MercadoPagoRequestBuilder {
        self.request(Method::GET, path)
    }

    /// Performs a POST request on the specified path.
    pub fn post(&self, path: &str) -> MercadoPagoRequestBuilder {
        self.request(Method::POST, path)
    }

    /// Performs a PUT request on the specified path.
    pub fn put(&self, path: &str) -> MercadoPagoRequestBuilder {
        self.request(Method::PUT, path)
    }

    /// Performs a DELETE request on the specified path.
    pub fn delete(&self, path: &str) -> MercadoPagoRequestBuilder {
        self.request(Method::DELETE, path)
    }

    /// Performs a PATCH request on the specified path.
    pub fn patch(&self, path: &str) -> MercadoPagoRequestBuilder {
        self.request(Method::PATCH, path)
    }

    /// Creates a custom request with the specified method and path.
    pub fn request(&self, method: Method, path: &str) -> MercadoPagoRequestBuilder {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.inner.base_url, path)
        };

        let rb = self
            .inner
            .client
            .request(method, &url)
            .bearer_auth(&self.inner.access_token);

        MercadoPagoRequestBuilder {
            builder: rb,
            client: self.clone(),
        }
    }

    /// Creates a new payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_payment(&self, request: PaymentRequest) -> Result<PaymentResponse, Error> {
        Ok(self
            .post("/v1/payments")
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Returns information about a payment by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the payment is not found or if the request fails.
    pub async fn get_payment(&self, id: u64) -> Result<PaymentResponse, Error> {
        Ok(self
            .get(&format!("/v1/payments/{}", id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// Searches for payments based on an external reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn search_payments(
        &self,
        external_reference: &str,
    ) -> Result<SearchResponse<PaymentResponse>, Error> {
        Ok(self
            .get("/v1/payments/search")
            .query(&[("external_reference", external_reference)])
            .send()
            .await?
            .json()
            .await?)
    }

    /// Generic search for payments with any criteria.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn search_payments_generic<T: serde::Serialize + ?Sized>(
        &self,
        filters: &T,
    ) -> Result<SearchResponse<PaymentResponse>, Error> {
        Ok(self
            .get("/v1/payments/search")
            .query(filters)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Creates a new payment preference for Checkout Pro.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_preference(
        &self,
        request: PreferenceRequest,
    ) -> Result<PreferenceResponse, Error> {
        Ok(self
            .post("/checkout/preferences")
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Creates a dynamic QR order for a specific collector and POS.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_qr_order(
        &self,
        user_id: u64,
        pos_id: &str,
        request: QrOrderRequest,
    ) -> Result<QrOrderResponse, Error> {
        Ok(self
            .post(&format!(
                "/instore/orders/qr/seller/collectors/{}/pos/{}/qrs",
                user_id, pos_id
            ))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Creates a new Store for a user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_store(
        &self,
        user_id: u64,
        request: StoreRequest,
    ) -> Result<StoreResponse, Error> {
        Ok(self
            .post(&format!("/users/{}/stores", user_id))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Creates a new Point of Sale (POS).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_pos(&self, request: PosRequest) -> Result<PosResponse, Error> {
        Ok(self
            .post("/pos")
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Searches for stores belonging to a user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn search_stores(
        &self,
        user_id: u64,
    ) -> Result<SearchResponse<StoreResponse>, Error> {
        Ok(self
            .get(&format!("/users/{}/stores/search", user_id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// Lists all Points of Sale (POS).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn list_pos(&self) -> Result<SearchResponse<PosResponse>, Error> {
        Ok(self.get("/pos").send().await?.json().await?)
    }

    /// Creates a refund for a payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the API returns an error response.
    pub async fn create_refund(
        &self,
        payment_id: u64,
        request: RefundRequest,
    ) -> Result<RefundResponse, Error> {
        Ok(self
            .post(&format!("/v1/payments/{}/refunds", payment_id))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    async fn execute_with_retry(&self, rb: RequestBuilder) -> Result<Response, Error> {
        let max_retries = self.inner.max_retries;
        let mut attempts = 0;

        loop {
            let rb_clone = rb
                .try_clone()
                .ok_or_else(|| Error::Internal("Cannot clone request for retry".to_string()))?;

            #[cfg(feature = "logging")]
            tracing::info!("Sending request to Mercado Pago");

            let res = rb_clone.send().await?;

            if res.status() == 429 {
                attempts += 1;
                if attempts > max_retries {
                    return Err(Error::ApiError {
                        message: "Too many retries".to_string(),
                        error: "too_many_requests".to_string(),
                        status: 429,
                        cause: None,
                    });
                }

                let retry_after = res
                    .headers()
                    .get("Retry-After")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(1);

                #[cfg(feature = "logging")]
                tracing::warn!(
                    "Received 429 Too Many Requests, retrying after {}s (attempt {}/{})",
                    retry_after,
                    attempts,
                    max_retries
                );

                sleep(Duration::from_secs(retry_after)).await;
                continue;
            }

            #[cfg(feature = "logging")]
            tracing::info!("Received response with status: {}", res.status());

            return Ok(res);
        }
    }
}

/// A builder for [`MercadoPagoClient`].
pub struct MercadoPagoClientBuilder {
    access_token: String,
    base_url: String,
    timeout: Duration,
    connect_timeout: Duration,
    max_retries: u32,
}

impl MercadoPagoClientBuilder {
    fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            max_retries: 3,
        }
    }

    /// Sets the base URL for the API. Defaults to `https://api.mercadopago.com`.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.trim_end_matches('/').to_string();
        self
    }

    /// Sets the total timeout for requests.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the connection timeout for requests.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Sets the maximum number of retries for rate-limited (429) requests. Defaults to 3.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Builds the `MercadoPagoClient`.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client fails to initialize.
    pub fn build(self) -> Result<MercadoPagoClient, Error> {
        let client = Client::builder()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .build()?;

        Ok(MercadoPagoClient {
            inner: Arc::new(MercadoPagoClientInner {
                client,
                access_token: self.access_token,
                base_url: self.base_url,
                max_retries: self.max_retries,
            }),
        })
    }
}

/// A wrapper around `reqwest::RequestBuilder` to add Mercado Pago specific logic like retries and error parsing.
pub struct MercadoPagoRequestBuilder {
    builder: RequestBuilder,
    client: MercadoPagoClient,
}

impl MercadoPagoRequestBuilder {
    /// Adds a header to the request.
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        reqwest::header::HeaderName: TryFrom<K>,
        <reqwest::header::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        reqwest::header::HeaderValue: TryFrom<V>,
        <reqwest::header::HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.builder = self.builder.header(key, value);
        self
    }

    /// Adds query parameters to the request.
    pub fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.builder = self.builder.query(query);
        self
    }

    /// Sets the JSON body of the request.
    pub fn json<T: serde::Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.builder = self.builder.json(json);
        self
    }

    /// Sends the request and returns the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the network request fails or if the API returns an error status code.
    pub async fn send(self) -> Result<Response, Error> {
        let res = self.client.execute_with_retry(self.builder).await?;

        if res.status().is_success() {
            #[cfg(feature = "logging")]
            {
                // Note: This consumes the body, so we can only do it if we return the body or clone it.
                // For temporary debugging, we'll just be careful.
            }
            Ok(res)
        } else {
            let status = res.status();
            let body = res
                .text()
                .await
                .map_err(|e| Error::Network(e.to_string()))?;
            let api_error: Error = serde_json::from_str(&body).unwrap_or_else(|_| {
                Error::Internal(format!(
                    "Failed to parse API error: {} (Status: {})",
                    body, status
                ))
            });
            Err(api_error)
        }
    }
}
