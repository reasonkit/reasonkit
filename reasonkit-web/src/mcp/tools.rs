//! MCP tool definitions and registry
//!
//! This module defines the available MCP tools and their implementations.

use crate::browser::{BrowserController, CaptureFormat, CaptureOptions, PageCapture};
use crate::error::Result;
use crate::extraction::{ContentExtractor, LinkExtractor, MetadataExtractor};
use crate::mcp::types::{McpToolDefinition, ToolCallResult, ToolContent};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, instrument};

/// A registered MCP tool
pub trait McpTool: Send + Sync {
    /// Tool name
    fn name(&self) -> &str;
    /// Tool description
    fn description(&self) -> &str;
    /// Input schema as JSON
    fn input_schema(&self) -> Value;
    /// Get tool definition
    fn definition(&self) -> McpToolDefinition {
        McpToolDefinition {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: self.input_schema(),
        }
    }
}

/// Tool registry holding all available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn McpTool>>,
    #[allow(dead_code)]
    browser: Arc<RwLock<Option<BrowserController>>>,
}

impl ToolRegistry {
    /// Create a new tool registry with all built-in tools
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
            browser: Arc::new(RwLock::new(None)),
        };

        // Register all built-in tools
        registry.register(Box::new(WebNavigateTool));
        registry.register(Box::new(WebScreenshotTool));
        registry.register(Box::new(WebPdfTool));
        registry.register(Box::new(WebExtractContentTool));
        registry.register(Box::new(WebExtractLinksTool));
        registry.register(Box::new(WebExtractMetadataTool));
        registry.register(Box::new(WebExecuteJsTool));
        registry.register(Box::new(WebCaptureMhtmlTool));

        registry
    }

    /// Register a tool
    pub fn register(&mut self, tool: Box<dyn McpTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Get all tool definitions
    pub fn definitions(&self) -> Vec<McpToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Execute a tool by name
    #[instrument(skip(self, args))]
    pub async fn execute(&self, name: &str, args: Value) -> ToolCallResult {
        info!("Executing tool: {}", name);

        if !self.tools.contains_key(name) {
            return ToolCallResult::error(format!("Tool not found: {}", name));
        }

        // Ensure browser is available
        let browser = self.get_or_create_browser().await;
        let browser = match browser {
            Ok(b) => b,
            Err(e) => return ToolCallResult::error(format!("Failed to create browser: {}", e)),
        };

        match name {
            "web_navigate" => self.execute_navigate(&browser, args).await,
            "web_screenshot" => self.execute_screenshot(&browser, args).await,
            "web_pdf" => self.execute_pdf(&browser, args).await,
            "web_extract_content" => self.execute_extract_content(&browser, args).await,
            "web_extract_links" => self.execute_extract_links(&browser, args).await,
            "web_extract_metadata" => self.execute_extract_metadata(&browser, args).await,
            "web_execute_js" => self.execute_js(&browser, args).await,
            "web_capture_mhtml" => self.execute_capture_mhtml(&browser, args).await,
            _ => ToolCallResult::error(format!("Unknown tool: {}", name)),
        }
    }

    /// Get or create browser instance
    async fn get_or_create_browser(&self) -> Result<BrowserController> {
        // For simplicity, create a new browser each time
        // In production, you'd want to pool/reuse browsers
        BrowserController::new().await
    }

    async fn execute_navigate(&self, browser: &BrowserController, args: Value) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        match browser.navigate(url).await {
            Ok(page) => {
                let current_url = page.url().await;
                ToolCallResult::text(format!("Successfully navigated to: {}", current_url))
            }
            Err(e) => {
                error!("Navigation failed: {}", e);
                ToolCallResult::error(format!("Navigation failed: {}", e))
            }
        }
    }

    async fn execute_screenshot(&self, browser: &BrowserController, args: Value) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        let full_page = args
            .get("fullPage")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let format_str = args.get("format").and_then(|v| v.as_str()).unwrap_or("png");

        let format = match format_str {
            "jpeg" | "jpg" => CaptureFormat::Jpeg,
            "webp" => CaptureFormat::Webp,
            _ => CaptureFormat::Png,
        };

        match browser.navigate(url).await {
            Ok(page) => {
                let options = CaptureOptions {
                    format,
                    full_page,
                    as_base64: true,
                    ..Default::default()
                };

                match PageCapture::capture(&page, &options).await {
                    Ok(result) => {
                        let base64 = result.base64.clone().unwrap_or_else(|| result.to_base64());
                        ToolCallResult::image(base64, result.mime_type())
                    }
                    Err(e) => ToolCallResult::error(format!("Screenshot failed: {}", e)),
                }
            }
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_pdf(&self, browser: &BrowserController, args: Value) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        match browser.navigate(url).await {
            Ok(page) => {
                let options = CaptureOptions::pdf();

                match PageCapture::capture(&page, &options).await {
                    Ok(result) => {
                        let base64 = result.to_base64();
                        ToolCallResult::multi(vec![
                            ToolContent::text(format!("PDF generated: {} bytes", result.size)),
                            ToolContent::Resource {
                                uri: format!("pdf://{}", url),
                                resource: crate::mcp::types::ResourceContent {
                                    mime_type: "application/pdf".to_string(),
                                    text: None,
                                    blob: Some(base64),
                                },
                            },
                        ])
                    }
                    Err(e) => ToolCallResult::error(format!("PDF generation failed: {}", e)),
                }
            }
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_extract_content(
        &self,
        browser: &BrowserController,
        args: Value,
    ) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        let selector = args.get("selector").and_then(|v| v.as_str());
        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("markdown");

        match browser.navigate(url).await {
            Ok(page) => {
                let content = if let Some(sel) = selector {
                    ContentExtractor::extract_from_selector(&page, sel).await
                } else {
                    ContentExtractor::extract_main_content(&page).await
                };

                match content {
                    Ok(c) => {
                        let output = match format {
                            "text" => c.text,
                            "html" => c.html,
                            _ => c.markdown.unwrap_or(c.text),
                        };
                        ToolCallResult::text(output)
                    }
                    Err(e) => ToolCallResult::error(format!("Content extraction failed: {}", e)),
                }
            }
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_extract_links(
        &self,
        browser: &BrowserController,
        args: Value,
    ) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        let link_type = args.get("type").and_then(|v| v.as_str());
        let selector = args.get("selector").and_then(|v| v.as_str());

        match browser.navigate(url).await {
            Ok(page) => {
                let links = if let Some(sel) = selector {
                    LinkExtractor::extract_from_selector(&page, sel).await
                } else {
                    match link_type {
                        Some("internal") => LinkExtractor::extract_internal(&page).await,
                        Some("external") => LinkExtractor::extract_external(&page).await,
                        _ => LinkExtractor::extract_all(&page).await,
                    }
                };

                match links {
                    Ok(links) => {
                        let json = serde_json::to_string_pretty(&links)
                            .unwrap_or_else(|_| "[]".to_string());
                        ToolCallResult::text(json)
                    }
                    Err(e) => ToolCallResult::error(format!("Link extraction failed: {}", e)),
                }
            }
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_extract_metadata(
        &self,
        browser: &BrowserController,
        args: Value,
    ) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        match browser.navigate(url).await {
            Ok(page) => match MetadataExtractor::extract(&page).await {
                Ok(meta) => {
                    let json =
                        serde_json::to_string_pretty(&meta).unwrap_or_else(|_| "{}".to_string());
                    ToolCallResult::text(json)
                }
                Err(e) => ToolCallResult::error(format!("Metadata extraction failed: {}", e)),
            },
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_js(&self, browser: &BrowserController, args: Value) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        let script = match args.get("script").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return ToolCallResult::error("Missing required parameter: script"),
        };

        match browser.navigate(url).await {
            Ok(page) => match page.page.evaluate(script).await {
                Ok(result) => {
                    let value: Value = result.into_value().unwrap_or(Value::Null);
                    let output =
                        serde_json::to_string_pretty(&value).unwrap_or_else(|_| "null".to_string());
                    ToolCallResult::text(output)
                }
                Err(e) => ToolCallResult::error(format!("JavaScript execution failed: {}", e)),
            },
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }

    async fn execute_capture_mhtml(
        &self,
        browser: &BrowserController,
        args: Value,
    ) -> ToolCallResult {
        let url = match args.get("url").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => return ToolCallResult::error("Missing required parameter: url"),
        };

        match browser.navigate(url).await {
            Ok(page) => match PageCapture::mhtml(&page).await {
                Ok(result) => {
                    let base64 = result.to_base64();
                    ToolCallResult::multi(vec![
                        ToolContent::text(format!("MHTML captured: {} bytes", result.size)),
                        ToolContent::Resource {
                            uri: format!("mhtml://{}", url),
                            resource: crate::mcp::types::ResourceContent {
                                mime_type: "multipart/related".to_string(),
                                text: None,
                                blob: Some(base64),
                            },
                        },
                    ])
                }
                Err(e) => ToolCallResult::error(format!("MHTML capture failed: {}", e)),
            },
            Err(e) => ToolCallResult::error(format!("Navigation failed: {}", e)),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tool Definitions
// ============================================================================

/// Navigate to a URL
struct WebNavigateTool;

impl McpTool for WebNavigateTool {
    fn name(&self) -> &str {
        "web_navigate"
    }

    fn description(&self) -> &str {
        "Navigate to a URL using a headless browser"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to navigate to"
                },
                "waitFor": {
                    "type": "string",
                    "description": "CSS selector to wait for before returning",
                    "optional": true
                }
            },
            "required": ["url"]
        })
    }
}

/// Capture screenshot
struct WebScreenshotTool;

impl McpTool for WebScreenshotTool {
    fn name(&self) -> &str {
        "web_screenshot"
    }

    fn description(&self) -> &str {
        "Capture a screenshot of a web page"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to capture"
                },
                "fullPage": {
                    "type": "boolean",
                    "description": "Capture full page (default: true)",
                    "default": true
                },
                "format": {
                    "type": "string",
                    "enum": ["png", "jpeg", "webp"],
                    "description": "Image format (default: png)",
                    "default": "png"
                },
                "selector": {
                    "type": "string",
                    "description": "CSS selector to capture specific element"
                }
            },
            "required": ["url"]
        })
    }
}

