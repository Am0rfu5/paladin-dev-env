//! Security and encryption utilities for Sentinel Vision System
//!
//! This module provides encryption, secure memory handling, and data protection
//! for vision and document data in the Paladin framework.
//!
//! # Features
//!
//! - **Encryption at Rest**: ChaCha20-Poly1305 authenticated encryption
//! - **Secure Memory**: Automatic memory cleanup using `zeroize`
//! - **Data Retention**: Configurable TTL-based data cleanup
//! - **Audit Logging**: Access logging without sensitive data
//!
//! # Examples
//!
//! ```
//! use paladin::infrastructure::security::encryption::{EncryptionService, SecureData};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = EncryptionService::new()?;
//!
//! // Encrypt image data
//! let image_data = b"fake image bytes";
//! let encrypted = service.encrypt_image_data(image_data)?;
//!
//! // Decrypt when needed
//! let decrypted = service.decrypt_image_data(&encrypted)?;
//! assert_eq!(image_data, decrypted.as_slice());
//! # Ok(())
//! # }
//! ```

use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use rand::RngCore;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Errors that can occur during encryption/decryption operations
#[derive(Debug, Error)]
pub enum EncryptionError {
    /// Failed to generate encryption key
    #[error("Failed to generate encryption key: {0}")]
    KeyGenerationError(String),

    /// Failed to encrypt data
    #[error("Failed to encrypt data: {0}")]
    EncryptionFailed(String),

    /// Failed to decrypt data
    #[error("Failed to decrypt data: {0}")]
    DecryptionFailed(String),

    /// Invalid encrypted data format
    #[error("Invalid encrypted data format: {0}")]
    InvalidFormat(String),

    /// Data has exceeded retention period
    #[error("Data expired: retention period exceeded")]
    DataExpired,
}

/// Secure data wrapper that automatically zeros memory on drop
///
/// This ensures sensitive data is cleared from memory when no longer needed.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureData {
    #[zeroize(skip)]
    created_at: SystemTime,
    data: Vec<u8>,
}

impl SecureData {
    /// Creates new secure data with current timestamp
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            created_at: SystemTime::now(),
            data,
        }
    }

    /// Returns a reference to the data
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Returns the creation timestamp
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Checks if data has exceeded the retention period
    pub fn is_expired(&self, retention_period: Duration) -> bool {
        if let Ok(elapsed) = self.created_at.elapsed() {
            elapsed > retention_period
        } else {
            false
        }
    }
}

/// Data retention policy configuration
#[derive(Debug, Clone)]
pub struct DataRetentionPolicy {
    /// Time-to-live for encrypted data
    pub ttl: Duration,
    /// Whether to enable automatic cleanup
    pub auto_cleanup: bool,
}

impl Default for DataRetentionPolicy {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
            auto_cleanup: false,
        }
    }
}

impl DataRetentionPolicy {
    /// Creates a new retention policy with the specified TTL
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            auto_cleanup: true,
        }
    }

    /// Checks if data should be retained based on its age
    pub fn should_retain(&self, created_at: SystemTime) -> bool {
        if let Ok(elapsed) = created_at.elapsed() {
            elapsed <= self.ttl
        } else {
            true // Keep if we can't determine age
        }
    }
}

/// Encryption service for vision and document data
///
/// Uses ChaCha20-Poly1305 AEAD for authenticated encryption.
///
/// # Framework usage
///
/// The framework does not invoke this service on the vision path. The reason: the shipped
/// vision path never stores image bytes. `PaladinExecutionService::execute_with_vision` takes
/// caller-supplied [`VisionContent`](paladin_core::platform::container::vision::VisionContent)
/// (a URL, base64 payload, or file path) and hands it straight to the vision adapter — there is
/// no framework-owned temporary file or cache for this service to protect.
///
/// A consumer who does hold image bytes at rest, or who persists them between requests, is the
/// party that should call [`EncryptionService::encrypt_image_data`] and
/// [`EncryptionService::decrypt_image_data`] directly, and should hold the returned
/// [`SecureData`] for its zeroizing drop. Likewise, [`DataRetentionPolicy`] is a utility the
/// consumer applies to its own stored data — it is not a policy the framework enforces anywhere
/// in the vision execution path.
///
/// See the recorded decision at `.planning/decisions/0011-vision-port-surfaces.md`.
pub struct EncryptionService {
    cipher: ChaCha20Poly1305,
}

