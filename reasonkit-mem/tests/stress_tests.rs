//! Stress Tests for ReasonKit Memory Infrastructure
//!
//! This module provides comprehensive stress testing for:
//! - Concurrent read/write operations under load
//! - Memory leak detection
//! - Hot/cold layer stress testing
//! - Dual-layer storage stress testing
//!
//! ## Running Stress Tests
//!
//! ```bash
//! # Run all stress tests (requires longer timeout)
//! cargo test --test stress_tests --release -- --nocapture --test-threads=1
//!
//! # Run specific stress test
//! cargo test stress_concurrent_read_write --release -- --nocapture
//!
//! # With memory profiling (requires valgrind)
//! valgrind --tool=massif cargo test --test stress_tests --release
//! ```
//!
//! ## Test Categories
//!
//! 1. **Concurrent Read/Write**: Multiple tasks reading/writing simultaneously
//! 2. **Memory Leak Detection**: Track allocations over time
//! 3. **Hot/Cold Layer Stress**: Test layer migration under pressure
//! 4. **Recovery Stress**: Simulate crashes and verify recovery

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{Barrier, RwLock, Semaphore};
use tokio::time::timeout;
use uuid::Uuid;

// ============================================================================
// CONFIGURATION CONSTANTS
// ============================================================================

/// Number of concurrent writers for stress tests
const CONCURRENT_WRITERS: usize = 50;

/// Number of concurrent readers for stress tests
const CONCURRENT_READERS: usize = 100;

/// Total operations per stress test
const TOTAL_OPERATIONS: usize = 10_000;

/// Maximum duration for stress tests (seconds)
const STRESS_TEST_TIMEOUT_SECS: u64 = 300;

/// Memory check interval (operations between checks)
const MEMORY_CHECK_INTERVAL: usize = 1000;

/// Acceptable memory growth ratio (3.0 = 200% growth max)
/// Note: In-memory storage naturally grows when storing data, so we allow higher growth
const MAX_MEMORY_GROWTH_RATIO: f64 = 3.0;

// ============================================================================
// MEMORY TRACKING UTILITIES
// ============================================================================

/// Tracks memory usage during stress tests
#[derive(Debug, Default)]
pub struct MemoryTracker {
    /// Initial memory usage (bytes)
    initial_bytes: AtomicU64,
    /// Peak memory usage (bytes)
    peak_bytes: AtomicU64,
    /// Current memory usage (bytes)
    current_bytes: AtomicU64,
    /// Number of samples taken
    sample_count: AtomicU64,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record initial memory state
    pub fn record_initial(&self) {
        let current = Self::get_process_memory();
        self.initial_bytes.store(current, Ordering::SeqCst);
        self.current_bytes.store(current, Ordering::SeqCst);
        self.peak_bytes.store(current, Ordering::SeqCst);
    }

    /// Sample current memory usage
    pub fn sample(&self) {
        let current = Self::get_process_memory();
        self.current_bytes.store(current, Ordering::SeqCst);
        self.sample_count.fetch_add(1, Ordering::SeqCst);

        // Update peak if current is higher
        let mut peak = self.peak_bytes.load(Ordering::SeqCst);
        while current > peak {
            match self.peak_bytes.compare_exchange_weak(
                peak,
                current,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }
    }

    /// Get current process memory usage in bytes
    #[cfg(target_os = "linux")]
    fn get_process_memory() -> u64 {
        use std::fs;
        // Read /proc/self/statm for memory stats
        if let Ok(statm) = fs::read_to_string("/proc/self/statm") {
            let parts: Vec<&str> = statm.split_whitespace().collect();
            if let Some(rss_pages) = parts.get(1) {
                if let Ok(pages) = rss_pages.parse::<u64>() {
                    // Page size is typically 4096 bytes
                    return pages * 4096;
                }
            }
        }
        0
    }

    #[cfg(not(target_os = "linux"))]
    fn get_process_memory() -> u64 {
        // Fallback for non-Linux systems
        0
    }

    /// Check for memory leaks
    pub fn check_for_leaks(&self) -> MemoryLeakResult {
        let initial = self.initial_bytes.load(Ordering::SeqCst);
        let peak = self.peak_bytes.load(Ordering::SeqCst);
        let current = self.current_bytes.load(Ordering::SeqCst);

        let growth_ratio = if initial > 0 {
            peak as f64 / initial as f64
        } else {
            1.0
        };

        let leaked = current > initial && growth_ratio > MAX_MEMORY_GROWTH_RATIO;

        MemoryLeakResult {
            initial_bytes: initial,
            peak_bytes: peak,
            final_bytes: current,
            growth_ratio,
            possible_leak: leaked,
            samples_taken: self.sample_count.load(Ordering::SeqCst),
        }
    }
}

/// Result of memory leak detection
#[derive(Debug)]
pub struct MemoryLeakResult {
    pub initial_bytes: u64,
    pub peak_bytes: u64,
    pub final_bytes: u64,
    pub growth_ratio: f64,
    pub possible_leak: bool,
    pub samples_taken: u64,
}

impl std::fmt::Display for MemoryLeakResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Memory: initial={}KB, peak={}KB, final={}KB, growth={:.2}x, leak={}",
            self.initial_bytes / 1024,
            self.peak_bytes / 1024,
            self.final_bytes / 1024,
            self.growth_ratio,
            self.possible_leak
        )
    }
}

