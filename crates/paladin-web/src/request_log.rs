//! Request-logging middleware with request-id correlation (Milestone 12, Epic 4).
//!
//! Applied as an `axum::middleware::from_fn` layer, [`request_log`] assigns a request-id
//! (honouring a well-formed inbound `x-request-id`, otherwise generating a UUID), runs the
//! request, logs one line at completion (method, path, status, latency) via `log`, and
//! returns the request-id to the client in the `x-request-id` response header.
//!
//! **Secret hygiene:** the log line contains only method, path, status, latency, and the
//! request-id — never request/response headers or bodies — so credentials such as
//! `Authorization` and `X-API-Key` are never logged.

use std::time::Instant;

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::header::HeaderName;
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

/// The correlation-id header, both inbound and on the response.
const REQUEST_ID_HEADER: &str = "x-request-id";

/// Whether an inbound request-id is safe to echo (non-empty, bounded, printable ASCII).
fn is_acceptable_request_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128 && value.chars().all(|c| c.is_ascii_graphic())
}

/// Axum middleware: correlate, time, and log each request.
pub async fn request_log(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    // Honour a well-formed inbound id (correlation across hops), else generate one.
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .filter(|s| is_acceptable_request_id(s))
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let start = Instant::now();
    let mut response = next.run(request).await;
    let latency_ms = start.elapsed().as_millis();
    let status = response.status().as_u16();

    log::info!("request_id={request_id} {method} {path} {status} {latency_ms}ms");

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID_HEADER), value);
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::routing::get;
    use tower::ServiceExt; // for `Router::oneshot`

    fn app() -> Router {
        Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(request_log))
    }

    #[tokio::test]
    async fn generates_request_id_header_when_absent() {
        let resp = app()
            .oneshot(HttpRequest::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let id = resp
            .headers()
            .get(REQUEST_ID_HEADER)
            .expect("x-request-id present");
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn echoes_well_formed_inbound_request_id() {
        let resp = app()
            .oneshot(
                HttpRequest::builder()
                    .uri("/")
                    .header(REQUEST_ID_HEADER, "abc-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.headers().get(REQUEST_ID_HEADER).unwrap(), "abc-123");
    }

    #[test]
    fn rejects_unacceptable_inbound_ids() {
        assert!(!is_acceptable_request_id(""));
        assert!(!is_acceptable_request_id("has space"));
        assert!(!is_acceptable_request_id(&"x".repeat(200)));
        assert!(is_acceptable_request_id("abc-123"));
    }
}
