# Security Guide: Parallel Batch Processing
## Simple Image Converter - Security Specialist Guidelines

**Document Version:** 1.0  
**Created:** December 30, 2025  
**Author:** Security Specialist (Casey Morgan)  
**Status:** Draft - For Sprint 9 Implementation

---

## Executive Summary

This document provides security guidelines for implementing parallel batch processing in the Simple Image Converter GUI. These guidelines must be followed to ensure thread safety, resource limit enforcement, and protection against denial-of-service attacks.

**Critical Security Requirements:**
- ✅ Thread safety (no race conditions)
- ✅ Resource limits (max concurrent conversions)
- ✅ Memory limits per thread
- ✅ Deadlock prevention
- ✅ Path validation for all queue items
- ✅ Input sanitization

---

## Thread Safety Requirements

### 1. Shared State Protection

**REQUIRED:** All shared state must be protected with appropriate synchronization primitives.

```rust
// GOOD: Arc<Mutex<>> for shared mutable state
let queue: Arc<Mutex<BatchQueue>> = Arc::new(Mutex::new(BatchQueue::new()));

// GOOD: Arc<RwLock<>> for read-heavy shared state
let settings: Arc<RwLock<AppSettings>> = Arc::new(RwLock::new(AppSettings::default()));

// BAD: Unsynchronized shared state
let queue = BatchQueue::new(); // NOT THREAD-SAFE
```

### 2. Lock Ordering (Deadlock Prevention)

**CRITICAL:** Establish consistent lock ordering to prevent deadlocks.

```rust
// GOOD: Consistent lock ordering (queue first, then item)
fn update_item_status(
    queue: &Arc<Mutex<BatchQueue>>,
    item_id: Uuid,
    status: BatchItemStatus,
) -> Result<()> {
    let mut queue_guard = queue.lock().unwrap();
    // Find and update item
    if let Some(item) = queue_guard.items.iter_mut().find(|i| i.id == item_id) {
        item.status = status;
    }
    Ok(())
}

// BAD: Inconsistent lock ordering (can cause deadlock)
fn bad_update(queue1: &Arc<Mutex<BatchQueue>>, queue2: &Arc<Mutex<BatchQueue>>) {
    let _guard1 = queue1.lock().unwrap();
    let _guard2 = queue2.lock().unwrap(); // DEADLOCK RISK if another thread does reverse order
}
```

**Lock Ordering Rules:**
1. Always acquire locks in the same order across all threads
2. Minimize lock duration (acquire, use, release quickly)
3. Never hold multiple locks if avoidable
4. Use `try_lock()` with timeout for non-blocking operations

### 3. Atomic Operations

**USE ATOMICS** for simple counters and flags that don't require complex state.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

// GOOD: Atomic counter for progress tracking
let completed_count = Arc::new(AtomicUsize::new(0));

// Update atomically
completed_count.fetch_add(1, Ordering::Relaxed);

// BAD: Non-atomic counter (race condition)
let mut completed_count = 0; // NOT THREAD-SAFE
completed_count += 1;
```

**When to Use Atomics:**
- Simple counters (completed items, failed items)
- Boolean flags (processing state)
- Progress percentages (if simple)

**When NOT to Use Atomics:**
- Complex data structures (use Mutex/RwLock)
- Multiple related fields (use Mutex/RwLock)
- Operations requiring multiple steps

### 4. Thread-Safe Queue Updates

**REQUIRED:** All queue modifications must be thread-safe.

```rust
// GOOD: Thread-safe item addition
fn add_item_safe(queue: &Arc<Mutex<BatchQueue>>, item: BatchItem) {
    let mut queue_guard = queue.lock().unwrap();
    queue_guard.add_item(item);
    // Lock released automatically
}

// GOOD: Thread-safe item removal
fn remove_item_safe(queue: &Arc<Mutex<BatchQueue>>, item_id: Uuid) -> bool {
    let mut queue_guard = queue.lock().unwrap();
    queue_guard.remove_item(item_id)
}

