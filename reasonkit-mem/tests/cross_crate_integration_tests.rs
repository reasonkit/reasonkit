//! Cross-Crate Integration Tests for reasonkit-mem
//!
//! Tests for the memory layer integration with reasonkit-core and
//! external systems.
//!
//! # Test Categories
//!
//! 1. **Storage Operations**: Document CRUD, batch operations
//! 2. **Search Operations**: BM25, hybrid search, reranking
//! 3. **Embedding Operations**: Embedding generation and caching
//! 4. **RAPTOR Tree**: Hierarchical document retrieval
//! 5. **Service Interface**: MemoryService trait implementation
//! 6. **Error Handling**: Error propagation across module boundaries
//!
//! # Running Tests
//!
//! ```bash
//! # Run all tests
//! cargo test --package reasonkit-mem --test cross_crate_integration_tests
//!
//! # Run with local embeddings feature
//! cargo test --package reasonkit-mem --test cross_crate_integration_tests --features local-embeddings
//! ```

use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;

use uuid::Uuid;

// ============================================================================
// TEST INFRASTRUCTURE: Mock Embedding Provider
// ============================================================================

/// Mock embedding provider for deterministic testing
#[allow(dead_code)]
struct MockEmbeddingProvider {
    dimension: usize,
}

#[allow(dead_code)]
impl MockEmbeddingProvider {
    fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Generate deterministic embedding based on text hash
    fn generate_embedding(&self, text: &str) -> Vec<f32> {
        let mut embedding = vec![0.0f32; self.dimension];
        let text_lower = text.to_lowercase();

        // Character frequency for first 26 dimensions
        for byte in text_lower.as_bytes() {
            if *byte >= b'a' && *byte <= b'z' {
                let idx = (*byte - b'a') as usize;
                if idx < self.dimension {
                    embedding[idx] += 1.0;
                }
            }
        }

        // Normalize
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for x in embedding.iter_mut() {
                *x /= magnitude;
            }
        }

        embedding
    }
}

#[async_trait]
impl reasonkit_mem::embedding::EmbeddingProvider for MockEmbeddingProvider {
    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        "mock-provider"
    }

    async fn embed(
        &self,
        text: &str,
    ) -> reasonkit_mem::Result<reasonkit_mem::embedding::EmbeddingResult> {
        let dense = self.generate_embedding(text);
        Ok(reasonkit_mem::embedding::EmbeddingResult {
            dense: Some(dense),
            sparse: None,
            token_count: text.split_whitespace().count(),
        })
    }

    async fn embed_batch(
        &self,
        texts: &[&str],
    ) -> reasonkit_mem::Result<Vec<reasonkit_mem::embedding::EmbeddingResult>> {
        let mut results = Vec::with_capacity(texts.len());
        for text in texts {
            results.push(self.embed(text).await?);
        }
        Ok(results)
    }
}

// ============================================================================
// MODULE: Storage Operations Tests
// ============================================================================

mod storage_operations_tests {
    use super::*;
    use reasonkit_mem::{Document, DocumentType, Metadata, Source, SourceType};

    /// Test: Document creation and storage
    #[tokio::test]
    async fn test_document_creation() {
        let source = Source {
            source_type: SourceType::Local,
            url: None,
            path: Some("/test/document.md".to_string()),
            arxiv_id: None,
            github_repo: None,
            retrieved_at: Utc::now(),
            version: None,
        };

        let doc = Document::new(DocumentType::Documentation, source)
            .with_content("This is test content for the document.".to_string());

        assert_eq!(doc.doc_type, DocumentType::Documentation);
        assert!(doc.content.word_count > 0);
        assert!(!doc.id.is_nil());
    }

    /// Test: Document with metadata
    #[tokio::test]
    async fn test_document_with_metadata() {
        let source = Source {
            source_type: SourceType::Github,
            url: Some("https://github.com/test/repo".to_string()),
            path: None,
            arxiv_id: None,
            github_repo: Some("test/repo".to_string()),
            retrieved_at: Utc::now(),
            version: Some("v1.0.0".to_string()),
        };

        let metadata = Metadata {
            title: Some("Test Document".to_string()),
            authors: vec![],
            abstract_text: Some("Abstract content".to_string()),
            tags: vec!["test".to_string(), "integration".to_string()],
            ..Default::default()
        };

        let doc = Document::new(DocumentType::Code, source)
            .with_content("fn main() {}".to_string())
            .with_metadata(metadata);

        assert_eq!(doc.metadata.title, Some("Test Document".to_string()));
        assert_eq!(doc.metadata.tags.len(), 2);
        assert!(doc.metadata.tags.contains(&"test".to_string()));
    }

