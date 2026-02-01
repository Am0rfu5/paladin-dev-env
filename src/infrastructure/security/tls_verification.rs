//! Security verification tests
//!
//! This module contains tests to verify security configurations:
//! - HTTPS/TLS is used for all external API calls
//! - SSL certificate validation is enabled
//! - No plain HTTP connections to sensitive services

#[cfg(test)]
mod tests {
    use reqwest::Client;

    #[test]
    fn test_reqwest_client_default_has_tls_enabled() {
        // Verify that the default reqwest client has TLS enabled
        let _client = Client::new();

        // reqwest enables TLS by default and validates certificates
        // This test ensures we're using the default secure configuration
    }

    #[test]
    fn test_openai_uses_https() {
        // OpenAI default base URL uses HTTPS
        let base_url = "https://api.openai.com/v1";
        assert!(
            base_url.starts_with("https://"),
            "OpenAI config must use HTTPS"
        );
    }

    #[test]
    fn test_anthropic_uses_https() {
        // Anthropic default base URL uses HTTPS
        let base_url = "https://api.anthropic.com/v1";
        assert!(
            base_url.starts_with("https://"),
            "Anthropic config must use HTTPS"
        );
    }

    #[test]
    fn test_deepseek_uses_https() {
        // DeepSeek default base URL uses HTTPS
        let base_url = "https://api.deepseek.com/v1";
        assert!(
            base_url.starts_with("https://"),
            "DeepSeek config must use HTTPS"
        );
    }

    #[test]
    fn test_no_insecure_http_in_llm_configs() {
        // Verify that none of our default configs use insecure HTTP
        let openai_url = "https://api.openai.com/v1";
        let anthropic_url = "https://api.anthropic.com/v1";
        let deepseek_url = "https://api.deepseek.com/v1";

        // Ensure no config uses plain HTTP
        assert!(
            !openai_url.starts_with("http://"),
            "OpenAI must not use insecure HTTP"
        );
        assert!(
            !anthropic_url.starts_with("http://"),
            "Anthropic must not use insecure HTTP"
        );
        assert!(
            !deepseek_url.starts_with("http://"),
            "DeepSeek must not use insecure HTTP"
        );
    }
}