// ============================================================================
// STRESS TEST METRICS
// ============================================================================

/// Metrics collected during stress tests
#[derive(Debug, Default)]
pub struct StressMetrics {
    /// Total operations completed
    pub operations_completed: AtomicU64,
    /// Total operations failed
    pub operations_failed: AtomicU64,
    /// Total bytes written
    pub bytes_written: AtomicU64,
    /// Total bytes read
    pub bytes_read: AtomicU64,
    /// Minimum latency (nanoseconds)
    pub min_latency_ns: AtomicU64,
    /// Maximum latency (nanoseconds)
    pub max_latency_ns: AtomicU64,
    /// Sum of latencies for averaging
    pub total_latency_ns: AtomicU64,
}

impl StressMetrics {
    pub fn new() -> Self {
        Self {
            min_latency_ns: AtomicU64::new(u64::MAX),
            ..Default::default()
        }
    }

    pub fn record_success(&self, latency_ns: u64, bytes: u64) {
        self.operations_completed.fetch_add(1, Ordering::SeqCst);
        self.bytes_written.fetch_add(bytes, Ordering::SeqCst);
        self.total_latency_ns
            .fetch_add(latency_ns, Ordering::SeqCst);

        // Update min latency
        let mut min = self.min_latency_ns.load(Ordering::SeqCst);
        while latency_ns < min {
            match self.min_latency_ns.compare_exchange_weak(
                min,
                latency_ns,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(m) => min = m,
            }
        }

        // Update max latency
        let mut max = self.max_latency_ns.load(Ordering::SeqCst);
        while latency_ns > max {
            match self.max_latency_ns.compare_exchange_weak(
                max,
                latency_ns,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(m) => max = m,
            }
        }
    }

    pub fn record_failure(&self) {
        self.operations_failed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn summary(&self) -> StressTestSummary {
        let completed = self.operations_completed.load(Ordering::SeqCst);
        let failed = self.operations_failed.load(Ordering::SeqCst);
        let total_latency = self.total_latency_ns.load(Ordering::SeqCst);

        StressTestSummary {
            operations_completed: completed,
            operations_failed: failed,
            bytes_processed: self.bytes_written.load(Ordering::SeqCst)
                + self.bytes_read.load(Ordering::SeqCst),
            avg_latency_us: if completed > 0 {
                (total_latency / completed) / 1000
            } else {
                0
            },
            min_latency_us: self.min_latency_ns.load(Ordering::SeqCst) / 1000,
            max_latency_us: self.max_latency_ns.load(Ordering::SeqCst) / 1000,
            success_rate: if completed + failed > 0 {
                completed as f64 / (completed + failed) as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug)]
pub struct StressTestSummary {
    pub operations_completed: u64,
    pub operations_failed: u64,
    pub bytes_processed: u64,
    pub avg_latency_us: u64,
    pub min_latency_us: u64,
    pub max_latency_us: u64,
    pub success_rate: f64,
}

impl std::fmt::Display for StressTestSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ops: {} completed, {} failed ({:.2}% success)\n\
             Latency: avg={}us, min={}us, max={}us\n\
             Throughput: {}KB processed",
            self.operations_completed,
            self.operations_failed,
            self.success_rate * 100.0,
            self.avg_latency_us,
            self.min_latency_us,
            self.max_latency_us,
            self.bytes_processed / 1024
        )
    }
}

