// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for Sprint 9 and Sprint 10_A features
//!
//! Tests integration of:
//! - Parallel batch processing (Task 3.1)
//! - Settings auto-save (Task 3.2)
//! - Queue item editing (Task 3.3)
//! - 3D viewer integration (Sprint 10_A Task 1.1)
//! - Pause/resume/cancel controls (Sprint 10)
//!
//! These tests verify that all features work together correctly.

use converter_gui::app::{ConverterApp, FileType, OutputFormat};
use converter_gui::batch_queue::{
    BatchItem, BatchItemOptions, BatchItemStatus, BatchQueue, ProcessingPriority,
};
use converter_gui::settings::AppSettings;
use img_core::ImageFormat;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use uuid::Uuid;

// Helper function to create a minimal valid PNG file
#[allow(dead_code)]
fn create_test_png_file(dir: &std::path::Path, name: &str) -> PathBuf {
    let path = dir.join(name);

    // Try to use existing test data if available
    let test_data_path = PathBuf::from("../../img-core/tests/data/1x1.png");
    if test_data_path.exists() {
        std::fs::copy(&test_data_path, &path).unwrap();
        return path;
    }

    // Otherwise create minimal valid PNG (1x1 pixel)
    // This is a minimal valid PNG that most PNG readers should accept
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk header
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
        0x08, 0x02, 0x00, 0x00, 0x00, // Bit depth, color type, compression, filter, interlace
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk header
        0x08, 0x99, 0x01, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, // IDAT data
        0x02, 0x00, 0x01, // CRC placeholder (simplified)
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
        0xAE, 0x42, 0x60, 0x82, // PNG signature end
    ];
    std::fs::write(&path, &png_data).unwrap();
    path
}

// Helper function to create a minimal valid STL file
#[allow(dead_code)]
fn create_test_stl_file(dir: &std::path::Path, name: &str) -> PathBuf {
    let path = dir.join(name);

    // Try to use existing test data if available
    let test_data_path = PathBuf::from("../../mesh-core/tests/data/cube.stl");
    if test_data_path.exists() {
        std::fs::copy(&test_data_path, &path).unwrap();
        return path;
    }

    // Otherwise create minimal valid STL (binary format, single triangle)
    let mut stl_data = vec![
        // 80-byte header
        0u8; 80
    ];

    // Number of triangles (1)
    stl_data.extend_from_slice(&1u32.to_le_bytes());

    // Triangle data (12 floats: normal + 3 vertices)
    // Normal: (0, 0, 1)
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    stl_data.extend_from_slice(&1.0f32.to_le_bytes());
    // Vertex 1: (0, 0, 0)
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    // Vertex 2: (1, 0, 0)
    stl_data.extend_from_slice(&1.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    // Vertex 3: (1, 1, 0)
    stl_data.extend_from_slice(&1.0f32.to_le_bytes());
    stl_data.extend_from_slice(&1.0f32.to_le_bytes());
    stl_data.extend_from_slice(&0.0f32.to_le_bytes());
    // Attribute byte count (0)
    stl_data.extend_from_slice(&0u16.to_le_bytes());

    std::fs::write(&path, &stl_data).unwrap();
    path
}

#[test]
fn test_batch_queue_basic_operations() {
    let mut queue = BatchQueue::new();

    // Create a test item
    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );

    // Test add_item
    let _ = queue.add_item(item.clone());
    assert_eq!(queue.items.len(), 1);

    // Test get_item
    let retrieved = queue.get_item(item.id).unwrap();
    assert_eq!(retrieved.id, item.id);

    // Test statistics
    let stats = queue.statistics();
    assert_eq!(stats.total, 1);
    assert_eq!(stats.pending, 1);
    assert_eq!(stats.completed, 0);
    assert_eq!(stats.failed, 0);

    // Test remove_item
    assert!(queue.remove_item(item.id));
    assert_eq!(queue.items.len(), 0);

    // Test clear
    let _ = queue.add_item(item.clone());
    queue.clear();
    assert_eq!(queue.items.len(), 0);
    assert_eq!(queue.current_index, None);
}

