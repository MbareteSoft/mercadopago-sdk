use mercadopago_sdk::error::Error;
use serde_json::json;

#[test]
fn test_parse_api_error() {
    let error_json = json!({
        "message": "Invalid access token",
        "error": "invalid_token",
        "status": 401,
        "cause": [
            {
                "code": "1",
                "description": "Invalid parameter",
                "data": "access_token"
            }
        ]
    });

    let error_str = error_json.to_string();
    let api_error: Error = serde_json::from_str(&error_str).expect("Failed to parse error");

    match api_error {
        Error::ApiError {
            message, status, ..
        } => {
            assert_eq!(message, "Invalid access token");
            assert_eq!(status, 401);
        }
        _ => panic!("Expected ApiError"),
    }
}

#[test]
fn test_api_error_display_includes_status() {
    let error = Error::ApiError {
        message: "Not found".to_string(),
        error: "not_found".to_string(),
        status: 404,
        cause: None,
    };

    let display = format!("{}", error);
    assert!(
        display.contains("404"),
        "Display should contain status code: {}",
        display
    );
    assert!(
        display.contains("Not found"),
        "Display should contain message: {}",
        display
    );
}

#[test]
fn test_internal_error_variant() {
    let error = Error::Internal("something went wrong".to_string());
    let display = format!("{}", error);
    assert!(display.contains("something went wrong"));
}

#[test]
fn test_serde_json_error_conversion() {
    let bad_json = "{ invalid json }";
    let serde_err = serde_json::from_str::<serde_json::Value>(bad_json).unwrap_err();
    let error: Error = Error::from(serde_err);

    match error {
        Error::Serialization(msg) => {
            assert!(!msg.is_empty());
        }
        _ => panic!("Expected Serialization error"),
    }
}