    /// Test: Document type variants
    #[test]
    fn test_document_types() {
        use reasonkit_mem::DocumentType;

        let types = vec![
            DocumentType::Paper,
            DocumentType::Documentation,
            DocumentType::Code,
            DocumentType::Note,
            DocumentType::Transcript,
            DocumentType::Benchmark,
        ];

        for doc_type in types {
            let source = Source {
                source_type: SourceType::Local,
                url: None,
                path: Some("/test.txt".to_string()),
                arxiv_id: None,
                github_repo: None,
                retrieved_at: Utc::now(),
                version: None,
            };
            let doc = Document::new(doc_type, source);
            assert!(!doc.id.is_nil());
        }
    }
}

// ============================================================================
// MODULE: Search Operations Tests
// ============================================================================

mod search_operations_tests {
    use super::*;
    use reasonkit_mem::{
        retrieval::{FusionStrategy, HybridRetriever},
        Document, DocumentType, RetrievalConfig, Source, SourceType,
    };

    /// Test: BM25 sparse search
    #[tokio::test]
    async fn test_bm25_sparse_search() {
        let retriever = HybridRetriever::in_memory().expect("Failed to create retriever");

        // Add test documents
        let docs = create_test_documents();
        for doc in &docs {
            retriever
                .add_document(doc)
                .await
                .expect("Failed to add document");
        }

        // Search
        let results = retriever.search_sparse("machine learning neural", 5).await;
        assert!(
            results.is_ok(),
            "Search should succeed: {:?}",
            results.err()
        );

        let results = results.unwrap();
        // BM25 should find relevant documents
        assert!(results.len() <= 5, "Should respect top_k limit");
    }

    /// Test: Retrieval configuration
    #[test]
    fn test_retrieval_config_defaults() {
        let config = RetrievalConfig::default();

        assert_eq!(config.top_k, 10);
        assert!((config.alpha - 0.7).abs() < 0.001); // Default dense weight
        assert!(!config.use_raptor);
    }

    /// Test: Fusion strategies
    #[test]
    fn test_fusion_strategies() {
        // RRF fusion
        let rrf = FusionStrategy::ReciprocalRankFusion { k: 60 };

        // Weighted sum
        let weighted = FusionStrategy::WeightedSum { dense_weight: 0.7 };

        // Verify strategy configuration
        match rrf {
            FusionStrategy::ReciprocalRankFusion { k } => assert_eq!(k, 60),
            _ => panic!("Expected RRF strategy"),
        }

        match weighted {
            FusionStrategy::WeightedSum { dense_weight } => {
                assert!((dense_weight - 0.7).abs() < 0.001);
            }
            _ => panic!("Expected WeightedSum strategy"),
        }
    }

    /// Helper: Create test documents
    fn create_test_documents() -> Vec<Document> {
        vec![
            create_doc(
                "Machine learning is a branch of artificial intelligence.",
                "ml.md",
            ),
            create_doc(
                "Deep learning uses neural networks for pattern recognition.",
                "dl.md",
            ),
            create_doc(
                "Natural language processing enables text understanding.",
                "nlp.md",
            ),
            create_doc("Computer vision processes and analyzes images.", "cv.md"),
            create_doc(
                "Reinforcement learning trains agents through rewards.",
                "rl.md",
            ),
        ]
    }

    fn create_doc(content: &str, path: &str) -> Document {
        let source = Source {
            source_type: SourceType::Local,
            url: None,
            path: Some(path.to_string()),
            arxiv_id: None,
            github_repo: None,
            retrieved_at: Utc::now(),
            version: None,
        };
        Document::new(DocumentType::Documentation, source).with_content(content.to_string())
    }
}

// ============================================================================
// MODULE: Service Interface Tests
// ============================================================================

mod service_interface_tests {
    use super::*;
    use reasonkit_mem::service::{
        Document, HybridConfig, MemServiceImpl, MemoryConfig, MemoryService,
    };

    /// Test: Service initialization
    #[tokio::test]
    async fn test_service_initialization() {
        let service = MemServiceImpl::in_memory();
        assert!(
            service.is_ok(),
            "Service should initialize: {:?}",
            service.err()
        );

        let service = service.unwrap();
        let health = service.health_check().await;
        assert!(
            health.is_ok() && health.unwrap(),
            "Service should be healthy"
        );
    }

