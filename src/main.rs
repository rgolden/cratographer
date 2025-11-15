mod analyzer;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{CallToolResult, Content, ErrorData as McpError, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler, ServiceExt,
    transport::stdio,
};

/// Cratographer MCP Server
/// Provides tools for indexing and querying Rust code symbols
#[derive(Debug, Clone)]
struct CratographerServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CratographerServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Find all occurrences of a symbol by name across the indexed codebase
    /// TODO: Expose search mode, include_library, and types_only parameters via MCP
    #[tool(description = "Find all occurrences of a Rust symbol (struct, enum, trait, function, method) by name")]
    async fn find_symbol(&self) -> Result<CallToolResult, McpError> {
        // Placeholder - will be implemented with actual analyzer integration
        Ok(CallToolResult::success(vec![Content::text(
            "find_symbol tool - search parameters (mode, include_library, types_only) added to analyzer API".to_string()
        )]))
    }

    /// List all symbols defined in a specific file
    #[tool(description = "Enumerate all Rust symbols defined in a specific file")]
    async fn enumerate_file(&self) -> Result<CallToolResult, McpError> {
        // No-op implementation for now
        Ok(CallToolResult::success(vec![Content::text(
            "enumerate_file tool (not yet implemented)".to_string()
        )]))
    }
}

#[tool_handler]
impl ServerHandler for CratographerServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "cratographer".to_string(),
                version: "0.1.0".to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            instructions: Some(
                "Cratographer: An MCP tool for indexing and analyzing Rust code. \
                Use find_symbol to locate symbol definitions and enumerate_file to list all symbols in a file."
                    .to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the server instance and start serving
    let service = CratographerServer::new().serve(stdio()).await?;

    // Wait for shutdown
    service.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_symbol_returns_ok() {
        let server = CratographerServer::new();
        let result = server.find_symbol().await;

        assert!(result.is_ok(), "find_symbol should return Ok");
        let tool_result = result.unwrap();

        // Check that we got content back
        assert!(!tool_result.content.is_empty(), "Result should contain content");

        // Verify it's a success result (not an error)
        assert!(!tool_result.is_error.unwrap_or(false), "Result should not be an error");
    }

    #[tokio::test]
    async fn test_enumerate_file_returns_ok() {
        let server = CratographerServer::new();
        let result = server.enumerate_file().await;

        assert!(result.is_ok(), "enumerate_file should return Ok");
        let tool_result = result.unwrap();

        // Check that we got content back
        assert!(!tool_result.content.is_empty(), "Result should contain content");

        // Verify it's a success result (not an error)
        assert!(!tool_result.is_error.unwrap_or(false), "Result should not be an error");
    }

    #[test]
    fn test_server_info() {
        let server = CratographerServer::new();
        let info = server.get_info();

        // Verify server name and version
        assert_eq!(info.server_info.name, "cratographer");
        assert_eq!(info.server_info.version, "0.1.0");

        // Verify protocol version
        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);

        // Verify capabilities - should have tools enabled
        assert!(
            info.capabilities.tools.is_some(),
            "Server should have tools capability"
        );

        // Verify instructions are present
        assert!(
            info.instructions.is_some(),
            "Server should have instructions"
        );
        let instructions = info.instructions.unwrap();
        assert!(
            instructions.to_lowercase().contains("cratographer"),
            "Instructions should mention Cratographer"
        );
        assert!(
            instructions.contains("find_symbol"),
            "Instructions should mention find_symbol"
        );
        assert!(
            instructions.contains("enumerate_file"),
            "Instructions should mention enumerate_file"
        );
    }

    #[test]
    fn test_server_creation() {
        let _server = CratographerServer::new();
        // Just verify we can create the server without panicking
        // If we get here, the server was created successfully
    }
}