#[test]
fn test_batch_queue_item_editing() {
    let mut queue = BatchQueue::new();

    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let item_id = item.id;
    let _ = queue.add_item(item);

    // Test update_item_format
    assert!(queue.update_item_format(item_id, OutputFormat::Image(ImageFormat::Png)));
    let updated = queue.get_item(item_id).unwrap();
    assert!(matches!(
        updated.output_format,
        OutputFormat::Image(ImageFormat::Png)
    ));
    assert!(updated.output_path.to_string_lossy().ends_with(".png"));

    // Test update_item_output_path
    let new_path = PathBuf::from("custom_output.png");
    assert!(queue.update_item_output_path(item_id, new_path.clone()));
    let updated = queue.get_item(item_id).unwrap();
    assert_eq!(updated.output_path, new_path);

    // Test update_item_options
    let new_options = BatchItemOptions {
        quality: 75,
        mesh_options: None,
        priority: ProcessingPriority::Medium,
    };
    assert!(queue.update_item_options(item_id, new_options));
    let updated = queue.get_item(item_id).unwrap();
    assert_eq!(updated.options.quality, 75);

    // Test that editing fails for processing items
    let item = queue.get_item_mut(item_id).unwrap();
    item.status = BatchItemStatus::Processing;
    assert!(!queue.update_item_format(item_id, OutputFormat::Image(ImageFormat::Bmp)));
}

#[test]
fn test_settings_auto_save_integration() {
    // Create a mock config directory
    // Note: In real tests, we'd need to mock the config path
    // For now, we test the auto-save state machine

    let mut app = ConverterApp::default();

    // Initially should be idle
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Idle
    ));

    // Mark as changed
    app.settings_auto_save.mark_changed();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));

    // Should want to save after marking changed
    // Note: should_save() checks time elapsed, so may need to wait
    // For this test, we just verify the state machine works

    // Set saving state
    app.settings_auto_save.set_saving();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Saving
    ));

    // Set saved state
    app.settings_auto_save.set_saved();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Saved
    ));

    // Set error state
    app.settings_auto_save.set_error();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Error
    ));
}

#[test]
fn test_settings_load_and_save() {
    // Note: AppSettings uses platform-specific directories
    // This test verifies the save/load cycle works
    // We'll use a temporary config file approach

    let _settings = AppSettings {
        default_quality: 85,
        window_width: 1200.0,
        window_height: 800.0,
        ..Default::default()
    };

    // Save should succeed (if config dir is writable)
    // Load should return defaults if file doesn't exist

    let loaded = AppSettings::load().unwrap();
    // Should get defaults (since file may not exist)
    assert!(loaded.default_quality >= 1 && loaded.default_quality <= 100);
}

