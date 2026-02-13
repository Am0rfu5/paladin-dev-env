//! Test helpers and utilities
//!
//! Common test infrastructure including mocks, fixtures, and helper functions.

pub mod mock_llm_adapter;

pub use mock_llm_adapter::{
    MockLlmAdapter, create_mock_with_responses, create_test_paladin_with_mock,
};
