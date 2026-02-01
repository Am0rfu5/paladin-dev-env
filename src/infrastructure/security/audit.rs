//! Audit logging for security events
//!
//! This module provides audit logging for file access and API calls
//! without logging sensitive data like image content or API keys.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during audit logging
#[derive(Debug, Error)]
pub enum AuditError {
    /// Failed to write audit log
    #[error("Failed to write audit log: {0}")]
    WriteError(String),

    /// Failed to serialize audit event
    #[error("Failed to serialize audit event: {0}")]
    SerializationError(String),
}

/// Types of audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// File access event (read)
    FileAccess,
    /// File write event
    FileWrite,
    /// LLM API call
    LlmApiCall,
    /// Vision processing
    VisionProcessing,
    /// Document processing
    DocumentProcessing,
    /// Encryption operation
    Encryption,
    /// Decryption operation
    Decryption,
}

/// Audit event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Type of event
    pub event_type: AuditEventType,
    /// User or system identifier
    pub user_id: String,
    /// Resource being accessed (e.g., file path, API endpoint)
    /// Note: This should NOT contain sensitive data
    pub resource: String,
    /// Action performed
    pub action: String,
    /// Success or failure status
    pub success: bool,
    /// Optional error message (should not contain sensitive data)
    pub error_message: Option<String>,
    /// Additional metadata (sanitized, no sensitive data)
    pub metadata: Option<serde_json::Value>,
}

impl AuditEvent {
    /// Creates a new audit event
    pub fn new(
        event_type: AuditEventType,
        user_id: impl Into<String>,
        resource: impl Into<String>,
        action: impl Into<String>,
        success: bool,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            user_id: user_id.into(),
            resource: resource.into(),
            action: action.into(),
            success,
            error_message: None,
            metadata: None,
        }
    }

    /// Sets the error message (should not contain sensitive data)
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error_message = Some(error.into());
        self
    }

    /// Sets metadata (should not contain sensitive data)
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Audit logger for security events
pub struct AuditLogger {
    enabled: bool,
}

impl AuditLogger {
    /// Creates a new audit logger
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Logs a file access event
    ///
    /// # Arguments
    ///
    /// * `user_id` - User or system identifier
    /// * `file_path` - Path to the file (sanitized, no sensitive data in path)
    /// * `success` - Whether the access was successful
    pub fn log_file_access(
        &self,
        user_id: impl Into<String>,
        file_path: &Path,
        success: bool,
    ) -> Result<(), AuditError> {
        if !self.enabled {
            return Ok(());
        }

        let event = AuditEvent::new(
            AuditEventType::FileAccess,
            user_id,
            file_path.display().to_string(),
            "read",
            success,
        );

        self.log_event(&event)
    }

    /// Logs an LLM API call event
    ///
    /// # Arguments
    ///
    /// * `user_id` - User or system identifier
    /// * `provider` - LLM provider name (e.g., "openai", "anthropic")
    /// * `model` - Model name (e.g., "gpt-4")
    /// * `success` - Whether the call was successful
    ///
    /// Note: This does NOT log the prompt or response content
    pub fn log_llm_api_call(
        &self,
        user_id: impl Into<String>,
        provider: impl Into<String>,
        model: impl Into<String>,
        success: bool,
    ) -> Result<(), AuditError> {
        if !self.enabled {
            return Ok(());
        }

        let provider_str = provider.into();
        let model_str = model.into();
        let resource = format!("{}/{}", provider_str, model_str);

        let event = AuditEvent::new(
            AuditEventType::LlmApiCall,
            user_id,
            resource,
            "api_call",
            success,
        );

        self.log_event(&event)
    }

    /// Logs a vision processing event
    ///
    /// # Arguments
    ///
    /// * `user_id` - User or system identifier
    /// * `image_count` - Number of images processed
    /// * `success` - Whether processing was successful
    pub fn log_vision_processing(
        &self,
        user_id: impl Into<String>,
        image_count: usize,
        success: bool,
    ) -> Result<(), AuditError> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = serde_json::json!({
            "image_count": image_count,
        });

        let event = AuditEvent::new(
            AuditEventType::VisionProcessing,
            user_id,
            "vision_system",
            "process_images",
            success,
        )
        .with_metadata(metadata);