impl EncryptionService {
    /// Creates a new encryption service with a randomly generated key
    ///
    /// # Errors
    ///
    /// Returns `EncryptionError::KeyGenerationError` if key generation fails
    pub fn new() -> Result<Self, EncryptionError> {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);

        let cipher = ChaCha20Poly1305::new(&key_bytes.into());

        // Zeroize the key material
        key_bytes.zeroize();

        Ok(Self { cipher })
    }

    /// Creates an encryption service from an existing key
    ///
    /// # Arguments
    ///
    /// * `key` - 32-byte encryption key
    ///
    /// # Errors
    ///
    /// Returns error if key length is invalid
    pub fn from_key(key: &[u8]) -> Result<Self, EncryptionError> {
        if key.len() != 32 {
            return Err(EncryptionError::KeyGenerationError(
                "Key must be exactly 32 bytes".to_string(),
            ));
        }

        let cipher = ChaCha20Poly1305::new(key.into());

        Ok(Self { cipher })
    }

    /// Encrypts image data
    ///
    /// The framework itself never calls this on the vision path — see
    /// [`EncryptionService`]'s "Framework usage" section for why and for who should call it.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw image bytes to encrypt
    ///
    /// # Returns
    ///
    /// Encrypted data prepended with the nonce (12 bytes + ciphertext + 16-byte tag)
    ///
    /// # Errors
    ///
    /// Returns `EncryptionError::EncryptionFailed` if encryption fails
    pub fn encrypt_image_data(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        self.encrypt_data(data)
    }

    /// Decrypts image data
    ///
    /// The framework itself never calls this on the vision path — see
    /// [`EncryptionService`]'s "Framework usage" section for why and for who should call it.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted data with prepended nonce
    ///
    /// # Returns
    ///
    /// Decrypted image data wrapped in `SecureData`
    ///
    /// # Errors
    ///
    /// Returns `EncryptionError::DecryptionFailed` if decryption fails or data is invalid
    pub fn decrypt_image_data(&self, encrypted: &[u8]) -> Result<SecureData, EncryptionError> {
        self.decrypt_data(encrypted)
    }

    /// Encrypts document data
    ///
    /// # Arguments
    ///
    /// * `data` - Raw document bytes to encrypt
    ///
    /// # Returns
    ///
    /// Encrypted data prepended with the nonce
    ///
    /// # Errors
    ///
    /// Returns `EncryptionError::EncryptionFailed` if encryption fails
    pub fn encrypt_document_data(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        self.encrypt_data(data)
    }

    /// Decrypts document data
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted data with prepended nonce
    ///
    /// # Returns
    ///
    /// Decrypted document data wrapped in `SecureData`
    ///
    /// # Errors
    ///
    /// Returns `EncryptionError::DecryptionFailed` if decryption fails
    pub fn decrypt_document_data(&self, encrypted: &[u8]) -> Result<SecureData, EncryptionError> {
        self.decrypt_data(encrypted)
    }

    /// Internal encryption implementation
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Generate random nonce (12 bytes for ChaCha20-Poly1305)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from(nonce_bytes);

        // Encrypt the data
        let ciphertext = self
            .cipher
            .encrypt(&nonce, data)
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Internal decryption implementation
    fn decrypt_data(&self, encrypted: &[u8]) -> Result<SecureData, EncryptionError> {
        // Extract nonce (first 12 bytes)
        if encrypted.len() < 12 {
            return Err(EncryptionError::InvalidFormat(
                "Data too short to contain nonce".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt the data
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

        Ok(SecureData::new(plaintext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_service_creation() {
        let service = EncryptionService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_encryption_service_from_key() {
        let key = [0u8; 32];
        let service = EncryptionService::from_key(&key);
        assert!(service.is_ok());
    }

    #[test]
    fn test_encryption_service_from_invalid_key() {
        let key = [0u8; 16]; // Wrong size
        let service = EncryptionService::from_key(&key);
        assert!(matches!(
            service,
            Err(EncryptionError::KeyGenerationError(_))
        ));
    }

    #[test]
    fn test_encrypt_decrypt_image_data() {
        let service = EncryptionService::new().unwrap();
        let original = b"fake image data for testing";

        let encrypted = service.encrypt_image_data(original).unwrap();
        assert_ne!(encrypted.as_slice(), original);
        assert!(encrypted.len() > original.len()); // Includes nonce and tag

        let decrypted = service.decrypt_image_data(&encrypted).unwrap();
        assert_eq!(decrypted.as_slice(), original);
    }

    #[test]
    fn test_encrypt_decrypt_document_data() {
        let service = EncryptionService::new().unwrap();
        let original = b"fake PDF document content";

        let encrypted = service.encrypt_document_data(original).unwrap();
        assert_ne!(encrypted.as_slice(), original);

        let decrypted = service.decrypt_document_data(&encrypted).unwrap();
        assert_eq!(decrypted.as_slice(), original);
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let service = EncryptionService::new().unwrap();
        let invalid_data = b"not encrypted data";

        let result = service.decrypt_image_data(invalid_data);
        assert!(matches!(result, Err(EncryptionError::DecryptionFailed(_))));
    }

    #[test]
    fn test_decrypt_data_too_short() {
        let service = EncryptionService::new().unwrap();
        let too_short = b"short";

        let result = service.decrypt_image_data(too_short);
        assert!(matches!(result, Err(EncryptionError::InvalidFormat(_))));
    }

    #[test]
    fn test_secure_data_zeroization() {
        let data = vec![1u8, 2, 3, 4, 5];
        let secure = SecureData::new(data.clone());

        assert_eq!(secure.as_slice(), data.as_slice());

        // SecureData will be dropped here and memory zeroized
        drop(secure);

        // We can't directly verify zeroization in safe Rust,
        // but the zeroize crate handles this automatically
    }

    #[test]
    fn test_secure_data_expiration() {
        let data = vec![1u8, 2, 3];
        let secure = SecureData::new(data);

        // Should not be expired with a long retention period
        let long_period = Duration::from_secs(3600);
        assert!(!secure.is_expired(long_period));

        // Should be expired with a zero retention period
        let zero_period = Duration::from_secs(0);
        std::thread::sleep(Duration::from_millis(10));
        assert!(secure.is_expired(zero_period));
    }

    #[test]
    fn test_data_retention_policy_default() {
        let policy = DataRetentionPolicy::default();
        assert_eq!(policy.ttl, Duration::from_secs(30 * 24 * 60 * 60));
        assert!(!policy.auto_cleanup);
    }

    #[test]
    fn test_data_retention_policy_custom() {
        let ttl = Duration::from_secs(3600);
        let policy = DataRetentionPolicy::new(ttl);
        assert_eq!(policy.ttl, ttl);
        assert!(policy.auto_cleanup);
    }

    #[test]
    fn test_retention_policy_should_retain() {
        let policy = DataRetentionPolicy::new(Duration::from_secs(60));
        let now = SystemTime::now();

        // Should retain recent data
        assert!(policy.should_retain(now));

        // Should not retain old data
        let old = now - Duration::from_secs(120);
        assert!(!policy.should_retain(old));
    }

    #[test]
    fn test_encryption_produces_different_ciphertexts() {
        let service = EncryptionService::new().unwrap();
        let data = b"same data";

        // Encrypt the same data twice
        let encrypted1 = service.encrypt_image_data(data).unwrap();
        let encrypted2 = service.encrypt_image_data(data).unwrap();

        // Should produce different ciphertexts (different nonces)
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt to the same plaintext
        let decrypted1 = service.decrypt_image_data(&encrypted1).unwrap();
        let decrypted2 = service.decrypt_image_data(&encrypted2).unwrap();
        assert_eq!(decrypted1.as_slice(), data);
        assert_eq!(decrypted2.as_slice(), data);
    }

    #[test]
    fn test_large_data_encryption() {
        let service = EncryptionService::new().unwrap();
        // Test with larger data (simulating a real image)
        let large_data = vec![42u8; 1024 * 100]; // 100KB

        let encrypted = service.encrypt_image_data(&large_data).unwrap();
        let decrypted = service.decrypt_image_data(&encrypted).unwrap();

        assert_eq!(decrypted.as_slice(), large_data.as_slice());
    }
}