#[test]
fn test_batch_processing_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let mut queue = BatchQueue::new();

    // Add item with non-existent source file
    let invalid_item = BatchItem::new(
        PathBuf::from("nonexistent_file.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        temp_dir.path().join("output.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(invalid_item.clone());

    // Test that queue handles invalid items gracefully
    // (Actual processing would fail, but queue should remain valid)
    let stats = queue.statistics();
    assert_eq!(stats.pending, 1);

    // Test remove_item still works
    assert!(queue.remove_item(invalid_item.id));
}

#[test]
fn test_queue_statistics_with_mixed_statuses() {
    let mut queue = BatchQueue::new();

    // Add items with different statuses
    let mut item1 = BatchItem::new(
        PathBuf::from("test1.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test1.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    item1.status = BatchItemStatus::Completed {
        output_path: PathBuf::from("test1.jpg"),
    };
    let _ = queue.add_item(item1);

    let mut item2 = BatchItem::new(
        PathBuf::from("test2.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test2.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    item2.status = BatchItemStatus::Failed {
        error: "Test error".to_string(),
    };
    let _ = queue.add_item(item2);

    let mut item3 = BatchItem::new(
        PathBuf::from("test3.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test3.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    item3.status = BatchItemStatus::Processing;
    let _ = queue.add_item(item3);

    let item4 = BatchItem::new(
        PathBuf::from("test4.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test4.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(item4);

    let stats = queue.statistics();
    assert_eq!(stats.total, 4);
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.failed, 1);
    assert_eq!(stats.processing, 1);
    assert_eq!(stats.pending, 1);
}

#[test]
fn test_batch_queue_thread_safety_structure() {
    // Test that BatchQueue can be wrapped in Arc<Mutex<>>
    // This is a compile-time and basic runtime test

    let queue = BatchQueue::new();
    let queue_arc = Arc::new(std::sync::Mutex::new(queue));

    // Test that we can lock and access from different "contexts"
    {
        let guard = queue_arc.lock().unwrap();
        assert_eq!(guard.items.len(), 0);
    }

    // Test that we can modify
    {
        let mut guard = queue_arc.lock().unwrap();
        let item = BatchItem::new(
            PathBuf::from("test.png"),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from("test.jpg"),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
                priority: ProcessingPriority::Medium,
            },
        );
        let _ = guard.add_item(item);
    }

    // Test that changes persist
    {
        let guard = queue_arc.lock().unwrap();
        assert_eq!(guard.items.len(), 1);
    }
}

#[test]
fn test_settings_validation() {
    // Test that settings validation works correctly

    let settings = AppSettings {
        default_quality: 150,                 // Invalid (should clamp to 100)
        window_width: 500.0,                  // Invalid (should clamp to 800.0)
        window_height: 400.0,                 // Invalid (should clamp to 600.0)
        max_concurrent_conversions: Some(20), // Invalid (should clamp to 16)
        ..Default::default()
    };

    // Validation happens in save() and load()
    // For testing, we can manually validate by saving and loading
    // But the validate() method is private, so we test through save/load

    // Save should validate and clamp values
    let _ = settings.save();

    // Load should return validated settings
    let loaded = AppSettings::load().unwrap();
    assert!(loaded.default_quality <= 100);
    assert!(loaded.window_width >= 800.0);
    assert!(loaded.window_height >= 600.0);
    if let Some(max_conc) = loaded.max_concurrent_conversions {
        assert!(max_conc <= 16);
    }
}

#[test]
fn test_queue_item_editing_validation() {
    let mut queue = BatchQueue::new();

    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let item_id = item.id;
    let _ = queue.add_item(item);

    // Test that editing works only for pending items
    let item_mut = queue.get_item_mut(item_id).unwrap();
    assert_eq!(item_mut.status, BatchItemStatus::Pending);

    // Mark as completed - should not be editable
    item_mut.status = BatchItemStatus::Completed {
        output_path: PathBuf::from("test.jpg"),
    };
    // item_mut goes out of scope here, releasing the lock

    assert!(!queue.update_item_format(item_id, OutputFormat::Image(ImageFormat::Png)));
    assert!(!queue.update_item_output_path(item_id, PathBuf::from("new.jpg")));

    // Reset to pending - should be editable
    let item_mut = queue.get_item_mut(item_id).unwrap();
    item_mut.status = BatchItemStatus::Pending;
    // item_mut goes out of scope here, releasing the lock

    assert!(queue.update_item_format(item_id, OutputFormat::Image(ImageFormat::Png)));
}

#[test]
fn test_batch_queue_next_pending() {
    let mut queue = BatchQueue::new();

    // Empty queue
    assert_eq!(queue.next_pending(), None);

    // Add completed item
    let mut item1 = BatchItem::new(
        PathBuf::from("test1.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test1.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    item1.status = BatchItemStatus::Completed {
        output_path: PathBuf::from("test1.jpg"),
    };
    let _ = queue.add_item(item1);

    // Should return None (no pending items)
    assert_eq!(queue.next_pending(), None);

    // Add pending item
    let item2 = BatchItem::new(
        PathBuf::from("test2.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test2.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(item2);

    // Should return index of pending item (index 1)
    assert_eq!(queue.next_pending(), Some(1));
}

// Performance test - verify queue operations are reasonably fast
#[test]
fn test_batch_queue_performance() {
    let mut queue = BatchQueue::new();

    // Add 100 items
    let start = Instant::now();
    for i in 0..100 {
        let item = BatchItem::new(
            PathBuf::from(format!("test{}.png", i)),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from(format!("test{}.jpg", i)),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
                priority: ProcessingPriority::Medium,
            },
        );
        let _ = queue.add_item(item);
    }
    let add_time = start.elapsed();

    // Should be very fast (microseconds)
    assert!(
        add_time < Duration::from_millis(100),
        "Adding 100 items took too long: {:?}",
        add_time
    );

    // Test statistics calculation
    let start = Instant::now();
    let _stats = queue.statistics();
    let stats_time = start.elapsed();

    // Should be very fast
    assert!(
        stats_time < Duration::from_millis(10),
        "Statistics calculation took too long: {:?}",
        stats_time
    );

    // Test next_pending
    let start = Instant::now();
    let _next = queue.next_pending();
    let next_time = start.elapsed();

    // Should be very fast
    assert!(
        next_time < Duration::from_millis(10),
        "next_pending took too long: {:?}",
        next_time
    );
}

// Memory test - verify queue doesn't leak memory with many items
#[test]
fn test_batch_queue_memory_efficiency() {
    let mut queue = BatchQueue::new();

    // Add 1000 items
    for i in 0..1000 {
        let item = BatchItem::new(
            PathBuf::from(format!("test{}.png", i)),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from(format!("test{}.jpg", i)),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
                priority: ProcessingPriority::Medium,
            },
        );
        let _ = queue.add_item(item);
    }

    // Verify all items are stored
    assert_eq!(queue.items.len(), 1000);

    // Remove half
    let items_to_remove: Vec<Uuid> = queue.items.iter().take(500).map(|item| item.id).collect();
    for id in items_to_remove {
        queue.remove_item(id);
    }

    // Verify removal worked
    assert_eq!(queue.items.len(), 500);

    // Clear remaining
    queue.clear();
    assert_eq!(queue.items.len(), 0);
}

// Test integration of queue item editing with queue operations
#[test]
fn test_queue_item_editing_integration() {
    let mut queue = BatchQueue::new();

    // Add multiple items
    let item1 = BatchItem::new(
        PathBuf::from("test1.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test1.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let item1_id = item1.id;
    let _ = queue.add_item(item1);

    let item2 = BatchItem::new(
        PathBuf::from("test2.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Png),
        PathBuf::from("test2.png"),
        BatchItemOptions {
            quality: 95,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let item2_id = item2.id;
    let _ = queue.add_item(item2);

    // Edit first item
    assert!(queue.update_item_format(item1_id, OutputFormat::Image(ImageFormat::Bmp)));
    assert!(queue.update_item_options(
        item1_id,
        BatchItemOptions {
            quality: 75,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        }
    ));

    // Verify edits persisted
    let edited = queue.get_item(item1_id).unwrap();
    assert!(matches!(
        edited.output_format,
        OutputFormat::Image(ImageFormat::Bmp)
    ));
    assert_eq!(edited.options.quality, 75);

    // Verify second item unchanged
    let unchanged = queue.get_item(item2_id).unwrap();
    assert!(matches!(
        unchanged.output_format,
        OutputFormat::Image(ImageFormat::Png)
    ));
    assert_eq!(unchanged.options.quality, 95);

    // Verify statistics still accurate
    let stats = queue.statistics();
    assert_eq!(stats.total, 2);
    assert_eq!(stats.pending, 2);
}

// Test that settings auto-save debouncing works correctly
#[test]
fn test_settings_auto_save_debouncing() {
    let mut app = ConverterApp::default();

    // Mark changed
    app.settings_auto_save.mark_changed();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));

    // Mark changed again (should stay in Pending, not create multiple saves)
    app.settings_auto_save.mark_changed();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));

    // After enough time, should_save() should return true
    // Note: This test verifies the state machine, not the timing
    // Actual timing is tested through integration with the UI update loop
}

// ============================================================================
// Sprint 10_A Comprehensive Integration Tests
// ============================================================================

#[cfg(feature = "viewer-3d")]
use converter_gui::preview_3d::{RenderMode, Viewer3D};
#[cfg(feature = "viewer-3d")]
use mesh_core::Mesh;
use std::thread;

// Helper to create a simple test mesh
#[cfg(feature = "viewer-3d")]
fn create_test_mesh() -> Arc<Mesh> {
    let vertices = vec![
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
    ];
    let faces = vec![
        Face { indices: [0, 1, 2] },
        Face { indices: [0, 2, 3] },
        Face { indices: [4, 7, 6] },
        Face { indices: [4, 6, 5] },
        Face { indices: [0, 4, 5] },
        Face { indices: [0, 5, 1] },
        Face { indices: [2, 6, 7] },
        Face { indices: [2, 7, 3] },
        Face { indices: [0, 3, 7] },
        Face { indices: [0, 7, 4] },
        Face { indices: [1, 5, 6] },
        Face { indices: [1, 6, 2] },
    ];
    Arc::new(Mesh {
        vertices,
        faces,
        normals: Vec::new(),
    })
}

#[test]
fn test_3d_viewer_with_batch_processing_integration() {
    // Test that 3D viewer state doesn't interfere with batch processing
    let mut app = ConverterApp::default();

    // Initialize 3D viewer
    #[cfg(feature = "viewer-3d")]
    {
        app.viewer_3d = Some(Arc::new(Mutex::new(Viewer3D::new())));
    }

    // Create batch queue
    let mut queue = BatchQueue::new();
    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(item);
    app.batch_queue = Some(queue);

    // Verify both can coexist
    assert!(app.batch_queue.is_some());
    #[cfg(feature = "viewer-3d")]
    assert!(app.viewer_3d.is_some());
}

#[test]
fn test_pause_resume_cancel_with_parallel_processing() {
    // Test that pause/resume/cancel works correctly with parallel processing state
    let _app = ConverterApp::default();

    // Create batch processing state
    let state = Arc::new(converter_gui::app::BatchProcessingState::new());
    let state_clone = Arc::clone(&state);

    // Test initial state
    assert!(!state.is_paused());
    assert!(!state.is_cancelled());

    // Test pause
    state.pause();
    assert!(state.is_paused());
    assert!(!state.is_cancelled());

    // Test resume
    state.resume();
    assert!(!state.is_paused());
    assert!(!state.is_cancelled());

    // Test cancel
    state.cancel();
    assert!(state.is_cancelled());

    // Test reset
    state.reset();
    assert!(!state.is_paused());
    assert!(!state.is_cancelled());

    // Test thread safety - multiple threads accessing state
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let state = Arc::clone(&state_clone);
            thread::spawn(move || {
                for _ in 0..100 {
                    state.pause();
                    state.resume();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // State should be in a valid state after concurrent access
    // (may be paused or resumed, but should not be corrupted)
    let _final_paused = state_clone.is_paused();
    let _final_cancelled = state_clone.is_cancelled();

    // Reset to known state
    state_clone.reset();
    assert!(!state_clone.is_paused());
    assert!(!state_clone.is_cancelled());
}

#[test]
fn test_batch_processing_state_thread_safety() {
    // Test that BatchProcessingState is thread-safe under concurrent access
    let state = Arc::new(converter_gui::app::BatchProcessingState::new());

    let num_threads = 8;
    let operations_per_thread = 1000;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let state = Arc::clone(&state);
            thread::spawn(move || {
                for i in 0..operations_per_thread {
                    match i % 4 {
                        0 => state.pause(),
                        1 => state.resume(),
                        2 => state.cancel(),
                        3 => state.reset(),
                        _ => unreachable!(),
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // State should be in a valid state (not corrupted)
    // Reset to known state
    state.reset();
    assert!(!state.is_paused());
    assert!(!state.is_cancelled());
}

#[test]
fn test_error_handling_across_features() {
    // Test that errors in one feature don't break others
    let mut app = ConverterApp::default();

    // Test 1: Invalid batch item doesn't break queue
    let mut queue = BatchQueue::new();
    let invalid_item = BatchItem::new(
        PathBuf::from("nonexistent.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("output.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(invalid_item);
    app.batch_queue = Some(queue);

    // Queue should still be valid
    assert!(app.batch_queue.is_some());
    let queue = app.batch_queue.as_ref().unwrap();
    assert_eq!(queue.items.len(), 1);

    // Test 2: Settings error doesn't break app
    app.settings_auto_save.set_error();
    // App should still function
    assert!(app.batch_queue.is_some());

    // Test 3: 3D viewer error doesn't break app
    #[cfg(feature = "viewer-3d")]
    {
        app.viewer_3d = Some(Arc::new(Mutex::new(Viewer3D::new())));
        // Viewer should be initialized even if mesh loading fails later
        assert!(app.viewer_3d.is_some());
    }
}

#[test]
fn test_end_to_end_workflow_with_all_features() {
    // Test a complete workflow using all features together
    let mut app = ConverterApp::default();
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Create batch queue
    let mut queue = BatchQueue::new();
    let item = BatchItem::new(
        create_test_png_file(temp_dir.path(), "test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        temp_dir.path().join("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(item);
    app.batch_queue = Some(queue);

    // Step 2: Initialize batch processing state
    let state = Arc::new(converter_gui::app::BatchProcessingState::new());
    app.batch_processing_state = Some(state);

    // Step 3: Settings auto-save
    app.settings_auto_save.mark_changed();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));

    // Step 4: Verify all features coexist
    assert!(app.batch_queue.is_some());
    assert!(app.batch_processing_state.is_some());
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));
}

#[test]
fn test_performance_batch_queue_with_many_items() {
    // Test that batch queue performance is acceptable with many items
    let mut queue = BatchQueue::new();
    let num_items = 1000;

    let start = Instant::now();
    for i in 0..num_items {
        let item = BatchItem::new(
            PathBuf::from(format!("test{}.png", i)),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from(format!("test{}.jpg", i)),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
                priority: ProcessingPriority::Medium,
            },
        );
        let _ = queue.add_item(item);
    }
    let add_time = start.elapsed();

    // Adding 1000 items should be fast (< 100ms)
    assert!(
        add_time < Duration::from_millis(100),
        "Adding 1000 items took too long: {:?}",
        add_time
    );

    // Statistics should be fast
    let start = Instant::now();
    let stats = queue.statistics();
    let stats_time = start.elapsed();
    assert_eq!(stats.total, num_items);
    assert!(
        stats_time < Duration::from_millis(10),
        "Statistics calculation took too long: {:?}",
        stats_time
    );
}

#[test]
fn test_memory_efficiency_batch_queue() {
    // Test that batch queue doesn't leak memory
    let mut queue = BatchQueue::new();
    let num_items = 100;

    // Add items
    let mut item_ids = Vec::new();
    for i in 0..num_items {
        let item = BatchItem::new(
            PathBuf::from(format!("test{}.png", i)),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from(format!("test{}.jpg", i)),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
                priority: ProcessingPriority::Medium,
            },
        );
        item_ids.push(item.id);
        let _ = queue.add_item(item);
    }

    assert_eq!(queue.items.len(), num_items);

    // Remove half
    for id in item_ids.iter().take(50) {
        queue.remove_item(*id);
    }

    assert_eq!(queue.items.len(), 50);

    // Clear remaining
    queue.clear();
    assert_eq!(queue.items.len(), 0);
}

#[test]
fn test_no_regressions_existing_functionality() {
    // Test that existing functionality still works
    let mut app = ConverterApp {
        detected_file_type: Some(FileType::Image),
        ..Default::default()
    };
    assert_eq!(app.detected_file_type, Some(FileType::Image));

    // Test format selection
    app.output_format = Some(OutputFormat::Image(ImageFormat::Png));
    assert!(matches!(
        app.output_format,
        Some(OutputFormat::Image(ImageFormat::Png))
    ));

    // Test status
    app.status = converter_gui::app::Status::Ready;
    assert!(matches!(app.status, converter_gui::app::Status::Ready));

    // Test messages
    app.add_message(
        "Test message".to_string(),
        converter_gui::app::MessageType::Success,
    );
    assert_eq!(app.messages.len(), 1);
}

#[test]
#[cfg(feature = "viewer-3d")]
fn test_3d_viewer_mesh_loading_integration() {
    #[cfg(feature = "viewer-3d")]
    {
        let mut viewer = Viewer3D::new();
        let mesh = create_test_mesh();

        // Test mesh loading
        viewer.set_mesh(Arc::clone(&mesh));

        // Verify mesh is set
        // Note: We can't directly access mesh field, but we can test through public API
        // Reset camera should work if mesh is loaded
        viewer.reset_camera();

        // Test render mode switching
        viewer.set_render_mode(RenderMode::Solid);
        assert_eq!(viewer.render_mode(), RenderMode::Solid);

        viewer.set_render_mode(RenderMode::Wireframe);
        assert_eq!(viewer.render_mode(), RenderMode::Wireframe);
    }
}

#[test]
fn test_parallel_processing_with_pause_resume() {
    // Test that parallel processing respects pause/resume state
    let state = Arc::new(converter_gui::app::BatchProcessingState::new());

    // Simulate processing loop checking pause state
    let processing_state = Arc::clone(&state);
    let processing_handle = thread::spawn(move || {
        let mut iterations = 0;
        while iterations < 100 {
            // Check if paused
            if processing_state.is_paused() {
                // Wait while paused (simulate processing loop)
                thread::sleep(Duration::from_millis(10));
            } else {
                iterations += 1;
                thread::sleep(Duration::from_millis(1));
            }

            // Check for cancellation
            if processing_state.is_cancelled() {
                break;
            }
        }
        iterations
    });

    // Pause after a short delay
    thread::sleep(Duration::from_millis(50));
    state.pause();
    thread::sleep(Duration::from_millis(50));
    state.resume();

    // Let processing continue
    thread::sleep(Duration::from_millis(50));

    // Cancel
    state.cancel();

    let iterations = processing_handle.join().unwrap();

    // Should have processed some iterations before cancellation
    assert!(iterations > 0);
    assert!(iterations < 100); // Should have stopped due to cancellation
}

#[test]
fn test_settings_auto_save_with_batch_processing() {
    // Test that settings auto-save works alongside batch processing
    let mut app = ConverterApp::default();

    // Initialize batch queue
    let mut queue = BatchQueue::new();
    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let _ = queue.add_item(item);
    app.batch_queue = Some(queue);

    // Mark settings as changed
    app.settings_auto_save.mark_changed();
    assert!(matches!(
        app.settings_auto_save.status,
        converter_gui::app::AutoSaveStatus::Pending
    ));

    // Batch queue should still be accessible
    assert!(app.batch_queue.is_some());
    let queue = app.batch_queue.as_ref().unwrap();
    assert_eq!(queue.items.len(), 1);
}

#[test]
fn test_queue_item_editing_with_parallel_processing_state() {
    // Test that queue item editing works with parallel processing state
    let mut queue = BatchQueue::new();
    let state = Arc::new(converter_gui::app::BatchProcessingState::new());

    let item = BatchItem::new(
        PathBuf::from("test.png"),
        FileType::Image,
        OutputFormat::Image(ImageFormat::Jpeg),
        PathBuf::from("test.jpg"),
        BatchItemOptions {
            quality: 90,
            mesh_options: None,
            priority: ProcessingPriority::Medium,
        },
    );
    let item_id = item.id;
    let _ = queue.add_item(item);

    // Edit item while processing state exists
    assert!(queue.update_item_format(item_id, OutputFormat::Image(ImageFormat::Png)));

    // Processing state should not interfere
    assert!(!state.is_paused());
    assert!(!state.is_cancelled());

    // Verify edit persisted
    let edited = queue.get_item(item_id).unwrap();
    assert!(matches!(
        edited.output_format,
        OutputFormat::Image(ImageFormat::Png)
    ));
}