        self.log_event(&event)
    }

    /// Logs a document processing event
    ///
    /// # Arguments
    ///
    /// * `user_id` - User or system identifier
    /// * `document_path` - Path to the document
    /// * `document_type` - Type of document (e.g., "pdf", "txt")
    /// * `success` - Whether processing was successful
    pub fn log_document_processing(
        &self,
        user_id: impl Into<String>,
        document_path: &Path,
        document_type: impl Into<String>,
        success: bool,
    ) -> Result<(), AuditError> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = serde_json::json!({
            "document_type": document_type.into(),
        });

        let event = AuditEvent::new(
            AuditEventType::DocumentProcessing,
            user_id,
            document_path.display().to_string(),
            "process_document",
            success,
        )
        .with_metadata(metadata);

        self.log_event(&event)
    }

    /// Logs an encryption event
    ///
    /// # Arguments
    ///
    /// * `user_id` - User or system identifier
    /// * `data_type` - Type of data being encrypted (e.g., "image", "document")
    /// * `data_size` - Size of data in bytes
    /// * `success` - Whether encryption was successful
    pub fn log_encryption(
        &self,
        user_id: impl Into<String>,
        data_type: impl Into<String>,
        data_size: usize,
        success: bool,
    ) -> Result<(), AuditError> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = serde_json::json!({
            "data_type": data_type.into(),
            "data_size_bytes": data_size,
        });

        let event = AuditEvent::new(
            AuditEventType::Encryption,
            user_id,
            "encryption_service",
            "encrypt",
            success,
        )
        .with_metadata(metadata);

        self.log_event(&event)
    }

    /// Internal method to log an event
    fn log_event(&self, event: &AuditEvent) -> Result<(), AuditError> {
        // In production, this would write to a file, database, or logging service
        // For now, we log to stderr using the log crate
        let event_json = serde_json::to_string(event)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;

        log::info!(target: "audit", "{}", event_json);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(
            AuditEventType::FileAccess,
            "user123",
            "/path/to/file.png",
            "read",
            true,
        );

        assert_eq!(event.event_type, AuditEventType::FileAccess);
        assert_eq!(event.user_id, "user123");
        assert_eq!(event.resource, "/path/to/file.png");
        assert_eq!(event.action, "read");
        assert!(event.success);
        assert!(event.error_message.is_none());
        assert!(event.metadata.is_none());
    }

    #[test]
    fn test_audit_event_with_error() {
        let event = AuditEvent::new(
            AuditEventType::FileAccess,
            "user123",
            "/path/to/file.png",
            "read",
            false,
        )
        .with_error("File not found");

        assert!(!event.success);
        assert_eq!(event.error_message, Some("File not found".to_string()));
    }

    #[test]
    fn test_audit_event_with_metadata() {
        let metadata = serde_json::json!({
            "file_size": 1024,
            "mime_type": "image/png",
        });

        let event = AuditEvent::new(
            AuditEventType::FileAccess,
            "user123",
            "/path/to/file.png",
            "read",
            true,
        )
        .with_metadata(metadata.clone());

        assert_eq!(event.metadata, Some(metadata));
    }

    #[test]
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::new(false);
        let result = logger.log_file_access("user123", &PathBuf::from("/test.png"), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_logger_file_access() {
        let logger = AuditLogger::new(true);
        let result = logger.log_file_access("user123", &PathBuf::from("/test.png"), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_logger_llm_api_call() {
        let logger = AuditLogger::new(true);
        let result = logger.log_llm_api_call("user123", "openai", "gpt-4", true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_logger_vision_processing() {
        let logger = AuditLogger::new(true);
        let result = logger.log_vision_processing("user123", 3, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_logger_document_processing() {
        let logger = AuditLogger::new(true);
        let result =
            logger.log_document_processing("user123", &PathBuf::from("/doc.pdf"), "pdf", true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_logger_encryption() {
        let logger = AuditLogger::new(true);
        let result = logger.log_encryption("user123", "image", 1024, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_event_serialization() {
        let event = AuditEvent::new(
            AuditEventType::FileAccess,
            "user123",
            "/path/to/file.png",
            "read",
            true,
        );

        let json = serde_json::to_string(&event);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("\"event_type\":\"file_access\""));
        assert!(json_str.contains("\"user_id\":\"user123\""));
        assert!(json_str.contains("\"resource\":\"/path/to/file.png\""));
    }

    #[test]
    fn test_audit_event_deserialization() {
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "event_type": "file_access",
            "user_id": "user123",
            "resource": "/path/to/file.png",
            "action": "read",
            "success": true,
            "error_message": null,
            "metadata": null
        }"#;

        let event: Result<AuditEvent, _> = serde_json::from_str(json);
        assert!(event.is_ok());

        let event = event.unwrap();
        assert_eq!(event.event_type, AuditEventType::FileAccess);
        assert_eq!(event.user_id, "user123");
    }
}
