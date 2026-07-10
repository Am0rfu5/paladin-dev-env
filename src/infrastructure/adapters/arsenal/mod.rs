//! Arsenal adapters for tool execution via Model Context Protocol (MCP)
//!
//! This module provides MCP protocol implementation and transport adapters
//! for connecting to external tool servers.

pub mod mcp_protocol;
pub mod mcp_stdio_adapter;
pub mod mcp_streamable_http_adapter;
pub mod resource_controls;
pub mod tool_result_formatter;