// BAD: Direct access without lock
fn bad_add_item(queue: &BatchQueue, item: BatchItem) {
    queue.items.push(item); // NOT THREAD-SAFE
}
```

---

## Resource Limits

### 1. Maximum Concurrent Conversions

**REQUIRED:** Enforce a maximum number of concurrent conversions to prevent resource exhaustion.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_CONCURRENT_CONVERSIONS: usize = 4; // Default: CPU cores

struct ParallelProcessor {
    active_count: Arc<AtomicUsize>,
    max_concurrent: usize,
}

impl ParallelProcessor {
    fn can_start_new(&self) -> bool {
        self.active_count.load(Ordering::Relaxed) < self.max_concurrent
    }
    
    fn start_conversion(&self) -> Result<()> {
        let current = self.active_count.fetch_add(1, Ordering::Relaxed);
        if current >= self.max_concurrent {
            self.active_count.fetch_sub(1, Ordering::Relaxed);
            return Err(ConversionError::ResourceLimitExceeded(
                "Maximum concurrent conversions reached".to_string()
            ));
        }
        Ok(())
    }
    
    fn finish_conversion(&self) {
        self.active_count.fetch_sub(1, Ordering::Relaxed);
    }
}
```

**Configuration:**
- Default: Number of CPU cores (detected at runtime)
- Maximum: 16 (hard limit to prevent system overload)
- Minimum: 1 (always allow at least one conversion)

### 2. Memory Limits Per Thread

**REQUIRED:** Enforce memory limits per conversion thread to prevent memory exhaustion.

```rust
use common::limits::ResourceLimits;

// GOOD: Per-thread resource limits
fn process_item_with_limits(
    item: &BatchItem,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // Validate file size before reading
    let input_data = read_file_bytes_checked(&item.source_path, limits)?;
    
    // Validate image/mesh dimensions
    // ... conversion logic ...
    
    Ok(output_path)
}

// BAD: No memory limits
fn process_item_unsafe(item: &BatchItem) -> Result<PathBuf> {
    let input_data = std::fs::read(&item.source_path)?; // NO SIZE CHECK
    // ... conversion logic ...
    Ok(output_path)
}
```

**Memory Limits:**
- Per-file: 100 MB (configurable via ResourceLimits)
- Per-thread: 200 MB (2x file limit for processing overhead)
- Total: 1 GB (5 threads × 200 MB)

### 3. CPU Usage Limits

**RECOMMENDED:** Monitor CPU usage and throttle if system is overloaded.

```rust
// GOOD: CPU-aware thread pool sizing
fn calculate_optimal_thread_count() -> usize {
    let cpu_count = num_cpus::get();
    // Use 75% of CPU cores to leave headroom
    (cpu_count as f32 * 0.75).ceil() as usize
        .max(1)
        .min(16) // Hard limit
}
```

**CPU Limits:**
- Default: 75% of CPU cores
- Maximum: 16 threads (hard limit)
- Minimum: 1 thread

### 4. Queue Size Limits

**REQUIRED:** Enforce maximum queue size to prevent memory exhaustion.

```rust
const MAX_QUEUE_SIZE: usize = 1000;

impl BatchQueue {
    pub fn add_item(&mut self, item: BatchItem) -> Result<()> {
        if self.items.len() >= MAX_QUEUE_SIZE {
            return Err(ConversionError::ResourceLimitExceeded(
                format!("Queue is full (max {} items)", MAX_QUEUE_SIZE)
            ));
        }
        self.items.push(item);
        Ok(())
    }
}
```

**Queue Limits:**
- Maximum: 1000 items
- Warning threshold: 500 items (show warning in UI)
- Reason: Prevent memory exhaustion from too many queued items

---

## Path Validation

### 1. Input Path Validation

**REQUIRED:** Validate all input paths before processing.

```rust
use common::validation::validate_file_path;

// GOOD: Validate before processing
fn process_item(item: &BatchItem) -> Result<PathBuf> {
    // Validate input path
    validate_file_path(&item.source_path)?;
    
    // Validate output path (directory must exist and be writable)
    if let Some(parent) = item.output_path.parent() {
        validate_directory_path(parent)?;
    }
    
    // ... conversion logic ...
    Ok(item.output_path.clone())
}

// BAD: No path validation
fn process_item_unsafe(item: &BatchItem) -> Result<PathBuf> {
    std::fs::read(&item.source_path)?; // NO VALIDATION
    Ok(item.output_path.clone())
}
```

### 2. Output Path Validation

