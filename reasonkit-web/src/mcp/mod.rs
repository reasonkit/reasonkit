//! Model Context Protocol (MCP) server module
//!
//! This module implements the MCP server for AI agent integration,
//! providing web sensing tools through the MCP protocol.

mod server;
mod tools;
/// MCP protocol types
pub mod types;

pub use server::McpServer;
pub use tools::{McpTool, ToolRegistry, AVAILABLE_TOOLS};
pub use types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpCapabilities, McpServerInfo,
    McpToolDefinition, ToolCallParams, ToolCallResult, ToolContent,
};
