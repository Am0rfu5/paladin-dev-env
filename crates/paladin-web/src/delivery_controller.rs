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
//! A success body is the serialized domain type; failures use the unified
//! [`ApiError`](crate::error::ApiError) envelope (`{ "error": { "code", "message", "details" } }`).

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::adapters::api_content_deliverer::ApiContentDeliverer;
use crate::error::ApiError;
use paladin_ports::output::content_delivery_port::{ContentDeliveryService, DeliveryRequest};

/// JSON response body type used by every delivery handler's success path.
type JsonValue = Json<serde_json::Value>;

/// Serialize a successful payload to a JSON body (`null` on the unreachable error path).
fn ok_body<T: serde::Serialize>(value: &T) -> JsonValue {
    Json(serde_json::to_value(value).unwrap_or(serde_json::Value::Null))
}

/// `POST /api/delivery/deliver` — deliver a content payload immediately.
///
/// Returns `200 OK` with the [`DeliveryResponse`](paladin_ports::output::content_delivery_port::DeliveryResponse)
/// on success, or `400 Bad Request` (unified error envelope) if delivery fails.
pub async fn deliver_content(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
    Json(request): Json<DeliveryRequest>,
) -> Result<(StatusCode, JsonValue), ApiError> {
    match deliverer.deliver_content_async(request).await {
        Ok(response) => Ok((StatusCode::OK, ok_body(&response))),
        Err(e) => Err(ApiError::bad_request(e.to_string())),
    }
}

/// `GET /api/delivery/status/{delivery_id}` — fetch the status of a delivery by id.
///
/// Returns `200 OK` with the delivery record, `404 Not Found` if no such delivery exists, or
/// `400 Bad Request` if `delivery_id` is not a valid UUID.
pub async fn get_delivery_status(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
    Path(delivery_id): Path<String>,
) -> Result<(StatusCode, JsonValue), ApiError> {
    let id = Uuid::parse_str(&delivery_id)
        .map_err(|_| ApiError::bad_request("Invalid delivery ID format"))?;
    match deliverer.get_delivery_status(id) {
        Ok(response) => Ok((StatusCode::OK, ok_body(&response))),
        Err(e) => Err(ApiError::not_found(e.to_string())),
    }
}

/// `GET /api/delivery/stats` — aggregate delivery statistics across all recipients.
///
/// Returns `200 OK` with [`DeliveryStats`](paladin_ports::output::content_delivery_port::DeliveryStats),
/// or `500 Internal Server Error` (unified error envelope) on failure.
pub async fn get_delivery_stats(
    State(deliverer): State<Arc<ApiContentDeliverer>>,
) -> Result<(StatusCode, JsonValue), ApiError> {
    match deliverer.get_delivery_stats(None) {
        Ok(stats) => Ok((StatusCode::OK, ok_body(&stats))),
        Err(e) => Err(ApiError::internal(e.to_string())),
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
    use axum::body::Body;
    use axum::http::Request;
    use paladin_ports::output::content_delivery_port::{
        ContentPayload, DeliveryMethod, DeliveryPriority, NotificationContent,
    };
    use tower::ServiceExt; // for `Router::oneshot`

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
        let err = deliver_content(State(deliverer()), Json(sample_request()))
            .await
            .unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(err.to_body()["error"]["code"], "bad_request");
    }

    #[tokio::test]
    async fn get_delivery_status_unknown_id_returns_404() {
        let id = Uuid::new_v4().to_string();
        let err = get_delivery_status(State(deliverer()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(err.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_delivery_status_invalid_uuid_returns_400() {
        let err = get_delivery_status(State(deliverer()), Path("not-a-uuid".to_string()))
            .await
            .unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            err.to_body()["error"]["message"],
            "Invalid delivery ID format"
        );
    }

    #[tokio::test]
    async fn get_delivery_stats_returns_200() {
        let (status, _body) = get_delivery_stats(State(deliverer())).await.expect("ok");
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn stats_route_is_reachable_through_router() {
        let app = create_delivery_routes(deliverer());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/delivery/stats")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("router responds");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn status_invalid_uuid_route_returns_400_through_router() {
        let app = create_delivery_routes(deliverer());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/delivery/status/not-a-uuid")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("router responds");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