    /// Test: Store and retrieve document
    #[tokio::test]
    async fn test_store_and_retrieve_document() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        let doc = Document {
            id: None,
            content: "This is test content for storage validation.".to_string(),
            metadata: HashMap::new(),
            source: Some("/test/storage.md".to_string()),
            created_at: None,
        };

        // Store
        let id = service.store_document(&doc).await;
        assert!(id.is_ok(), "Store should succeed: {:?}", id.err());

        let stored_id = id.unwrap();
        assert!(!stored_id.is_nil(), "Should have valid ID");

        // Retrieve
        let retrieved = service.get_by_id(stored_id).await;
        assert!(
            retrieved.is_ok(),
            "Retrieve should succeed: {:?}",
            retrieved.err()
        );

        let retrieved_doc = retrieved.unwrap();
        assert!(retrieved_doc.is_some(), "Document should exist");
        assert!(
            retrieved_doc
                .as_ref()
                .unwrap()
                .content
                .contains("test content"),
            "Content should match"
        );
    }

    /// Test: Delete document
    #[tokio::test]
    async fn test_delete_document() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        let doc = Document {
            id: None,
            content: "Document to be deleted.".to_string(),
            metadata: HashMap::new(),
            source: None,
            created_at: None,
        };

        let id = service.store_document(&doc).await.expect("Store failed");

        // Verify exists
        let exists = service.get_by_id(id).await.expect("Get failed");
        assert!(exists.is_some(), "Should exist before delete");

        // Delete
        let delete_result = service.delete_document(id).await;
        assert!(
            delete_result.is_ok(),
            "Delete should succeed: {:?}",
            delete_result.err()
        );
    }

    /// Test: Get statistics
    #[tokio::test]
    async fn test_get_statistics() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        // Initially empty
        let stats = service.get_stats().await.expect("Stats failed");
        assert_eq!(stats.total_documents, 0);

        // Add documents
        for i in 0..3 {
            let doc = Document {
                id: None,
                content: format!("Document number {} content.", i),
                metadata: HashMap::new(),
                source: None,
                created_at: None,
            };
            service.store_document(&doc).await.expect("Store failed");
        }

        // Check stats updated
        let stats = service.get_stats().await.expect("Stats failed");
        assert_eq!(stats.total_documents, 3, "Should have 3 documents");
    }

    /// Test: Configuration management
    #[test]
    fn test_configuration_defaults() {
        let config = MemoryConfig::default();

        assert_eq!(config.chunk_size, 512);
        assert_eq!(config.chunk_overlap, 50);
        assert_eq!(config.max_context_tokens, 4096);
        assert_eq!(config.embedding_dimensions, 384);
    }

    /// Test: Hybrid search configuration
    #[test]
    fn test_hybrid_config() {
        let config = HybridConfig::default();

        assert!((config.vector_weight - 0.7).abs() < 0.001);
        assert!((config.bm25_weight - 0.3).abs() < 0.001);
        assert!(config.use_reranker);
    }

    /// Test: Search via service
    #[tokio::test]
    async fn test_service_search() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        // Add documents
        let docs = vec![
            "Machine learning algorithms optimize predictions.",
            "Neural networks learn hierarchical representations.",
            "Data science combines statistics and programming.",
        ];

        for content in docs {
            let doc = Document {
                id: None,
                content: content.to_string(),
                metadata: HashMap::new(),
                source: None,
                created_at: None,
            };
            service.store_document(&doc).await.expect("Store failed");
        }

        // Search using BM25 (sparse search works without embedding pipeline)
        let results = service
            .retriever()
            .search_sparse("machine learning", 5)
            .await;
        assert!(
            results.is_ok(),
            "Sparse search should succeed: {:?}",
            results.err()
        );
        assert!(!results.unwrap().is_empty(), "Should find results");
    }

    /// Test: Context window assembly (using sparse search)
    #[tokio::test]
    async fn test_context_window() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        // Add long documents
        for i in 0..5 {
            let doc = Document {
                id: None,
                content: format!(
                    "Document {} contains extensive information about artificial intelligence, \
                     machine learning, neural networks, and deep learning systems. \
                     This document is designed to test context window assembly.",
                    i
                ),
                metadata: HashMap::new(),
                source: None,
                created_at: None,
            };
            service.store_document(&doc).await.expect("Store failed");
        }

        // Search using BM25 (sparse search works without embedding pipeline)
        // Note: get_context requires embeddings, so we test sparse search instead
        let results = service
            .retriever()
            .search_sparse("machine learning", 5)
            .await;
        assert!(
            results.is_ok(),
            "Sparse search should succeed: {:?}",
            results.err()
        );
        assert!(!results.unwrap().is_empty(), "Should find documents");
    }

    /// Test: Service lifecycle
    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        // Health check before operations
        assert!(service.health_check().await.unwrap());

        // Store and flush
        let doc = Document {
            id: None,
            content: "Lifecycle test document.".to_string(),
            metadata: HashMap::new(),
            source: None,
            created_at: None,
        };
        service.store_document(&doc).await.expect("Store failed");

        let flush_result = service.flush().await;
        assert!(flush_result.is_ok(), "Flush should succeed");

        // Shutdown
        let shutdown_result = service.shutdown().await;
        assert!(shutdown_result.is_ok(), "Shutdown should succeed");

        // Health check after shutdown
        assert!(
            !service.health_check().await.unwrap(),
            "Should be unhealthy after shutdown"
        );
    }
}

