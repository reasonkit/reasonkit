<div align="center">

# ReasonKit

**The Reasoning Engine — Auditable Reasoning for Production AI**

[![CI](https://img.shields.io/github/actions/workflow/status/reasonkit/reasonkit/ci.yml?branch=main&style=flat-square&logo=github&label=CI&color=06b6d4&logoColor=06b6d4)](https://github.com/reasonkit/reasonkit/actions/workflows/ci.yml)
[![Security](https://img.shields.io/github/actions/workflow/status/reasonkit/reasonkit/security.yml?branch=main&style=flat-square&logo=github&label=Security&color=10b981&logoColor=10b981)](https://github.com/reasonkit/reasonkit/actions/workflows/security.yml)
[![Crates.io](https://img.shields.io/crates/v/reasonkit?style=flat-square&logo=rust&color=10b981&logoColor=f9fafb)](https://crates.io/crates/reasonkit)
[![docs.rs](https://img.shields.io/docsrs/reasonkit?style=flat-square&logo=docs.rs&color=06b6d4&logoColor=f9fafb)](https://docs.rs/reasonkit)
[![Downloads](https://img.shields.io/crates/d/reasonkit?style=flat-square&color=ec4899&logo=rust&logoColor=f9fafb)](https://crates.io/crates/reasonkit)
[![License](https://img.shields.io/badge/license-Apache%202.0-a855f7?style=flat-square&labelColor=030508)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-f97316?style=flat-square&logo=rust&logoColor=f9fafb)](https://www.rust-lang.org/)

_Meta-crate providing unified installation for the complete ReasonKit suite_

[Documentation](https://docs.rs/reasonkit) | [Crates.io](https://crates.io/crates/reasonkit) | [Website](https://reasonkit.sh)

</div>

---

## Overview

ReasonKit transforms ad-hoc LLM prompting into structured, auditable reasoning chains.

**Philosophy:** _Designed, Not Dreamed_ — Structure beats raw intelligence.

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

| Crate                                                       | Description                                          |
| ----------------------------------------------------------- | ---------------------------------------------------- |
| [`reasonkit`](https://crates.io/crates/reasonkit)           | Meta-crate — installs complete suite                 |
| [`reasonkit-core`](https://crates.io/crates/reasonkit-core) | The Reasoning Engine — ThinkTools, protocols, MCP    |
| [`reasonkit-mem`](https://crates.io/crates/reasonkit-mem)   | Memory layer — vector storage, hybrid search, RAPTOR |
| [`reasonkit-web`](https://crates.io/crates/reasonkit-web)   | Web sensing — browser automation, content extraction |

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
