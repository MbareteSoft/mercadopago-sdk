# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-02-16

### Added

- `list_store_payments` example: filter payments by `store_id` and/or `pos_id`
- GitHub Actions CI pipeline (`.github/workflows/ci.yml`): fmt, clippy, test, doc
- `CHANGELOG.md` following Keep a Changelog format
- `LICENSE` file (MIT)
- 14 new tests bringing total to 34 (19 client + 4 error + 9 model + 2 doc-tests):
  - Client: `search_payments`, `search_payments_generic`, `create_qr_order`, `create_store`, `create_pos`, `search_stores`, `list_pos`, `create_refund`
  - Error: `api_error_display_includes_status`, `internal_error_variant`, `serde_json_error_conversion`
  - Model: `refund_request_serialization`, `search_response_deserialization`, `skip_serializing_none_fields`

### Improved

- Complete rustdoc on all public items: 6 structs in `instore.rs`, 2 structs in `preferences.rs`, `Cause` fields in `error.rs`, `# Errors` sections on 6 client methods
- `Cargo.toml` metadata for crates.io: description, license, readme, repository, keywords, categories

### Fixed

- `.gitignore` was malformed (contained literal `\n` instead of newline)
- `.env.example` contained a real sandbox token — replaced with placeholder

### Security

- Sanitized `.env.example` to remove hardcoded access token

## [0.1.0] - 2025-02-16

### Added

- `MercadoPagoClient` with builder pattern, configurable timeouts, and retry logic
- Payments API: `create_payment`, `get_payment`, `search_payments`, `search_payments_generic`
- Checkout Pro API: `create_preference`
- In-store QR API: `create_qr_order`, `create_store`, `create_pos`, `search_stores`, `list_pos`
- Refunds API: `create_refund`
- Low-level HTTP methods: `get`, `post`, `put`, `delete`, `patch`
- Automatic retry with `Retry-After` support for 429 responses (configurable max retries)
- Structured error types: `ApiError`, `Internal`, `Network`, `Serialization`
- Idempotency key support via `X-Idempotency-Key` header
- Optional `tracing` instrumentation behind `logging` feature flag
- Type-safe request/response models for all supported APIs
- `wiremock`-based integration tests
