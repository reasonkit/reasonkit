# ReasonKit

> **The Reasoning Engine — Auditable Reasoning for Production AI**

[![Crates.io](https://img.shields.io/crates/v/reasonkit.svg)](https://crates.io/crates/reasonkit)
[![Documentation](https://docs.rs/reasonkit/badge.svg)](https://docs.rs/reasonkit)
[![License](https://img.shields.io/crates/l/reasonkit.svg)](LICENSE)
[![CI](https://github.com/reasonkit/reasonkit/actions/workflows/ci.yml/badge.svg)](https://github.com/reasonkit/reasonkit/actions)

**ReasonKit** transforms ad-hoc LLM prompting into structured, auditable reasoning chains. This meta-crate provides a unified installation for the complete ReasonKit suite.

## One-Line Install

```bash
# Install complete ReasonKit suite
cargo install reasonkit

# Or use the universal installer
curl -fsSL https://reasonkit.sh/install | bash
```

## What's Included

| Component  | Crate                                                     | Purpose                                     |
| ---------- | --------------------------------------------------------- | ------------------------------------------- |
| **Core**   | [reasonkit-core](https://crates.io/crates/reasonkit-core) | Reasoning engine with ThinkTools            |
| **Memory** | [reasonkit-mem](https://crates.io/crates/reasonkit-mem)   | Vector storage, hybrid search, RAPTOR trees |
| **Web**    | [reasonkit-web](https://crates.io/crates/reasonkit-web)   | Browser automation, MCP sidecar             |

## Quick Start

### CLI Usage

```bash
# Run structured reasoning (ThinkTools)
reasonkit think --profile balanced "Should we migrate to microservices?"

# Quick 2-step analysis
reasonkit think --profile quick "Is this email phishing?"

# Maximum rigor (paranoid mode)
reasonkit think --profile paranoid "Validate this cryptographic implementation"

# Verify claims with triangulation
reasonkit verify "GPT-4 has 1.76 trillion parameters"

# Start MCP server for AI agent integration
reasonkit serve
```

### Library Usage

```rust
use reasonkit::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create reasoning executor
    let executor = reasonkit::core::thinktool::ProtocolExecutor::new()?;

    // Run GigaThink for multi-perspective analysis
    let result = executor.execute(
        "gigathink",
        reasonkit::core::thinktool::ProtocolInput::query("What factors drive startup success?")
    ).await?;

    println!("Confidence: {:.1}%", result.confidence * 100.0);
    for step in &result.steps {
        println!("- {}", step.as_text().unwrap_or_default());
    }

    Ok(())
}
```

## ThinkTools

Five core reasoning protocols:

| Tool              | Shortcut | Purpose                                          |
| ----------------- | -------- | ------------------------------------------------ |
| **GigaThink**     | `gt`     | Generate 10+ diverse perspectives                |
| **LaserLogic**    | `ll`     | Precision deductive reasoning, fallacy detection |
| **BedRock**       | `br`     | First principles decomposition                   |
| **ProofGuard**    | `pg`     | Multi-source verification (3+ sources)           |
| **BrutalHonesty** | `bh`     | Adversarial self-critique                        |

## Profiles

Pre-configured protocol chains:

| Profile    | ThinkTools         | Confidence | Use Case           |
| ---------- | ------------------ | ---------- | ------------------ |
| `quick`    | GT, LL             | 70%        | Fast analysis      |
| `balanced` | GT, LL, BR, PG     | 80%        | Standard decisions |
| `deep`     | All 5              | 85%        | Complex problems   |
| `paranoid` | All 5 + validation | 95%        | High-stakes        |

## Features

```toml
[dependencies]
# Full suite (default)
reasonkit = "0.1"

# Core reasoning only
reasonkit = { version = "0.1", default-features = false, features = ["core"] }

# Memory layer only
reasonkit = { version = "0.1", default-features = false, features = ["mem"] }

# Web automation only
reasonkit = { version = "0.1", default-features = false, features = ["web"] }
```

| Feature  | Description                 |
| -------- | --------------------------- |
| `full`   | All components (default)    |
| `core`   | Reasoning engine only       |
| `mem`    | Memory layer only           |
| `web`    | Web/browser automation only |
| `python` | Python bindings via PyO3    |

## LLM Providers

18+ providers supported out of the box:

- **Major Cloud**: Anthropic, OpenAI, Google Gemini, Vertex AI, Azure OpenAI, AWS Bedrock
- **Specialized**: xAI (Grok), Groq, Mistral, DeepSeek, Cohere, Perplexity
- **Aggregation**: OpenRouter (300+ models)

```bash
# Set your API key
export ANTHROPIC_API_KEY="sk-ant-..."

# Or use a different provider
reasonkit think --provider openai --model gpt-4o "Your question"
```

## Architecture

```
reasonkit (meta-crate)
│
├── reasonkit-core    ─── The Reasoning Engine
│   ├── ThinkTools
│   ├── Protocol Executor
│   ├── LLM Client
│   └── MCP Server
│
├── reasonkit-mem     ─── Memory Infrastructure
│   ├── Vector Storage (Qdrant)
│   ├── Sparse Index (Tantivy)
│   ├── Hybrid Retrieval
│   └── RAPTOR Trees
│
└── reasonkit-web     ─── Web Sensing Layer
    ├── Browser Controller
    ├── Content Extraction
    └── MCP Sidecar
```

## Philosophy

**"Designed, Not Dreamed"** — Structure beats raw intelligence.

ReasonKit imposes systematic reasoning protocols on LLM outputs, producing more reliable, verifiable, and explainable results.

## Documentation

- **Website**: https://reasonkit.sh
- **API Docs**: https://docs.rs/reasonkit
- **Core Docs**: https://docs.rs/reasonkit-core
- **Memory Docs**: https://docs.rs/reasonkit-mem
- **Web Docs**: https://docs.rs/reasonkit-web

## Individual Crates

If you only need specific functionality:

```bash
# Reasoning only
cargo install reasonkit-core

# Memory layer only
cargo install reasonkit-mem

# Web automation only
cargo install reasonkit-web
```

## License

Apache-2.0 — See [LICENSE](LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

**Turn Prompts into Protocols** | https://reasonkit.sh