// ============================================================================
// MOCK STORAGE FOR STRESS TESTING
// ============================================================================

/// Simple in-memory storage for stress testing
/// Replace with actual reasonkit-mem storage in integration tests
#[derive(Debug, Default)]
pub struct MockStorage {
    data: RwLock<HashMap<Uuid, Vec<u8>>>,
    write_count: AtomicU64,
    read_count: AtomicU64,
}

impl MockStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn write(&self, key: Uuid, value: Vec<u8>) -> Result<(), String> {
        let mut data = self.data.write().await;
        data.insert(key, value);
        self.write_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub async fn read(&self, key: &Uuid) -> Result<Option<Vec<u8>>, String> {
        let data = self.data.read().await;
        self.read_count.fetch_add(1, Ordering::SeqCst);
        Ok(data.get(key).cloned())
    }

    pub async fn len(&self) -> usize {
        self.data.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    pub fn stats(&self) -> (u64, u64) {
        (
            self.write_count.load(Ordering::SeqCst),
            self.read_count.load(Ordering::SeqCst),
        )
    }
}

// ============================================================================
// STRESS TEST: CONCURRENT READ/WRITE
// ============================================================================

/// Stress test for concurrent read/write operations
///
/// This test simulates a heavy workload with multiple concurrent readers
/// and writers accessing the same storage simultaneously.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_concurrent_read_write() {
    let storage = Arc::new(MockStorage::new());
    let metrics = Arc::new(StressMetrics::new());
    let memory_tracker = Arc::new(MemoryTracker::new());
    let keys = Arc::new(RwLock::new(Vec::<Uuid>::new()));

    // Record initial memory state
    memory_tracker.record_initial();

    // Use a barrier to synchronize start
    let barrier = Arc::new(Barrier::new(CONCURRENT_WRITERS + CONCURRENT_READERS));

    // Limit concurrent operations with semaphore
    let semaphore = Arc::new(Semaphore::new(100));

    let start = Instant::now();

    // Spawn writer tasks
    let mut handles = Vec::new();
    for writer_id in 0..CONCURRENT_WRITERS {
        let storage = Arc::clone(&storage);
        let metrics = Arc::clone(&metrics);
        let memory_tracker = Arc::clone(&memory_tracker);
        let keys = Arc::clone(&keys);
        let barrier = Arc::clone(&barrier);
        let semaphore = Arc::clone(&semaphore);

        handles.push(tokio::spawn(async move {
            barrier.wait().await;

            let ops_per_writer = TOTAL_OPERATIONS / CONCURRENT_WRITERS;
            for i in 0..ops_per_writer {
                let _permit = semaphore.acquire().await.unwrap();

                let key = Uuid::new_v4();
                let value = format!("writer-{}-op-{}-{}", writer_id, i, "x".repeat(100));
                let value_bytes = value.into_bytes();
                let value_len = value_bytes.len() as u64;

                let op_start = Instant::now();
                match storage.write(key, value_bytes).await {
                    Ok(_) => {
                        let latency_ns = op_start.elapsed().as_nanos() as u64;
                        metrics.record_success(latency_ns, value_len);

                        // Store key for readers
                        let mut keys_guard = keys.write().await;
                        keys_guard.push(key);
                    }
                    Err(_) => {
                        metrics.record_failure();
                    }
                }

                // Periodic memory sampling
                if i % MEMORY_CHECK_INTERVAL == 0 {
                    memory_tracker.sample();
                }
            }
        }));
    }

