//! ReasonKit Web - High-Performance Web Sensing & Browser Automation
//!
//! This binary runs the MCP (Model Context Protocol) server for web sensing,
//! providing AI agents with browser automation and content extraction capabilities.

use anyhow::Result;
use clap::{Parser, Subcommand};
use reasonkit_web::mcp::McpServer;
use reasonkit_web::{BrowserController, ContentExtractor, LinkExtractor, MetadataExtractor};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// ReasonKit Web - Web Sensing & Browser Automation Layer
#[derive(Parser)]
#[command(name = "reasonkit-web")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the MCP server (default)
    Serve,

    /// Test browser automation
    Test {
        /// URL to test with
        #[arg(default_value = "https://example.com")]
        url: String,
    },

    /// Extract content from a URL
    Extract {
        /// URL to extract from
        url: String,

        /// Output format (text, markdown, html)
        #[arg(short, long, default_value = "markdown")]
        format: String,
    },

    /// Take a screenshot
    Screenshot {
        /// URL to capture
        url: String,

        /// Output file path
        #[arg(short, long, default_value = "screenshot.png")]
        output: String,

        /// Full page capture
        #[arg(long)]
        full_page: bool,
    },

    /// List available tools
    Tools,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging - always write to stderr so MCP can use stdout
    let log_level = match cli.log_level.to_lowercase().as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => {
            if cli.verbose {
                Level::DEBUG
            } else {
                Level::INFO
            }
        }
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("ReasonKit Web v{} starting...", env!("CARGO_PKG_VERSION"));

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => run_mcp_server().await,
        Commands::Test { url } => run_test(&url).await,
        Commands::Extract { url, format } => run_extract(&url, &format).await,
        Commands::Screenshot {
            url,
            output,
            full_page,
        } => run_screenshot(&url, &output, full_page).await,
        Commands::Tools => run_list_tools().await,
    }
}

async fn run_mcp_server() -> Result<()> {
    info!("Starting MCP server...");
    let server = McpServer::new();
    server.run().await?;
    Ok(())
}

async fn run_test(url: &str) -> Result<()> {
    info!("Testing browser automation with: {}", url);

    let controller = BrowserController::new().await?;
    let page = controller.navigate(url).await?;

    info!("Navigated successfully to: {}", page.url().await);

    // Extract metadata
    let metadata = MetadataExtractor::extract(&page).await?;
    println!("\n=== Page Metadata ===");
    println!("Title: {:?}", metadata.title);
    println!("Description: {:?}", metadata.description);
    println!("Language: {:?}", metadata.language);

    // Extract content
    let content = ContentExtractor::extract_main_content(&page).await?;
    println!("\n=== Content Summary ===");
    println!("Word count: {}", content.word_count);
    println!("From main element: {}", content.from_main);
    println!(
        "Preview: {}...",
        &content.text.chars().take(200).collect::<String>()
    );

    // Extract links
    let links = LinkExtractor::extract_all(&page).await?;
    println!("\n=== Links ===");
    println!("Total links: {}", links.len());

    let external: Vec<_> = links
        .iter()
        .filter(|l| l.link_type == reasonkit_web::extraction::LinkType::External)
        .collect();
    println!("External links: {}", external.len());

    controller.close().await?;
    info!("Test complete!");

    Ok(())
}

async fn run_extract(url: &str, format: &str) -> Result<()> {
    info!("Extracting content from: {}", url);

    let controller = BrowserController::new().await?;
    let page = controller.navigate(url).await?;

    let content = ContentExtractor::extract_main_content(&page).await?;

    let output = match format {
        "text" => content.text,
        "html" => content.html,
        _ => content.markdown.unwrap_or(content.text),
    };

    println!("{}", output);

    controller.close().await?;
    Ok(())
}

async fn run_screenshot(url: &str, output: &str, full_page: bool) -> Result<()> {
    use reasonkit_web::browser::{CaptureOptions, PageCapture};

    info!("Taking screenshot of: {}", url);

    let controller = BrowserController::new().await?;
    let page = controller.navigate(url).await?;

    let options = CaptureOptions {
        full_page,
        ..Default::default()
    };

    let result = PageCapture::capture(&page, &options).await?;

    std::fs::write(output, &result.data)?;
    info!("Screenshot saved to: {} ({} bytes)", output, result.size);

    controller.close().await?;
    Ok(())
}

async fn run_list_tools() -> Result<()> {
    use reasonkit_web::mcp::AVAILABLE_TOOLS;

    println!("Available MCP Tools:\n");
    for tool in AVAILABLE_TOOLS {
        println!("  - {}", tool);
    }
    println!();

    // Also show detailed tool info
    let registry = reasonkit_web::mcp::ToolRegistry::new();
    let definitions = registry.definitions();

    println!("Tool Details:\n");
    for def in definitions {
        println!("{}:", def.name);
        println!("  Description: {}", def.description);
        println!(
            "  Schema: {}",
            serde_json::to_string_pretty(&def.input_schema)?
        );
        println!();
    }

    Ok(())
}
