// ReasonKit — Unified CLI
//
// Complete command-line interface for the ReasonKit suite.
//
// This binary provides a unified entry point to all ReasonKit components:
// - `reasonkit think` — Reasoning engine (via reasonkit-core)
// - `reasonkit mem` — Memory operations (via reasonkit-mem)
// - `reasonkit web` — Web/browser automation (via reasonkit-web)
// - `reasonkit serve` — Start MCP server

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// =============================================================================
// CLI STRUCTURE
// =============================================================================

#[derive(Parser)]
#[command(name = "reasonkit")]
#[command(author = "ReasonKit Team <team@reasonkit.sh>")]
#[command(version)]
#[command(about = "The Reasoning Engine — Auditable Reasoning for Production AI")]
#[command(long_about = r#"
ReasonKit — Complete Suite for Structured AI Reasoning

This unified CLI provides access to all ReasonKit components:

  REASONING (reasonkit-core):
    reasonkit think      Execute ThinkTools protocols
    reasonkit verify     Triangulate claims with 3+ sources

  MEMORY (reasonkit-mem):
    reasonkit mem        Memory and knowledge base operations
    reasonkit rag        Retrieval-augmented generation

  WEB (reasonkit-web):
    reasonkit web        Browser automation and capture
    reasonkit serve      Start MCP server

EXAMPLES:
    # Quick reasoning analysis
    reasonkit think --profile quick "Is this a good investment?"

    # Deep analysis with full protocol chain
    reasonkit think --profile paranoid "Validate this architecture"

    # Search knowledge base
    reasonkit mem search "machine learning fundamentals"

    # Start unified MCP server
    reasonkit serve

WEBSITE: https://reasonkit.sh
DOCS:    https://docs.rs/reasonkit
"#)]
struct Cli {
    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text", global = true)]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    // =========================================================================
    // CORE: Reasoning Engine
    // =========================================================================
    /// Execute structured reasoning protocols (ThinkTools)
    #[cfg(feature = "core")]
    #[command(alias = "t")]
    Think {
        /// The query or question to analyze
        query: String,

        /// Protocol to execute (gigathink, laserlogic, bedrock, proofguard, brutalhonesty)
        #[arg(short, long)]
        protocol: Option<String>,

        /// Profile to execute (quick, balanced, deep, paranoid)
        #[arg(long, default_value = "balanced")]
        profile: String,

        /// LLM provider
        #[arg(long, default_value = "anthropic")]
        provider: String,

        /// LLM model to use
        #[arg(short, long)]
        model: Option<String>,

        /// Use mock LLM (for testing)
        #[arg(long)]
        mock: bool,

        /// List available protocols and profiles
        #[arg(long)]
        list: bool,
    },

    /// Triangulate and verify claims with 3+ sources
    #[cfg(feature = "core")]
    #[command(alias = "v")]
    Verify {
        /// Claim or statement to verify
        claim: String,

        /// Minimum number of sources required
        #[arg(short, long, default_value = "3")]
        sources: usize,
    },

    // =========================================================================
    // MEMORY: Knowledge Base Operations
    // =========================================================================
    /// Memory and knowledge base operations
    #[cfg(feature = "mem")]
    #[command(alias = "m")]
    Mem {
        #[command(subcommand)]
        action: MemAction,
    },

    /// Retrieval-augmented generation queries
    #[cfg(feature = "mem")]
    Rag {
        /// Query for RAG retrieval
        query: String,

        /// Number of results to retrieve
        #[arg(short = 'k', long, default_value = "5")]
        top_k: usize,

        /// Use hybrid search (BM25 + vector)
        #[arg(long)]
        hybrid: bool,
    },

    // =========================================================================
    // WEB: Browser Automation
    // =========================================================================
    /// Browser automation and web capture
    #[cfg(feature = "web")]
    #[command(alias = "w")]
    Web {
        #[command(subcommand)]
        action: WebAction,
    },