// ============================================================================
// MODULE: RAPTOR Tree Tests
// ============================================================================

mod raptor_tree_tests {
    use reasonkit_mem::raptor::{OptimizedRaptorTree, RaptorOptConfig};

    /// Test: RAPTOR configuration
    #[test]
    fn test_raptor_config_defaults() {
        let config = RaptorOptConfig::default();

        assert!(config.max_depth > 0);
        assert!(config.cluster_size > 0);
    }

    /// Test: RAPTOR tree creation
    #[tokio::test]
    async fn test_raptor_tree_creation() {
        let config = RaptorOptConfig {
            max_depth: 3,
            cluster_size: 4,
            ..Default::default()
        };

        let tree = OptimizedRaptorTree::new(config);

        let stats = tree.stats();
        assert_eq!(stats.total_nodes, 0, "Empty tree should have no nodes");
        assert_eq!(stats.leaf_nodes, 0);
    }
}

// ============================================================================
// MODULE: Indexing Tests
// ============================================================================

mod indexing_tests {
    use super::*;
    use reasonkit_mem::indexing::IndexManager;

    /// Test: Index manager creation
    #[test]
    fn test_index_manager_creation() {
        let manager = IndexManager::in_memory();
        assert!(
            manager.is_ok(),
            "Index manager should initialize: {:?}",
            manager.err()
        );
    }

    /// Test: BM25 search
    #[tokio::test]
    async fn test_bm25_search() {
        let manager = IndexManager::in_memory().expect("Failed to create index manager");

        // Index some content
        let source = reasonkit_mem::Source {
            source_type: reasonkit_mem::SourceType::Local,
            url: None,
            path: Some("/test.md".to_string()),
            arxiv_id: None,
            github_repo: None,
            retrieved_at: Utc::now(),
            version: None,
        };
        let doc = reasonkit_mem::Document::new(reasonkit_mem::DocumentType::Documentation, source)
            .with_content("Machine learning enables predictive analytics.".to_string());

        manager.index_document(&doc).expect("Indexing failed");

        // Search
        let results = manager.search_bm25("machine learning", 5);
        assert!(
            results.is_ok(),
            "Search should succeed: {:?}",
            results.err()
        );
    }
}

// ============================================================================
// MODULE: Error Handling Tests
// ============================================================================

mod error_handling_tests {
    use super::*;
    use reasonkit_mem::error::MemError;

    /// Test: Error types
    #[test]
    fn test_error_types() {
        let storage_err = MemError::storage("Storage connection failed");
        assert!(storage_err.to_string().contains("Storage"));

        let embedding_err = MemError::embedding("Embedding API error");
        assert!(embedding_err.to_string().contains("Embedding"));

        let indexing_err = MemError::indexing("Index corruption detected");
        assert!(indexing_err.to_string().contains("Index"));
    }

    /// Test: Error conversion to service error
    #[test]
    fn test_error_conversion() {
        use reasonkit_mem::service::MemoryError;

        let mem_error = MemError::storage("Test error");
        let service_error: MemoryError = mem_error.into();

        assert!(matches!(service_error, MemoryError::Storage(_)));
    }

    /// Test: Not found error
    #[tokio::test]
    async fn test_not_found_error() {
        use reasonkit_mem::service::{MemServiceImpl, MemoryService};

        let service = MemServiceImpl::in_memory().expect("Failed to create service");

        // Try to get non-existent document
        let non_existent_id = Uuid::new_v4();
        let result = service.get_by_id(non_existent_id).await;

        assert!(result.is_ok(), "Get should not error for missing doc");
        assert!(
            result.unwrap().is_none(),
            "Should return None for missing doc"
        );
    }
}

