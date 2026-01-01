<div align="center">

# ReasonKit

**The Reasoning Engine — Auditable Reasoning for Production AI**

[![Crates.io](https://img.shields.io/crates/v/reasonkit.svg)](https://crates.io/crates/reasonkit)
[![Documentation](https://docs.rs/reasonkit/badge.svg)](https://docs.rs/reasonkit)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

[Website](https://reasonkit.sh) • [Documentation](https://docs.rs/reasonkit) • [Crates.io](https://crates.io/crates/reasonkit)

</div>

---

## Overview

ReasonKit transforms ad-hoc LLM prompting into structured, auditable reasoning chains.

**Philosophy:** *Designed, Not Dreamed* — Structure beats raw intelligence.

## Installation

```bash
# Install complete suite
cargo install reasonkit

# Or install individual components
cargo install reasonkit-core  # Reasoning engine
cargo install reasonkit-mem   # Memory layer
cargo install reasonkit-web   # Web sensing
```

## Quick Start

```bash
# Run structured reasoning
reasonkit think --profile balanced "Should we adopt microservices?"

# Search knowledge base
reasonkit mem search "machine learning fundamentals"

# Start MCP server
reasonkit serve
```

## Components

| Crate | Description |
|-------|-------------|
| [`reasonkit`](https://crates.io/crates/reasonkit) | Meta-crate — installs complete suite |
| [`reasonkit-core`](https://crates.io/crates/reasonkit-core) | The Reasoning Engine — ThinkTools, protocols, MCP |
| [`reasonkit-mem`](https://crates.io/crates/reasonkit-mem) | Memory layer — vector storage, hybrid search, RAPTOR |
| [`reasonkit-web`](https://crates.io/crates/reasonkit-web) | Web sensing — browser automation, content extraction |

## Architecture

```
reasonkit (meta-crate)
│
├── reasonkit-core    ─── The Reasoning Engine
│   ├── ThinkTools (GigaThink, LaserLogic, BedRock, ProofGuard, BrutalHonesty)
│   ├── Protocol Executor
│   ├── LLM Client (18+ providers)
│   └── MCP Server
│
├── reasonkit-mem     ─── Memory Infrastructure
│   ├── Vector Storage (Qdrant)
│   ├── Sparse Index (Tantivy BM25)
│   ├── Hybrid Retrieval
│   └── RAPTOR Trees
│
└── reasonkit-web     ─── Web Sensing Layer
    ├── Browser Controller (CDP)
    ├── Content Extraction
    └── MCP Sidecar
```

## License

Apache 2.0 — See [LICENSE](LICENSE) for details.

---

<div align="center">

**[reasonkit.sh](https://reasonkit.sh)** — Turn Prompts into Protocols

</div>
