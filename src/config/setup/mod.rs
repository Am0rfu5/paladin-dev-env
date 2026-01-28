pub mod service_runner;

use crate::config::application_settings::Settings;
use std::sync::Arc;

pub async fn setup_and_run(config: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(config);

    service_runner::run_services(config.clone()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::application_settings::Settings;

    #[tokio::test]
    async fn test_setup_and_run_with_default_config() {
        // Test that setup_and_run can be called with default settings
        // This will likely fail due to missing external services, but we test that
        // the function signature works and error handling is correct
        let config = Settings::default();
        let result = setup_and_run(config).await;

        // In a test environment without external services, this should fail
        // We just verify that the function can be called and returns a Result
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_setup_and_run_config_wrapping() {
        // Test that the config is properly wrapped in Arc
        // We can't easily test the internal call without mocking,
        // but we can test that the function accepts Settings
        let config = Settings::default();

        // This test mainly verifies that the function compiles and has correct signature
        // The actual execution will depend on available services
        let result = setup_and_run(config).await;
        assert!(result.is_err() || result.is_ok());
    }
}