/// Generate PDF
struct WebPdfTool;

impl McpTool for WebPdfTool {
    fn name(&self) -> &str {
        "web_pdf"
    }

    fn description(&self) -> &str {
        "Generate a PDF of a web page"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to convert to PDF"
                },
                "printBackground": {
                    "type": "boolean",
                    "description": "Print background graphics (default: true)",
                    "default": true
                }
            },
            "required": ["url"]
        })
    }
}

/// Extract content
struct WebExtractContentTool;

impl McpTool for WebExtractContentTool {
    fn name(&self) -> &str {
        "web_extract_content"
    }

    fn description(&self) -> &str {
        "Extract main content from a web page as text or markdown"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to extract content from"
                },
                "selector": {
                    "type": "string",
                    "description": "CSS selector to extract from (default: auto-detect main content)"
                },
                "format": {
                    "type": "string",
                    "enum": ["text", "markdown", "html"],
                    "description": "Output format (default: markdown)",
                    "default": "markdown"
                }
            },
            "required": ["url"]
        })
    }
}

/// Extract links
struct WebExtractLinksTool;

impl McpTool for WebExtractLinksTool {
    fn name(&self) -> &str {
        "web_extract_links"
    }

    fn description(&self) -> &str {
        "Extract all links from a web page with context"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to extract links from"
                },
                "type": {
                    "type": "string",
                    "enum": ["all", "internal", "external"],
                    "description": "Type of links to extract (default: all)",
                    "default": "all"
                },
                "selector": {
                    "type": "string",
                    "description": "CSS selector to extract links from"
                }
            },
            "required": ["url"]
        })
    }
}

