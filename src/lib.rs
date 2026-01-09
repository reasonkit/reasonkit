//! # ReasonKit — The Reasoning Engine
//!
//! Complete ReasonKit suite providing auditable reasoning for production AI.
//!
//! This crate is a **meta-crate** that combines all ReasonKit components:
//!
//! | Component | Crate | Purpose |
//! |-----------|-------|---------|
//! | **Core** | [`reasonkit-core`] | Reasoning engine with ThinkTools |
//! | **Memory** | [`reasonkit-mem`] | Vector storage, hybrid search, RAPTOR trees |
//! | **Web** | [`reasonkit-web`] | Browser automation, MCP sidecar |
//!
//! ## Quick Start
//!
//! ### Installation
//!
//! ```bash
//! # Install the complete suite (recommended)
//! cargo install reasonkit
//!
//! # Or install with specific features
//! cargo install reasonkit --no-default-features --features core
//! ```
//!
//! ### CLI Usage
//!
//! ```bash
//! # Run structured reasoning
//! reasonkit think --profile balanced "Should we migrate to microservices?"
//!
//! # Memory operations
//! reasonkit mem search "machine learning fundamentals"
//!
//! # Start MCP server
//! reasonkit serve
//! ```
//!
//! ### Library Usage
//!
//! ```rust,ignore
//! use reasonkit::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Access reasoning engine
//!     let executor = reasonkit::reasoning::thinktool::ProtocolExecutor::new()?;
//!     let result = executor.execute(
//!         "gigathink",
//!         reasonkit::reasoning::thinktool::ProtocolInput::query("Your question")
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! | Feature | Description | Default |
//! |---------|-------------|---------|
//! | `full` | All components (core + mem + web) | Yes |
//! | `core` | Reasoning engine only | No |
//! | `mem` | Memory layer only | No |
//! | `web` | Web/browser automation only | No |
//! | `python` | Python bindings via PyO3 | No |
//! | `arf` | Autonomous Reasoning Framework | No |
//!
//! ## Architecture
//!
//! ```text
//! reasonkit (meta-crate)
//! │
//! ├── reasonkit-core    ─── The Reasoning Engine
//! │   ├── ThinkTools (GigaThink, LaserLogic, BedRock, ProofGuard, BrutalHonesty)
//! │   ├── Protocol Executor
//! │   ├── LLM Client (18+ providers)
//! │   └── MCP Server
//! │
//! ├── reasonkit-mem     ─── Memory Infrastructure
//! │   ├── Vector Storage (Qdrant)
//! │   ├── Sparse Index (Tantivy BM25)
//! │   ├── Hybrid Retrieval
//! │   └── RAPTOR Trees
//! │
//! └── reasonkit-web     ─── Web Sensing Layer
//!     ├── Browser Controller (CDP)
//!     ├── Content Extraction
//!     └── MCP Sidecar
//! ```
//!
//! ## Philosophy
//!
//! **"Designed, Not Dreamed"** — Structure beats raw intelligence.
//!
//! ReasonKit transforms ad-hoc LLM prompting into auditable, reproducible
//! reasoning chains through structured protocols.
//!
//! ## Links
//!
//! - Website: <https://reasonkit.sh>
//! - Documentation: <https://docs.rs/reasonkit>
//! - Repository: <https://github.com/reasonkit/reasonkit>
//!
//! [`reasonkit-core`]: https://docs.rs/reasonkit-core
//! [`reasonkit-mem`]: https://docs.rs/reasonkit-mem
//! [`reasonkit-web`]: https://docs.rs/reasonkit-web

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

// =============================================================================
// RE-EXPORTS
// =============================================================================

/// Re-export of `reasonkit-core` — The Reasoning Engine.
///
/// Provides ThinkTools, protocol execution, LLM client, and MCP server.
///
/// Note: Named `reasoning` to avoid conflict with Rust's `core` crate.
#[cfg(feature = "core")]
#[cfg_attr(docsrs, doc(cfg(feature = "core")))]
pub mod reasoning {
    pub use reasonkit_core::*;
}

/// Re-export of `reasonkit-mem` — Memory Infrastructure.
///
/// Provides vector storage, hybrid search, RAPTOR trees, and knowledge base.
#[cfg(feature = "mem")]
#[cfg_attr(docsrs, doc(cfg(feature = "mem")))]
pub mod mem {
    pub use reasonkit_mem::*;
}

/// Re-export of `reasonkit-web` — Web Sensing Layer.
///
/// Provides browser automation, content extraction, and MCP sidecar.
#[cfg(feature = "web")]
#[cfg_attr(docsrs, doc(cfg(feature = "web")))]
pub mod web {
    pub use reasonkit_web::*;
}

// =============================================================================
// PRELUDE
// =============================================================================

/// Convenient imports for common usage patterns.
///
/// ```rust,ignore
/// use reasonkit::prelude::*;
/// ```
pub mod prelude {
    // Core types (when available)
    #[cfg(feature = "core")]
    pub use reasonkit_core::{
        error::{Error as CoreError, Result as CoreResult},
        thinktool::{ProtocolExecutor, ProtocolInput, ProtocolOutput},
    };

    // Memory types (when available)
    #[cfg(feature = "mem")]
    pub use reasonkit_mem::{Document, DocumentType, MemError, MemResult, Source, SourceType};

    // Web types (when available)
    #[cfg(feature = "web")]
    pub use reasonkit_web::{BrowserController, ContentExtractor, Error as WebError, McpServer};
}

// =============================================================================
// VERSION INFO
// =============================================================================

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get version information for all enabled components.
pub fn version_info() -> VersionInfo {
    VersionInfo {
        reasonkit: VERSION.to_string(),
        #[cfg(feature = "core")]
        core: Some(reasonkit_core::VERSION.to_string()),
        #[cfg(not(feature = "core"))]
        core: None,
        #[cfg(feature = "mem")]
        mem: None, // reasonkit-mem doesn't export VERSION yet
        #[cfg(not(feature = "mem"))]
        mem: None,
        #[cfg(feature = "web")]
        web: Some(reasonkit_web::VERSION.to_string()),
        #[cfg(not(feature = "web"))]
        web: None,
    }
}

/// Version information for all components.
#[derive(Debug, Clone, serde::Serialize)]
pub struct VersionInfo {
    /// Meta-crate version.
    pub reasonkit: String,
    /// Core component version (if enabled).
    pub core: Option<String>,
    /// Memory component version (if enabled).
    pub mem: Option<String>,
    /// Web component version (if enabled).
    pub web: Option<String>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(!info.reasonkit.is_empty());
    }

    #[test]
    fn test_constants() {
        assert_eq!(NAME, "reasonkit");
        assert!(!VERSION.is_empty());
    }
}
