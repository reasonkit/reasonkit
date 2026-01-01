# Changelog

All notable changes to ReasonKit Memory will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-01

### Added

- **Dual-Layer Memory Storage**
  - Hot memory layer (in-memory, fast access)
  - Cold memory layer (disk-backed, persistent)
  - Automatic hot-to-cold migration
  - Configurable sync intervals
- **Vector Search Capabilities**
  - Cosine similarity search
  - Embedding-based retrieval
  - Configurable similarity thresholds
  - Batch search operations
- **RAPTOR Tree Implementation**
  - Hierarchical clustering
  - Multi-level summarization
  - Efficient tree construction
  - Query optimization
- **Write-Ahead Log (WAL)**
  - Durability guarantees
  - Crash recovery support
  - Configurable sync modes
  - Segment management
- **Context Retrieval**
  - Semantic search combining hot and cold layers
  - Reciprocal Rank Fusion (RRF) for result merging
  - Recency weighting
  - Deduplication and filtering
- **Storage Backends**
  - Sled-based embedded storage
  - File-based storage option
  - Configurable storage paths
- **Memory Management**
  - TTL (Time-To-Live) support
  - Eviction policies
  - Capacity management
  - Statistics and monitoring

### Fixed

- All 37 build errors resolved
  - Fixed type mismatches in `retrieve_context`
  - Fixed stats field mismatches
  - Fixed MemoryEntry to ColdMemoryEntry conversion
  - Fixed `Send` trait issue in `service.rs`
  - Removed `?` operator from non-Result stats methods
  - Fixed outdated test assertions
- Configuration structure alignment
- Type system consistency
- Error handling improvements

### Changed

- Improved memory layer synchronization
- Enhanced error messages
- Better configuration defaults
- Optimized storage operations

### Documentation

- Comprehensive README
- Architecture documentation
- API documentation
- Usage examples

### Performance

- Optimized vector search
- Improved memory usage
- Faster hot-to-cold migration
- Reduced storage overhead

---

## [0.1.0] - 2024-12-XX

### Added

- Initial memory storage implementation
- Basic vector operations
- Simple retrieval mechanisms

---

## [Unreleased]

### Planned

- Additional storage backends
- Enhanced RAPTOR tree features
- Performance benchmarks
- More comprehensive examples
- Advanced query capabilities

---

[0.2.0]: https://github.com/reasonkit/reasonkit-mem/releases/tag/v0.2.0
[0.1.0]: https://github.com/reasonkit/reasonkit-mem/releases/tag/v0.1.0