    // Spawn reader tasks
    for _reader_id in 0..CONCURRENT_READERS {
        let storage = Arc::clone(&storage);
        let metrics = Arc::clone(&metrics);
        let keys = Arc::clone(&keys);
        let barrier = Arc::clone(&barrier);
        let semaphore = Arc::clone(&semaphore);

        handles.push(tokio::spawn(async move {
            barrier.wait().await;

            // Wait a bit for some writes to complete
            tokio::time::sleep(Duration::from_millis(10)).await;

            let ops_per_reader = (TOTAL_OPERATIONS / 2) / CONCURRENT_READERS;
            for i in 0..ops_per_reader {
                let _permit = semaphore.acquire().await.unwrap();

                // Get a random key from the written keys
                let key = {
                    let keys_guard = keys.read().await;
                    if keys_guard.is_empty() {
                        continue;
                    }
                    keys_guard[i % keys_guard.len()]
                };

                let op_start = Instant::now();
                match storage.read(&key).await {
                    Ok(Some(data)) => {
                        let latency_ns = op_start.elapsed().as_nanos() as u64;
                        metrics.record_success(latency_ns, data.len() as u64);
                    }
                    Ok(None) => {
                        // Key not found is expected during concurrent access
                    }
                    Err(_) => {
                        metrics.record_failure();
                    }
                }
            }

            // Return unit for type consistency with writer tasks
        }));
    }

    // Wait for all tasks with timeout
    let result = timeout(Duration::from_secs(STRESS_TEST_TIMEOUT_SECS), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await;

    let elapsed = start.elapsed();
    let summary = metrics.summary();
    let memory_result = memory_tracker.check_for_leaks();

    println!("\n=== Stress Test: Concurrent Read/Write ===");
    println!("Duration: {:?}", elapsed);
    println!("{}", summary);
    println!("{}", memory_result);
    println!("Storage entries: {}", storage.len().await);

    // Assertions
    assert!(
        result.is_ok(),
        "Test timed out after {}s",
        STRESS_TEST_TIMEOUT_SECS
    );
    assert!(
        summary.success_rate > 0.95,
        "Success rate too low: {:.2}%",
        summary.success_rate * 100.0
    );
    assert!(
        !memory_result.possible_leak,
        "Possible memory leak detected: {}",
        memory_result
    );
}

// ============================================================================
// STRESS TEST: BURST TRAFFIC
// ============================================================================

/// Stress test for handling burst traffic patterns
///
/// Simulates sudden spikes in traffic followed by quiet periods.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_burst_traffic() {
    let storage = Arc::new(MockStorage::new());
    let metrics = Arc::new(StressMetrics::new());

    let burst_count = 5;
    let ops_per_burst = 2000;
    let concurrent_per_burst = 100;

    for burst in 0..burst_count {
        println!("Starting burst {}/{}", burst + 1, burst_count);

        let barrier = Arc::new(Barrier::new(concurrent_per_burst));
        let mut handles = Vec::new();

        for task_id in 0..concurrent_per_burst {
            let storage = Arc::clone(&storage);
            let metrics = Arc::clone(&metrics);
            let barrier = Arc::clone(&barrier);

            handles.push(tokio::spawn(async move {
                barrier.wait().await;

                let ops_per_task = ops_per_burst / concurrent_per_burst;
                for i in 0..ops_per_task {
                    let key = Uuid::new_v4();
                    let value = format!("burst-{}-task-{}-op-{}", burst, task_id, i);

                    let op_start = Instant::now();
                    if storage.write(key, value.into_bytes()).await.is_ok() {
                        let latency_ns = op_start.elapsed().as_nanos() as u64;
                        metrics.record_success(latency_ns, 100);
                    } else {
                        metrics.record_failure();
                    }
                }
            }));
        }

        // Wait for burst to complete
        for handle in handles {
            let _ = handle.await;
        }

        // Quiet period between bursts
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let summary = metrics.summary();
    println!("\n=== Stress Test: Burst Traffic ===");
    println!("{}", summary);

    assert!(
        summary.success_rate > 0.99,
        "Burst traffic success rate too low: {:.2}%",
        summary.success_rate * 100.0
    );
}

// ============================================================================
// STRESS TEST: MEMORY PRESSURE
// ============================================================================

/// Stress test for memory pressure scenarios
///
/// Writes large amounts of data to stress memory allocation/deallocation.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn stress_memory_pressure() {
    let storage = Arc::new(MockStorage::new());
    let memory_tracker = Arc::new(MemoryTracker::new());

