//! Streamable-HTTP transport builder for MCP (rmcp-backed, D-02 remote half).
//!
//! This is the honestly-named replacement for the retired `MCPSseAdapter`
//! (plain, unauthenticated HTTP POST, never real SSE or Streamable-HTTP —
//! see Phase 12.1 Plan 01). `rmcp`'s `StreamableHttpClientTransport` already
//! owns session management, SSE reconnection, and the
//! `initialize -> notifications/initialized` handshake, so this adapter is a
//! thin builder that stores the endpoint + auth material and delegates the
//! actual connect to [`MCPClient::connect_streamable_http`] — mirroring
//! [`super::mcp_stdio_adapter::MCPStdioAdapter`]'s shape for the remote
//! transport (RESEARCH.md Pattern 2, Don't Hand-Roll).
//!
//! # Secret hygiene (D-03, T-12.1-03)
//!
//! The bearer token is held in [`BearerToken`], which zeroizes its contents
//! on drop and hand-implements a redacting `Debug` (never derives one) so no
//! log line or `{:?}` can ever leak the raw value — the token is exposed
//! only via [`BearerToken::expose_secret`], used exclusively inside
//! [`MCPStreamableHttpAdapter::connect`].
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp")
//!     .with_bearer_token("my-token-without-bearer-prefix");
//! let client = adapter.connect().await?;
//! let _ = client.discover_tools().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::ArsenalError;
use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use http::{HeaderName, HeaderValue};
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A bearer token / API-key secret, held only in memory for the lifetime of
/// the adapter and zeroized on drop.
///
/// Deliberately does NOT derive `Debug` — it hand-implements a redacting one
/// instead, so `{:?}` always prints a fixed placeholder rather than the raw
/// value (T-12.1-03). Never derive `Display` on this type either.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct BearerToken(String);

impl BearerToken {
    /// Wraps `token` for safe in-memory handling.
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }

    /// Returns the raw token. Callers MUST NOT log, print, or otherwise
    /// surface the returned value — used only to hand the token to the
    /// transport builder immediately before connecting.
    fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BearerToken").field(&"<redacted>").finish()
    }
}

/// Streamable-HTTP transport builder for remote MCP servers.
///
/// Stores the endpoint, an optional bearer token, and optional custom
/// headers; [`connect`](Self::connect) performs the actual connect + MCP
/// handshake via [`MCPClient::connect_streamable_http`].
#[derive(Debug)]
pub struct MCPStreamableHttpAdapter {
    /// The remote MCP server's Streamable-HTTP endpoint URI.
    endpoint: String,
    /// Optional bearer token (D-03) — see [`BearerToken`] for handling rules.
    bearer_token: Option<BearerToken>,
    /// Optional custom HTTP headers sent with every request.
    custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl MCPStreamableHttpAdapter {
    /// Creates a new Streamable-HTTP adapter targeting `endpoint`, with no
    /// auth configured yet.
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;
    ///
    /// let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp");
    /// ```
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            bearer_token: None,
            custom_headers: HashMap::new(),
        }
    }

    /// Attaches a bearer token (D-03). The token must NOT include a
    /// `"Bearer "` prefix — [`MCPClient::connect_streamable_http`] passes it
    /// straight to rmcp's `auth_header()`, which adds the prefix internally.
    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_token = Some(BearerToken::new(token));
        self
    }

    /// Attaches custom HTTP headers sent with every request (in addition to,
    /// or instead of, a bearer token).
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = headers;
        self
    }

    /// The configured endpoint URI.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Connects to the configured endpoint and performs the full MCP
    /// Streamable-HTTP handshake, returning a ready-to-use [`MCPClient`].
    ///
    /// # Errors
    ///
    /// See [`MCPClient::connect_streamable_http`] for the full error
    /// taxonomy (`AuthFailed`, `Timeout`, `ProtocolError`/`TransportError`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp")
    ///     .with_bearer_token("token");
    /// let client = adapter.connect().await?;
    /// # let _ = client;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&self) -> Result<MCPClient, ArsenalError> {
        let custom_headers = if self.custom_headers.is_empty() {
            None
        } else {
            Some(self.custom_headers.clone())
        };

        MCPClient::connect_streamable_http(
            &self.endpoint,
            self.bearer_token.as_ref().map(BearerToken::expose_secret),
            custom_headers,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_creation_defaults_to_no_auth() {
        let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp");
        assert_eq!(adapter.endpoint(), "https://mcp.example.com/mcp");
        assert!(adapter.bearer_token.is_none());
        assert!(adapter.custom_headers.is_empty());
    }

    #[test]
    fn with_bearer_token_stores_the_token() {
        let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp")
            .with_bearer_token("my-secret-token");
        assert_eq!(
            adapter
                .bearer_token
                .as_ref()
                .map(BearerToken::expose_secret),
            Some("my-secret-token")
        );
    }

    #[test]
    fn bearer_token_debug_never_leaks_the_raw_value() {
        let secret = "super-secret-value-should-never-appear-12345";
        let token = BearerToken::new(secret);

        let debug_output = format!("{token:?}");

        assert!(
            !debug_output.contains(secret),
            "BearerToken Debug output leaked the raw secret: {debug_output}"
        );
        assert!(debug_output.contains("redacted"));
    }

    #[test]
    fn adapter_debug_never_leaks_the_bearer_token() {
        let secret = "another-secret-that-must-not-leak-67890";
        let adapter =
            MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp").with_bearer_token(secret);

        let debug_output = format!("{adapter:?}");

        assert!(
            !debug_output.contains(secret),
            "MCPStreamableHttpAdapter Debug output leaked the raw secret: {debug_output}"
        );
    }

    #[test]
    fn with_custom_headers_stores_the_headers() {
        let mut headers = HashMap::new();
        headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom-value"),
        );
        let adapter = MCPStreamableHttpAdapter::new("https://mcp.example.com/mcp")
            .with_custom_headers(headers.clone());
        assert_eq!(adapter.custom_headers, headers);
    }
}
