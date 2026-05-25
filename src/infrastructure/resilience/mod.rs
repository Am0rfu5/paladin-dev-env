//! Infrastructure resilience primitives.
//!
//! This module is the canonical home for fault-tolerance and resilience
//! utilities used by infrastructure adapters and injected into application
//! services.
//!
//! Resilience concerns — circuit breaking, retry policies, rate limiting,
//! concurrency limiting — belong in the infrastructure layer because they
//! are implementation details of *how the system handles failure*, not
//! business logic about *what the system does*.
//!
//! # Current Modules
//!
//! - [`circuit_breaker`] — Three-state circuit breaker (Closed/Open/HalfOpen)
//!   that prevents cascading failures by temporarily blocking calls to a
//!   failing dependency.
//!
//! # Planned Additions (not yet implemented)
//!
//! - `retry` — Configurable retry policy with exponential backoff
//! - `rate_limiter` — Token-bucket rate limiter for LLM API calls
//! - `bulkhead` — Concurrency limiter for external service calls

pub mod circuit_breaker;