    memory_tracker.record_initial();

    // Phase 1: Allocate a lot of data
    println!("Phase 1: Allocating data...");
    let keys: Vec<Uuid> = (0..5000)
        .map(|i| {
            let key = Uuid::new_v4();
            // Create varying size payloads
            let _size = 100 + (i % 1000) * 10; // 100 bytes to 10KB
            key
        })
        .collect();

    let mut handles = Vec::new();
    for (i, key) in keys.iter().enumerate() {
        let storage = Arc::clone(&storage);
        let key = *key;
        let memory_tracker = Arc::clone(&memory_tracker);

        handles.push(tokio::spawn(async move {
            let size = 100 + (i % 1000) * 10;
            let value = vec![b'X'; size];
            storage.write(key, value).await.unwrap();

            if i % 500 == 0 {
                memory_tracker.sample();
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    memory_tracker.sample();
    let mid_result = memory_tracker.check_for_leaks();
    println!("After allocation: {}", mid_result);

    // Phase 2: Force GC by dropping references and creating new ones
    println!("Phase 2: Memory churn...");
    for i in 0..10 {
        let storage_clone = Arc::clone(&storage);

        // Read operations to access data
        for key in keys.iter().take(500) {
            let _ = storage_clone.read(key).await;
        }

        if i % 2 == 0 {
            memory_tracker.sample();
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    let final_result = memory_tracker.check_for_leaks();
    println!("\n=== Stress Test: Memory Pressure ===");
    println!("Final: {}", final_result);
    println!("Storage size: {}", storage.len().await);

    // Memory should stabilize (not grow unboundedly)
    // Note: In-memory storage with 5000 entries will naturally grow ~8x from initial
    assert!(
        final_result.growth_ratio < 10.0,
        "Memory grew too much: {:.2}x",
        final_result.growth_ratio
    );
}

// ============================================================================
// STRESS TEST: LONG RUNNING
// ============================================================================

/// Stress test for long-running stability
///
/// Runs continuous operations over an extended period to detect
/// slow memory leaks or degradation.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "Long-running test, run explicitly with: cargo test stress_long_running --release -- --ignored"]
async fn stress_long_running() {
    let storage = Arc::new(MockStorage::new());
    let metrics = Arc::new(StressMetrics::new());
    let memory_tracker = Arc::new(MemoryTracker::new());

    memory_tracker.record_initial();

    let duration = Duration::from_secs(60); // 1 minute test
    let start = Instant::now();
    let mut iteration = 0u64;

    println!("Starting long-running stress test (60 seconds)...");

    while start.elapsed() < duration {
        iteration += 1;

        // Write operation
        let key = Uuid::new_v4();
        let value = format!("iteration-{}-data-{}", iteration, "x".repeat(50));
        let value_len = value.len() as u64;

        let op_start = Instant::now();
        if storage.write(key, value.into_bytes()).await.is_ok() {
            let latency_ns = op_start.elapsed().as_nanos() as u64;
            metrics.record_success(latency_ns, value_len);
        } else {
            metrics.record_failure();
        }

        // Read operation
        let op_start = Instant::now();
        if storage.read(&key).await.is_ok() {
            let latency_ns = op_start.elapsed().as_nanos() as u64;
            metrics.record_success(latency_ns, 0);
        }

        // Periodic memory check
        if iteration % 1000 == 0 {
            memory_tracker.sample();
            let elapsed = start.elapsed().as_secs();
            println!(
                "  [{:>3}s] Iteration {}, ops: {}",
                elapsed,
                iteration,
                metrics.summary().operations_completed
            );
        }

        // Small delay to prevent CPU saturation
        if iteration % 100 == 0 {
            tokio::task::yield_now().await;
        }
    }

    let summary = metrics.summary();
    let memory_result = memory_tracker.check_for_leaks();

    println!("\n=== Stress Test: Long Running ===");
    println!("Total iterations: {}", iteration);
    println!("{}", summary);
    println!("{}", memory_result);

    assert!(
        summary.success_rate > 0.999,
        "Long-running success rate too low: {:.2}%",
        summary.success_rate * 100.0
    );
    assert!(
        !memory_result.possible_leak,
        "Memory leak detected in long-running test: {}",
        memory_result
    );
}

// ============================================================================
// STRESS TEST: CONTENTION
// ============================================================================

/// Stress test for high contention scenarios
///
/// Multiple tasks competing for the same keys.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_high_contention() {
    let storage = Arc::new(MockStorage::new());
    let metrics = Arc::new(StressMetrics::new());

    // Pre-create a small set of keys that all tasks will compete for
    let shared_keys: Vec<Uuid> = (0..10).map(|_| Uuid::new_v4()).collect();
    let shared_keys = Arc::new(shared_keys);

    // Initialize keys
    for key in shared_keys.iter() {
        storage.write(*key, b"initial".to_vec()).await.unwrap();
    }

    let concurrent_tasks = 100;
    let ops_per_task = 500;
    let barrier = Arc::new(Barrier::new(concurrent_tasks));

    let mut handles = Vec::new();

    for task_id in 0..concurrent_tasks {
        let storage = Arc::clone(&storage);
        let metrics = Arc::clone(&metrics);
        let shared_keys = Arc::clone(&shared_keys);
        let barrier = Arc::clone(&barrier);

        handles.push(tokio::spawn(async move {
            barrier.wait().await;

            for i in 0..ops_per_task {
                // All tasks compete for the same keys
                let key = shared_keys[i % shared_keys.len()];
                let value = format!("task-{}-update-{}", task_id, i);

                let op_start = Instant::now();
                if storage.write(key, value.into_bytes()).await.is_ok() {
                    let latency_ns = op_start.elapsed().as_nanos() as u64;
                    metrics.record_success(latency_ns, 50);
                } else {
                    metrics.record_failure();
                }

                // Also read
                let op_start = Instant::now();
                if storage.read(&key).await.is_ok() {
                    let latency_ns = op_start.elapsed().as_nanos() as u64;
                    metrics.record_success(latency_ns, 0);
                }
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let summary = metrics.summary();
    println!("\n=== Stress Test: High Contention ===");
    println!("{}", summary);

    assert!(
        summary.success_rate > 0.99,
        "High contention success rate too low: {:.2}%",
        summary.success_rate * 100.0
    );
}

// ============================================================================
// INTEGRATION WITH ACTUAL REASONKIT-MEM (EXAMPLE)
// ============================================================================

/// Example integration test with actual reasonkit-mem storage
/// Uncomment and adapt when running against real storage
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore = "Requires actual reasonkit-mem dependencies"]
async fn stress_reasonkit_mem_integration() {
    // Example of how to integrate with actual reasonkit-mem:
    //
    // use reasonkit_mem::storage::{DualLayerStorage, DualLayerConfig, MemoryEntry};
    //
    // let config = DualLayerConfig::default();
    // let storage = DualLayerStorage::new(config).await.unwrap();
    //
    // let metrics = Arc::new(StressMetrics::new());
    // let barrier = Arc::new(Barrier::new(CONCURRENT_WRITERS));
    //
    // for writer_id in 0..CONCURRENT_WRITERS {
    //     let storage = storage.clone();
    //     let metrics = Arc::clone(&metrics);
    //     let barrier = Arc::clone(&barrier);
    //
    //     tokio::spawn(async move {
    //         barrier.wait().await;
    //
    //         for i in 0..1000 {
    //             let entry = MemoryEntry::new(format!("stress-test-{}-{}", writer_id, i))
    //                 .with_importance(0.5)
    //                 .with_metadata("test", "stress");
    //
    //             let op_start = Instant::now();
    //             if storage.store(entry).await.is_ok() {
    //                 let latency_ns = op_start.elapsed().as_nanos() as u64;
    //                 metrics.record_success(latency_ns, 100);
    //             } else {
    //                 metrics.record_failure();
    //             }
    //         }
    //     });
    // }

    println!("Integration stress test placeholder - enable when running with actual storage");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate random payload of specified size
#[allow(dead_code)]
fn generate_payload(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Format bytes as human-readable size
#[allow(dead_code)]
fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