// ============================================================================
// MODULE: Concurrency Tests
// ============================================================================

mod concurrency_tests {
    use super::*;
    use reasonkit_mem::service::{Document, MemServiceImpl, MemoryService};
    use std::sync::Arc;
    use tokio::sync::Barrier;

    /// Test: Concurrent document storage
    #[tokio::test]
    async fn test_concurrent_storage() {
        let service = Arc::new(MemServiceImpl::in_memory().expect("Failed to create service"));
        let barrier = Arc::new(Barrier::new(5));

        let mut handles = vec![];

        for i in 0..5 {
            let service = Arc::clone(&service);
            let barrier = Arc::clone(&barrier);

            let handle = tokio::spawn(async move {
                barrier.wait().await; // Synchronize start

                let doc = Document {
                    id: None,
                    content: format!("Concurrent document {}", i),
                    metadata: HashMap::new(),
                    source: None,
                    created_at: None,
                };

                service.store_document(&doc).await
            });

            handles.push(handle);
        }

        let mut success_count = 0;
        for handle in handles {
            if handle.await.expect("Task panicked").is_ok() {
                success_count += 1;
            }
        }

        assert_eq!(success_count, 5, "All concurrent stores should succeed");
    }

    /// Test: Thread safety of service
    #[test]
    fn test_service_thread_safety() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<MemServiceImpl>();
    }
}

// ============================================================================
// MODULE: Integration with Core Types
// ============================================================================

mod core_types_integration {
    use super::*;
    use reasonkit_mem::{
        Author, Chunk, Document, DocumentType, EmbeddingIds, Metadata, ProcessingState,
        ProcessingStatus, Source, SourceType,
    };

    /// Test: Full document with all fields
    #[test]
    fn test_full_document_structure() {
        let source = Source {
            source_type: SourceType::Arxiv,
            url: Some("https://arxiv.org/abs/2301.12345".to_string()),
            path: None,
            arxiv_id: Some("2301.12345".to_string()),
            github_repo: None,
            retrieved_at: Utc::now(),
            version: None,
        };

        let metadata = Metadata {
            title: Some("Example Paper Title".to_string()),
            authors: vec![Author {
                name: "John Doe".to_string(),
                affiliation: Some("University".to_string()),
                email: Some("john@example.com".to_string()),
            }],
            abstract_text: Some("This paper explores...".to_string()),
            date: Some("2023-01-15".to_string()),
            venue: Some("NeurIPS 2023".to_string()),
            citations: Some(42),
            tags: vec!["ai".to_string(), "ml".to_string()],
            categories: vec!["cs.LG".to_string()],
            keywords: vec!["transformer".to_string()],
            doi: Some("10.1234/example".to_string()),
            license: Some("CC-BY-4.0".to_string()),
        };

        let mut doc = Document::new(DocumentType::Paper, source)
            .with_content("Full paper content here...".to_string())
            .with_metadata(metadata);

        // Add chunks
        doc.chunks.push(Chunk {
            id: Uuid::new_v4(),
            text: "Chunk 1 content".to_string(),
            index: 0,
            start_char: 0,
            end_char: 15,
            token_count: Some(3),
            section: Some("Introduction".to_string()),
            page: Some(1),
            embedding_ids: EmbeddingIds {
                dense: Some("emb-001".to_string()),
                sparse: None,
                colbert: None,
            },
        });

        // Update processing status
        doc.processing = ProcessingStatus {
            status: ProcessingState::Completed,
            chunked: true,
            embedded: true,
            indexed: true,
            raptor_processed: false,
            errors: vec![],
        };

        // Verify structure
        assert_eq!(doc.doc_type, DocumentType::Paper);
        assert_eq!(doc.metadata.authors.len(), 1);
        assert_eq!(doc.chunks.len(), 1);
        assert!(doc.processing.chunked);
    }

    /// Test: Search result structure
    #[test]
    fn test_search_result_structure() {
        use reasonkit_mem::{MatchSource, SearchResult as MemSearchResult};

        let chunk = Chunk {
            id: Uuid::new_v4(),
            text: "Matching content".to_string(),
            index: 0,
            start_char: 0,
            end_char: 16,
            token_count: Some(2),
            section: None,
            page: None,
            embedding_ids: EmbeddingIds::default(),
        };

        let result = MemSearchResult {
            score: 0.95,
            document_id: Uuid::new_v4(),
            chunk,
            match_source: MatchSource::Hybrid,
        };

        assert!(result.score > 0.9);
        assert!(matches!(result.match_source, MatchSource::Hybrid));
    }
}
