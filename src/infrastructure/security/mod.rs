//! Security module for Sentinel Vision System (Epic 13, Task 11.0)
//!
//! This module provides comprehensive security features for vision and document processing:
//!
//! - **Encryption**: ChaCha20-Poly1305 authenticated encryption for data at rest
//! - **Secure Memory**: Automatic memory zeroization using `zeroize` crate
//! - **Data Retention**: Configurable TTL-based data lifecycle management
//! - **Audit Logging**: Security event logging without sensitive data
//!
//! # Example
//!
//! ```
//! use paladin::infrastructure::security::{
//!     encryption::EncryptionService,
//!     audit::AuditLogger,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create encryption service
//! let encryption = EncryptionService::new()?;
//!
//! // Create audit logger
//! let audit = AuditLogger::new(true);
//!
//! // Encrypt sensitive data
//! let data = b"sensitive image data";
//! let encrypted = encryption.encrypt_image_data(data)?;
//!
//! // Log the operation
//! audit.log_encryption("user123", "image", data.len(), true)?;
//!
//! // Decrypt when needed
//! let decrypted = encryption.decrypt_image_data(&encrypted)?;
//! # Ok(())
//! # }
//! ```

pub mod audit;
pub mod encryption;
pub mod tls_verification;

// Re-export main types for convenience
pub use audit::{AuditError, AuditEvent, AuditEventType, AuditLogger};
pub use encryption::{DataRetentionPolicy, EncryptionError, EncryptionService, SecureData};