**REQUIRED:** Validate output paths to prevent writing to system directories.

```rust
use common::validation::validate_directory_path;
use std::path::Path;

// GOOD: Validate output directory
fn validate_output_path(output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        // Check directory exists and is writable
        validate_directory_path(parent)?;
        
        // Security: Prevent writing to system directories
        let canonical = parent.canonicalize()?;
        if is_system_directory(&canonical) {
            return Err(ConversionError::InvalidInput(
                "Cannot write to system directory".to_string()
            ));
        }
    }
    Ok(())
}

fn is_system_directory(path: &Path) -> bool {
    // Windows system directories
    #[cfg(windows)]
    {
        let path_str = path.to_string_lossy().to_lowercase();
        path_str.contains("\\windows\\system32") ||
        path_str.contains("\\program files") ||
        path_str.contains("\\program files (x86)")
    }
    
    // Unix system directories
    #[cfg(unix)]
    {
        path.starts_with("/bin") ||
        path.starts_with("/sbin") ||
        path.starts_with("/usr/bin") ||
        path.starts_with("/usr/sbin") ||
        path.starts_with("/etc") ||
        path.starts_with("/lib")
    }
}
```

### 3. Path Traversal Prevention

**REQUIRED:** Prevent path traversal attacks in queue item editing.

```rust
// GOOD: Validate edited paths
fn validate_edited_path(path: &Path) -> Result<()> {
    // Check for path traversal attempts
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        return Err(ConversionError::InvalidInput(
            "Path traversal not allowed".to_string()
        ));
    }
    
    // Validate using common validation
    validate_file_path(path)?;
    Ok(())
}
```

---

## Error Handling

### 1. Per-Item Error Handling

**REQUIRED:** Errors in one item must not affect other items.

```rust
// GOOD: Per-item error handling
fn process_queue_parallel(queue: Arc<Mutex<BatchQueue>>) {
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(MAX_CONCURRENT_CONVERSIONS)
        .build()
        .unwrap();
    
    loop {
        // Get next pending item
        let item_id = {
            let queue_guard = queue.lock().unwrap();
            queue_guard.next_pending().map(|idx| queue_guard.items[idx].id)
        };
        
        if let Some(id) = item_id {
            let queue_clone = queue.clone();
            thread_pool.spawn(move || {
                // Process item
                let result = process_item_safe(&queue_clone, id);
                
                // Update status (error or success)
                update_item_status(&queue_clone, id, result);
            });
        } else {
            break; // No more items
        }
    }
}

fn process_item_safe(
    queue: &Arc<Mutex<BatchQueue>>,
    item_id: Uuid,
) -> Result<PathBuf> {
    // Get item (with lock)
    let item = {
        let queue_guard = queue.lock().unwrap();
        queue_guard.items.iter()
            .find(|i| i.id == item_id)
            .cloned()
            .ok_or(ConversionError::InvalidInput("Item not found".to_string()))?
    };
    
    // Process without holding lock
    match convert_item(&item) {
        Ok(output_path) => {
            // Update status to Completed
            update_item_status(queue, item_id, BatchItemStatus::Completed {
                output_path,
            })?;
            Ok(output_path)
        }
        Err(e) => {
            // Update status to Failed
            update_item_status(queue, item_id, BatchItemStatus::Failed {
                error: e.to_string(),
            })?;
            Err(e)
        }
    }
}
```

### 2. Error Message Sanitization

**REQUIRED:** Never leak sensitive information in error messages.

```rust
use common::validation::sanitize_path;

// GOOD: Sanitized error messages
fn handle_conversion_error(error: ConversionError, item_path: &Path) -> String {
    match error {
        ConversionError::InvalidInput(msg) => {
            format!("Cannot process file '{}': {}", sanitize_path(item_path), msg)
        }
        ConversionError::Io(e) => {
            format!("File error: {}", e)
        }
        _ => "Conversion failed".to_string(),
    }
}

// BAD: Leaks full paths
fn bad_error_message(error: ConversionError, item_path: &Path) -> String {
    format!("Error processing {}: {}", item_path.display(), error) // LEAKS PATH
}
```

---

## Testing Requirements

### 1. Thread Safety Tests

