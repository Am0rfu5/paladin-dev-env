//! Axum HTTP controller for the content-delivery API.
//!
//! Exposes the content-delivery endpoints backed by
//! [`ApiContentDeliverer`](crate::adapters::api_content_deliverer::ApiContentDeliverer):
//!
//! | Method & path | Handler | Description |
//! |---------------|---------|-------------|
//! | `POST /api/delivery/deliver` | [`deliver_content`] | Deliver a content payload now |
//! | `GET /api/delivery/status/{delivery_id}` | [`get_delivery_status`] | Look up a delivery by id |
//! | `GET /api/delivery/stats` | [`get_delivery_stats`] | Aggregate delivery statistics |
//!
//! Build the router with [`create_delivery_routes`] and merge it into the application router.
//! Responses preserve the shape used by the previous implementation: a success body is the
//! serialized domain type, and an error body is `{ "error": "<message>" }`.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde_json::json;
use uuid::Uuid;

use crate::adapters::api_content_deliverer::ApiContentDeliverer;
use paladin_ports::output::content_delivery_port::{ContentDeliveryService, DeliveryRequest};

/// JSON response body type used by every delivery handler.
type JsonValue = Json<serde_json::Value>;

/// Serialize a successful payload to a JSON body, falling back to an error body if (very
/// unusually) serialization fails.
fn ok_body<T: serde::Serialize>(value: &T) -> JsonValue {
    match serde_json::to_value(value) {
        Ok(v) => Json(v),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// Build an `{ "error": "<message>" }` JSON body.
fn error_body(message: impl std::fmt::Display) -> JsonValue {
    Json(json!({ "error": message.to_string() }))
}

/// `POST /api/delivery/deliver` — deliver a content payload immediately.
///
/// Returns `200 OK` with the [`DeliveryResponse`](paladin_ports::output::content_delivery_port::DeliveryResponse)
/// on success, or `400 Bad Request` with `{ "error": ... }` if delivery fails.
pub async fn deliver_content(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
    Json(request): Json<DeliveryRequest>,
) -> (StatusCode, JsonValue) {
    match deliverer.deliver_content_async(request).await {
        Ok(response) => (StatusCode::OK, ok_body(&response)),
        Err(e) => (StatusCode::BAD_REQUEST, error_body(e)),
    }
}

/// `GET /api/delivery/status/{delivery_id}` — fetch the status of a delivery by id.
///
/// Returns `200 OK` with the delivery record, `404 Not Found` if no such delivery exists, or
/// `400 Bad Request` if `delivery_id` is not a valid UUID.
pub async fn get_delivery_status(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
    Path(delivery_id): Path<String>,
) -> (StatusCode, JsonValue) {
    match Uuid::parse_str(&delivery_id) {
        Ok(id) => match deliverer.get_delivery_status(id) {
            Ok(response) => (StatusCode::OK, ok_body(&response)),
            Err(e) => (StatusCode::NOT_FOUND, error_body(e)),
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            error_body("Invalid delivery ID format"),
        ),
    }
}

/// `GET /api/delivery/stats` — aggregate delivery statistics across all recipients.
///
/// Returns `200 OK` with [`DeliveryStats`](paladin_ports::output::content_delivery_port::DeliveryStats),
/// or `500 Internal Server Error` with `{ "error": ... }` on failure.
pub async fn get_delivery_stats(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
) -> (StatusCode, JsonValue) {
    match deliverer.get_delivery_stats(None) {
        Ok(stats) => (StatusCode::OK, ok_body(&stats)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, error_body(e)),
    }
}

/// Build the content-delivery router, with the shared [`ApiContentDeliverer`] injected as state.
///
/// Mount the returned router into the application router (see
/// [`create_app_router`](crate::app::create_app_router)).
pub fn create_delivery_routes(deliverer: Arc<ApiContentDeliverer>) -> Router {
    Router::new()
        .route("/api/delivery/deliver", post(deliver_content))
        .route(
            "/api/delivery/status/{delivery_id}",
            get(get_delivery_status),
        )
        .route("/api/delivery/stats", get(get_delivery_stats))
        .with_state(deliverer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_ports::output::content_delivery_port::{
        ContentPayload, DeliveryMethod, DeliveryPriority, NotificationContent,
    };

    fn deliverer() -> Arc<ApiContentDeliverer> {
        // One attempt, no backoff: keeps the delivery test fast and deterministic.
        Arc::new(ApiContentDeliverer::new().with_retry_config(1, 0))
    }

    fn sample_request() -> DeliveryRequest {
        DeliveryRequest {
            recipient_id: "test-recipient".to_string(),
            // Connection-refused address → fast, hermetic delivery failure (no real network).
            delivery_method: DeliveryMethod::Http {
                endpoint: "http://127.0.0.1:1/deliver".to_string(),
                headers: None,
            },
            content_payload: ContentPayload::Notification(NotificationContent {
                title: "Title".to_string(),
                body: "Body".to_string(),
                category: "test".to_string(),
                action_url: None,
                expires_at: None,
            }),
            priority: DeliveryPriority::Normal,
            scheduled_time: None,
            metadata: None,
        }
    }

    #[tokio::test]
    async fn deliver_content_failure_returns_400() {
        let (status, body) = deliver_content(State(deliverer()), Json(sample_request())).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body.0.get("error").is_some(), "expected an error body");
    }

    #[tokio::test]
    async fn get_delivery_status_unknown_id_returns_404() {
        let id = Uuid::new_v4().to_string();
        let (status, _body) = get_delivery_status(State(deliverer()), Path(id)).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_delivery_status_invalid_uuid_returns_400() {
        let (status, body) =
            get_delivery_status(State(deliverer()), Path("not-a-uuid".to_string())).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body.0.get("error").and_then(|v| v.as_str()),
            Some("Invalid delivery ID format")
        );
    }

    #[tokio::test]
    async fn get_delivery_stats_returns_200() {
        let (status, _body) = get_delivery_stats(State(deliverer())).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[test]
    fn create_delivery_routes_builds() {
        // Smoke check: the router builder wires up without panicking.
        let _router = create_delivery_routes(deliverer());
    }
}