/// Extract metadata
struct WebExtractMetadataTool;

impl McpTool for WebExtractMetadataTool {
    fn name(&self) -> &str {
        "web_extract_metadata"
    }

    fn description(&self) -> &str {
        "Extract page metadata (title, description, Open Graph, Twitter Card, etc.)"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to extract metadata from"
                }
            },
            "required": ["url"]
        })
    }
}

/// Execute JavaScript
struct WebExecuteJsTool;

impl McpTool for WebExecuteJsTool {
    fn name(&self) -> &str {
        "web_execute_js"
    }

    fn description(&self) -> &str {
        "Execute JavaScript on a web page and return the result"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to execute JavaScript on"
                },
                "script": {
                    "type": "string",
                    "description": "The JavaScript code to execute"
                }
            },
            "required": ["url", "script"]
        })
    }
}

/// Capture MHTML
struct WebCaptureMhtmlTool;

impl McpTool for WebCaptureMhtmlTool {
    fn name(&self) -> &str {
        "web_capture_mhtml"
    }

    fn description(&self) -> &str {
        "Capture a complete web page as an MHTML archive"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to capture"
                }
            },
            "required": ["url"]
        })
    }
}

/// List of all available tools (for documentation)
pub const AVAILABLE_TOOLS: &[&str] = &[
    "web_navigate",
    "web_screenshot",
    "web_pdf",
    "web_extract_content",
    "web_extract_links",
    "web_extract_metadata",
    "web_execute_js",
    "web_capture_mhtml",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_registry_new() {
        let registry = ToolRegistry::new();
        assert!(registry.tools.len() >= 8);
    }

    #[test]
    fn test_tool_definitions() {
        let registry = ToolRegistry::new();
        let defs = registry.definitions();
        assert!(!defs.is_empty());

        // Check that web_navigate exists
        let nav = defs.iter().find(|d| d.name == "web_navigate");
        assert!(nav.is_some());
    }

    #[test]
    fn test_web_navigate_tool() {
        let tool = WebNavigateTool;
        assert_eq!(tool.name(), "web_navigate");
        assert!(tool.description().contains("Navigate"));

        let schema = tool.input_schema();
        assert!(schema["properties"]["url"].is_object());
    }

    #[test]
    fn test_available_tools() {
        assert!(AVAILABLE_TOOLS.contains(&"web_navigate"));
        assert!(AVAILABLE_TOOLS.contains(&"web_screenshot"));
        assert!(AVAILABLE_TOOLS.contains(&"web_execute_js"));
    }
}