**REQUIRED:** Test concurrent access patterns.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    
    #[test]
    fn test_concurrent_queue_updates() {
        let queue = Arc::new(Mutex::new(BatchQueue::new()));
        let mut handles = vec![];
        
        // Spawn 10 threads, each adding 10 items
        for i in 0..10 {
            let queue_clone = queue.clone();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let item = create_test_item();
                    let mut queue_guard = queue_clone.lock().unwrap();
                    queue_guard.add_item(item).unwrap();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all items added
        let queue_guard = queue.lock().unwrap();
        assert_eq!(queue_guard.items.len(), 100);
    }
    
    #[test]
    fn test_deadlock_prevention() {
        // Test that consistent lock ordering prevents deadlocks
        let queue1 = Arc::new(Mutex::new(BatchQueue::new()));
        let queue2 = Arc::new(Mutex::new(BatchQueue::new()));
        
        // All threads acquire locks in same order
        let mut handles = vec![];
        for _ in 0..10 {
            let q1 = queue1.clone();
            let q2 = queue2.clone();
            let handle = thread::spawn(move || {
                let _g1 = q1.lock().unwrap();
                let _g2 = q2.lock().unwrap();
                // Should not deadlock
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

### 2. Resource Limit Tests

**REQUIRED:** Test resource limit enforcement.

```rust
#[test]
fn test_max_concurrent_conversions() {
    let processor = ParallelProcessor::new(2); // Max 2 concurrent
    
    // Try to start 5 conversions
    let mut started = 0;
    for _ in 0..5 {
        if processor.start_conversion().is_ok() {
            started += 1;
        }
    }
    
    // Only 2 should succeed
    assert_eq!(started, 2);
}

#[test]
fn test_queue_size_limit() {
    let mut queue = BatchQueue::new();
    
    // Add items until limit reached
    let mut added = 0;
    for _ in 0..1500 {
        if queue.add_item(create_test_item()).is_ok() {
            added += 1;
        } else {
            break;
        }
    }
    
    assert_eq!(added, MAX_QUEUE_SIZE);
}
```

### 3. Security Edge Case Tests

**REQUIRED:** Test security edge cases.

```rust
#[test]
fn test_path_traversal_prevention() {
    let malicious_paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "/etc/passwd",
        "C:\\Windows\\System32\\config\\SAM",
    ];
    
    for path_str in malicious_paths {
        let path = PathBuf::from(path_str);
        assert!(validate_edited_path(&path).is_err());
    }
}

#[test]
fn test_resource_exhaustion_prevention() {
    // Test that large files are rejected
    let limits = ResourceLimits::builder()
        .max_file_size_mb(100)
        .build();
    
    // Create a fake large file path (would be 200MB)
    let large_file = create_large_file_path(200 * 1024 * 1024);
    assert!(read_file_bytes_checked(&large_file, &limits).is_err());
}
```

---

## Security Checklist

Before implementing parallel batch processing, verify:

- [ ] All shared state protected with `Arc<Mutex<>>` or `Arc<RwLock<>>`
- [ ] Consistent lock ordering to prevent deadlocks
- [ ] Atomic operations for simple counters/flags
- [ ] Maximum concurrent conversions enforced
- [ ] Memory limits per thread enforced
- [ ] CPU usage limits considered
- [ ] Queue size limits enforced
- [ ] All input paths validated
- [ ] All output paths validated
- [ ] Path traversal prevention in queue editing
- [ ] Per-item error handling (one failure doesn't stop queue)
- [ ] Error messages sanitized (no path leaks)
- [ ] Thread safety tests written
- [ ] Resource limit tests written
- [ ] Security edge case tests written

---

## Summary

Parallel batch processing introduces significant security considerations:

1. **Thread Safety:** All shared state must be protected, locks must be ordered consistently
2. **Resource Limits:** Enforce limits on concurrent conversions, memory, CPU, and queue size
3. **Path Validation:** Validate all paths, prevent path traversal, prevent system directory writes
4. **Error Handling:** Per-item errors, sanitized messages, no information leakage

**Status:** ✅ Guidelines ready for implementation  
**Next Steps:** Review with Senior Engineer before implementation begins

---

**Document Version:** 1.0  
**Created:** December 30, 2025  
**Last Updated:** December 30, 2025  
**Status:** Draft - Ready for Review

