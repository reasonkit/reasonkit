//! # ReasonKit Memory - Quick Start Example
//!
//! This example demonstrates basic usage of ReasonKit Memory's dual-layer storage.
//!
//! Run with: `cargo run --example quick-start-mem --package reasonkit-mem`

use reasonkit_mem::storage::{DualLayerConfig, DualLayerMemory, MemoryEntry, MemoryLayer};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  ReasonKit Memory - Quick Start                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Create a memory storage instance with default configuration
    let config = DualLayerConfig::default();
    let storage = DualLayerMemory::new(config).await?;

    println!("✅ Memory storage initialized");
    println!();

    // Create a sample memory entry
    let entry = MemoryEntry {
        id: Uuid::new_v4(),
        content: "Rust is a systems programming language focused on safety and performance."
            .to_string(),
        embedding: Some(vec![0.1, 0.2, 0.3, 0.4, 0.5]), // Simplified embedding
        metadata: std::collections::HashMap::new(),
        importance: 0.8,
        access_count: 0,
        created_at: chrono::Utc::now(),
        last_accessed: chrono::Utc::now(),
        ttl_secs: None,
        layer: MemoryLayer::Hot,
        tags: vec!["programming".to_string(), "rust".to_string()],
    };

    println!("📝 Storing memory entry...");
    storage.store(entry.clone()).await?;
    println!("   ID: {}", entry.id);
    println!(
        "   Content: {}",
        &entry.content[..50.min(entry.content.len())]
    );
    println!();

    // Retrieve the entry
    println!("🔍 Retrieving memory entry...");
    let retrieved = storage.get(&entry.id).await?;

    if let Some(retrieved_entry) = retrieved {
        println!("   ✅ Found entry:");
        println!(
            "   Content: {}",
            &retrieved_entry.content[..50.min(retrieved_entry.content.len())]
        );
        println!("   Layer: {:?}", retrieved_entry.layer);
        println!("   Importance: {:.2}", retrieved_entry.importance);
    } else {
        println!("   ❌ Entry not found");
    }
    println!();

    // Note: Search functionality requires embedding pipeline setup
    // This is a simplified example - see full examples for complete usage
    println!("🔎 Search functionality available via context retrieval");
    println!("   (See examples/context_retrieval.rs for full example)");
    println!();

    // Get storage statistics
    println!("📊 Storage Statistics:");
    let stats = storage.stats().await?;
    println!("   Hot entries: {}", stats.hot_entry_count);
    println!("   Cold entries: {}", stats.cold_entry_count);
    println!("   Total entries: {}", stats.total_entries);
    println!();

    println!("✅ Quick start example completed!");
    println!();
    println!("💡 Next steps:");
    println!("   - Store more complex entries with embeddings");
    println!("   - Use context retrieval for semantic search");
    println!("   - Configure hot/cold layer sync intervals");
    println!();

    Ok(())
}