    // =========================================================================
    // SERVERS
    // =========================================================================
    /// Start the ReasonKit MCP server
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Server mode (core, web, full)
        #[arg(long, default_value = "full")]
        mode: ServerMode,
    },

    // =========================================================================
    // UTILITIES
    // =========================================================================
    /// Show version information for all components
    Version,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[cfg(feature = "mem")]
#[derive(Subcommand)]
enum MemAction {
    /// Search the knowledge base
    Search {
        /// Search query
        query: String,
        /// Number of results
        #[arg(short = 'k', long, default_value = "10")]
        top_k: usize,
    },
    /// Ingest documents into the knowledge base
    Ingest {
        /// Path to document or directory
        path: std::path::PathBuf,
        /// Process recursively
        #[arg(short, long)]
        recursive: bool,
    },
    /// Show knowledge base statistics
    Stats,
}

#[cfg(feature = "web")]
#[derive(Subcommand)]
enum WebAction {
    /// Navigate to a URL and capture content
    Capture {
        /// URL to capture
        url: String,
        /// Save screenshot
        #[arg(long)]
        screenshot: bool,
    },
    /// Extract content from a URL
    Extract {
        /// URL to extract from
        url: String,
        /// Extraction mode (text, links, metadata)
        #[arg(long, default_value = "text")]
        mode: String,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ServerMode {
    Core,
    Web,
    Full,
}

// =============================================================================
// LOGGING SETUP
// =============================================================================

fn setup_logging(verbosity: u8) {
    let level = match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(verbosity >= 3)
        .with_line_number(verbosity >= 3)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}

// =============================================================================
// COMMAND HANDLERS
// =============================================================================

#[cfg(feature = "core")]
#[allow(clippy::too_many_arguments)]
async fn handle_think(
    query: String,
    protocol: Option<String>,
    profile: String,
    _provider: String,
    _model: Option<String>,
    mock: bool,
    list: bool,
    format: OutputFormat,
) -> anyhow::Result<()> {
    use reasonkit_core::thinktool::{ProtocolExecutor, ProtocolInput};

    let executor = if mock {
        ProtocolExecutor::mock()?
    } else {
        ProtocolExecutor::new()?
    };

    if list {
        println!("Available Protocols:");
        for p in executor.list_protocols() {
            println!("  - {}", p);
        }
        println!("\nAvailable Profiles:");
        for p in executor.list_profiles() {
            println!("  - {}", p);
        }
        return Ok(());
    }

    let input = ProtocolInput::query(&query);

    let output = if let Some(proto) = protocol {
        executor.execute(&proto, input).await?
    } else {
        executor.execute_profile(&profile, input).await?
    };

    match format {
        OutputFormat::Text => {
            println!("Thinking Process:");
            for step in &output.steps {
                println!("\n[{}] {}", step.step_id, step.as_text().unwrap_or(""));
            }
            println!("\nConfidence: {:.2}", output.confidence);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

#[cfg(feature = "core")]
async fn handle_verify(claim: String, sources: usize) -> anyhow::Result<()> {
    println!("Verifying claim: {}", claim);
    println!("Minimum sources required: {}", sources);
    println!("\n[Not yet implemented - use rk-core verify]");
    Ok(())
}

#[cfg(feature = "mem")]
async fn handle_mem(action: MemAction, format: OutputFormat) -> anyhow::Result<()> {
    match action {
        MemAction::Search { query, top_k } => {
            println!("Searching knowledge base: {} (top {})", query, top_k);
            println!("\n[Not yet implemented - use rk-mem search]");
        }
        MemAction::Ingest { path, recursive } => {
            println!("Ingesting: {:?} (recursive: {})", path, recursive);
            println!("\n[Not yet implemented - use rk-mem ingest]");
        }
        MemAction::Stats => {
            println!("Knowledge Base Statistics:");
            println!("\n[Not yet implemented - use rk-mem stats]");
        }
    }
    let _ = format; // Suppress warning
    Ok(())
}

#[cfg(feature = "mem")]
async fn handle_rag(query: String, top_k: usize, hybrid: bool) -> anyhow::Result<()> {
    println!(
        "RAG Query: {} (top_k: {}, hybrid: {})",
        query, top_k, hybrid
    );
    println!("\n[Not yet implemented - use rk-core rag]");
    Ok(())
}

#[cfg(feature = "web")]
async fn handle_web(action: WebAction) -> anyhow::Result<()> {
    match action {
        WebAction::Capture { url, screenshot } => {
            println!("Capturing URL: {} (screenshot: {})", url, screenshot);
            println!("\n[Not yet implemented - use rk-web capture]");
        }
        WebAction::Extract { url, mode } => {
            println!("Extracting from URL: {} (mode: {})", url, mode);
            println!("\n[Not yet implemented - use rk-web extract]");
        }
    }
    Ok(())
}

async fn handle_serve(host: String, port: u16, mode: ServerMode) -> anyhow::Result<()> {
    info!("Starting ReasonKit server on {}:{}", host, port);
    info!("Mode: {:?}", mode);

    match mode {
        #[cfg(feature = "core")]
        ServerMode::Core | ServerMode::Full => {
            info!("Starting Core MCP server...");
            reasonkit_core::mcp::server::run_server().await?;
        }
        #[cfg(not(feature = "core"))]
        ServerMode::Core => {
            anyhow::bail!("Core feature not enabled. Rebuild with --features core");
        }
        #[cfg(feature = "web")]
        ServerMode::Web => {
            info!("Starting Web MCP server...");
            // reasonkit_web::McpServer::run().await?;
            println!("[Web server not yet integrated]");
        }
        #[cfg(not(feature = "web"))]
        ServerMode::Web => {
            anyhow::bail!("Web feature not enabled. Rebuild with --features web");
        }
        #[cfg(not(feature = "core"))]
        ServerMode::Full => {
            anyhow::bail!("Full mode requires core feature. Rebuild with --features full");
        }
    }

    Ok(())
}

fn handle_version(format: OutputFormat) -> anyhow::Result<()> {
    let info = reasonkit::version_info();

    match format {
        OutputFormat::Text => {
            println!("ReasonKit Suite v{}", info.reasonkit);
            println!();
            println!("Components:");
            if let Some(v) = &info.core {
                println!("  reasonkit-core: v{}", v);
            } else {
                println!("  reasonkit-core: not enabled");
            }
            if let Some(v) = &info.mem {
                println!("  reasonkit-mem:  v{}", v);
            } else {
                println!("  reasonkit-mem:  not enabled");
            }
            if let Some(v) = &info.web {
                println!("  reasonkit-web:  v{}", v);
            } else {
                println!("  reasonkit-web:  not enabled");
            }
            println!();
            println!("Website: https://reasonkit.sh");
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
    }

    Ok(())
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    info!("ReasonKit v{}", reasonkit::VERSION);

    match cli.command {
        #[cfg(feature = "core")]
        Commands::Think {
            query,
            protocol,
            profile,
            provider,
            model,
            mock,
            list,
        } => {
            handle_think(
                query, protocol, profile, provider, model, mock, list, cli.format,
            )
            .await?;
        }

        #[cfg(feature = "core")]
        Commands::Verify { claim, sources } => {
            handle_verify(claim, sources).await?;
        }

        #[cfg(feature = "mem")]
        Commands::Mem { action } => {
            handle_mem(action, cli.format).await?;
        }

        #[cfg(feature = "mem")]
        Commands::Rag {
            query,
            top_k,
            hybrid,
        } => {
            handle_rag(query, top_k, hybrid).await?;
        }

        #[cfg(feature = "web")]
        Commands::Web { action } => {
            handle_web(action).await?;
        }

        Commands::Serve { host, port, mode } => {
            handle_serve(host, port, mode).await?;
        }

        Commands::Version => {
            handle_version(cli.format)?;
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "reasonkit", &mut std::io::stdout());
        }
    }

    Ok(())
}
