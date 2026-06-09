//! Unified API error model (Milestone 12, Epic 4).
//!
//! Every `paladin-web` handler renders failures through [`ApiError`], which serializes a
//! single, stable envelope:
//!
//! ```json
//! { "error": { "code": "not_found", "message": "unknown agent 'x'", "details": null } }
//! ```
//!
//! `code` is a stable `snake_case` machine identifier; `message` is human-facing; `details`
//! is optional structured context (rendered as `null` when absent). Constructors map to
//! the HTTP statuses the API uses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use utoipa::ToSchema;

/// OpenAPI schema for the error envelope returned by every failing handler.
///
/// Mirrors [`ApiError::to_body`] (`{ "error": { "code", "message", "details" } }`); it exists
/// purely so the generated spec can describe error responses. The runtime body is built by
/// `ApiError`, not this type.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorBody {
    /// The error detail object.
    pub error: ApiErrorDetail,
}

/// The `error` object inside [`ApiErrorBody`].
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorDetail {
    /// Stable, machine-readable error code (e.g. `"not_found"`, `"unauthorized"`).
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Optional structured context; `null` when absent.
    #[schema(value_type = Object, nullable)]
    pub details: Option<Value>,
}

/// A structured API error: HTTP status + machine `code` + human `message` + optional details.
#[derive(Debug, Clone)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<Value>,
}

impl ApiError {
    /// Construct an error from an explicit status, stable code, and message.
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details: None,
        }
    }

    /// Attach structured details (rendered under `error.details`).
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }

    /// The HTTP status this error renders with.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// `400 Bad Request` — malformed or invalid input (`code = "bad_request"`).
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    /// `401 Unauthorized` — missing or invalid credentials (`code = "unauthorized"`).
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    /// `403 Forbidden` — authenticated but not permitted (`code = "forbidden"`).
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "forbidden", message)
    }

    /// `404 Not Found` (`code = "not_found"`).
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// `409 Conflict` (`code = "conflict"`).
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "conflict", message)
    }

    /// `422 Unprocessable Entity` (`code = "unprocessable_entity"`).
    pub fn unprocessable(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            message,
        )
    }

    /// `413 Payload Too Large` (`code = "payload_too_large"`).
    pub fn payload_too_large(message: impl Into<String>) -> Self {
        Self::new(StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large", message)
    }

    /// `429 Too Many Requests` (`code = "too_many_requests"`).
    pub fn too_many_requests(message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, "too_many_requests", message)
    }

    /// `501 Not Implemented` (`code = "not_implemented"`).
    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED, "not_implemented", message)
    }

    /// `502 Bad Gateway` — an upstream (LLM/tool) execution failure (`code = "bad_gateway"`).
    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_GATEWAY, "bad_gateway", message)
    }

    /// `504 Gateway Timeout` — execution exceeded its deadline (`code = "gateway_timeout"`).
    pub fn gateway_timeout(message: impl Into<String>) -> Self {
        Self::new(StatusCode::GATEWAY_TIMEOUT, "gateway_timeout", message)
    }

    /// `500 Internal Server Error` (`code = "internal"`).
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal", message)
    }

    /// Render the JSON body (without the status), e.g. for an SSE `error` event.
    pub fn to_body(&self) -> Value {
        json!({
            "error": {
                "code": self.code,
                "message": self.message,
                "details": self.details,
            }
        })
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.to_body())).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_has_nested_envelope_with_null_details() {
        let err = ApiError::not_found("unknown agent 'x'");
        assert_eq!(err.status(), StatusCode::NOT_FOUND);
        let body = err.to_body();
        assert_eq!(body["error"]["code"], "not_found");
        assert_eq!(body["error"]["message"], "unknown agent 'x'");
        assert!(body["error"]["details"].is_null());
    }

    #[test]
    fn error_body_schema_mirrors_runtime_body() {
        // The documented schema type must round-trip a real ApiError body 1:1, so the spec
        // and the wire format can't drift.
        let body = ApiError::not_found("missing").to_body();
        let parsed: ApiErrorBody =
            serde_json::from_value(body.clone()).expect("schema type parses the runtime body");
        assert_eq!(parsed.error.code, "not_found");
        assert_eq!(parsed.error.message, "missing");
        assert!(parsed.error.details.is_none());
        // Re-serializing the schema type yields the same JSON shape.
        assert_eq!(serde_json::to_value(&parsed).unwrap(), body);
    }

    #[test]
    fn with_details_is_rendered() {
        let err = ApiError::bad_request("invalid").with_details(json!({ "field": "input" }));
        let body = err.to_body();
        assert_eq!(body["error"]["details"]["field"], "input");
    }

    #[test]
    fn constructors_map_to_expected_status_and_code() {
        for (err, status, code) in [
            (
                ApiError::bad_request("m"),
                StatusCode::BAD_REQUEST,
                "bad_request",
            ),
            (ApiError::not_found("m"), StatusCode::NOT_FOUND, "not_found"),
            (ApiError::conflict("m"), StatusCode::CONFLICT, "conflict"),
            (
                ApiError::unprocessable("m"),
                StatusCode::UNPROCESSABLE_ENTITY,
                "unprocessable_entity",
            ),
            (
                ApiError::payload_too_large("m"),
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
            ),
            (
                ApiError::too_many_requests("m"),
                StatusCode::TOO_MANY_REQUESTS,
                "too_many_requests",
            ),
            (
                ApiError::not_implemented("m"),
                StatusCode::NOT_IMPLEMENTED,
                "not_implemented",
            ),
            (
                ApiError::bad_gateway("m"),
                StatusCode::BAD_GATEWAY,
                "bad_gateway",
            ),
            (
                ApiError::gateway_timeout("m"),
                StatusCode::GATEWAY_TIMEOUT,
                "gateway_timeout",
            ),
            (
                ApiError::internal("m"),
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
            ),
        ] {
            assert_eq!(err.status(), status);
            assert_eq!(err.to_body()["error"]["code"], code);
        }
    }
}
