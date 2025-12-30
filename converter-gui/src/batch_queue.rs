// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Batch processing queue for Simple Image Converter GUI
//!
//! This module provides data structures and management for batch conversion
//! operations, allowing users to queue multiple files for conversion.

use crate::app::{FileType, OutputFormat};
use std::path::PathBuf;
use uuid::Uuid;

/// A single item in the batch conversion queue
#[derive(Debug, Clone)]
pub struct BatchItem {
    /// Unique identifier for this queue item
    pub id: Uuid,
    /// Source file path
    pub source_path: PathBuf,
    /// Detected file type (Image or Mesh)
    #[allow(dead_code)] // Reserved for future filtering/grouping
    pub file_type: FileType,
    /// Output format selected for conversion
    pub output_format: OutputFormat,
    /// Output file path
    pub output_path: PathBuf,
    /// Conversion options (quality, mesh options, etc.)
    pub options: BatchItemOptions,
    /// Current status of this item
    pub status: BatchItemStatus,
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Error message if conversion failed
    pub error: Option<String>,
}

/// Options for batch item conversion
#[derive(Debug, Clone)]
pub struct BatchItemOptions {
    /// Quality setting (1-100) for lossy image formats
    pub quality: u8,
    /// Mesh conversion options (if applicable)
    pub mesh_options: Option<MeshOptions>,
}

/// Mesh-specific conversion options
#[derive(Debug, Clone)]
pub struct MeshOptions {
    /// Coordinate system transform (from, to)
    pub transform: Option<(mesh_core::CoordinateSystem, mesh_core::CoordinateSystem)>,
    /// Whether to recalculate vertex normals
    pub recalculate_normals: bool,
    /// Whether to validate mesh integrity
    pub validate: bool,
}

impl From<mesh_core::ConversionOptions> for MeshOptions {
    fn from(options: mesh_core::ConversionOptions) -> Self {
        Self {
            transform: options.transform,
            recalculate_normals: options.recalculate_normals,
            validate: options.validate,
        }
    }
}

impl From<MeshOptions> for mesh_core::ConversionOptions {
    fn from(options: MeshOptions) -> Self {
        Self {
            transform: options.transform,
            recalculate_normals: options.recalculate_normals,
            validate: options.validate,
        }
    }
}

/// Status of a batch queue item
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchItemStatus {
    /// Waiting to be processed
    Pending,
    /// Currently being processed
    Processing,
    /// Conversion completed successfully
    Completed { output_path: PathBuf },
    /// Conversion failed
    Failed { error: String },
    /// Conversion was cancelled
    #[allow(dead_code)] // Reserved for future cancel functionality
    Cancelled,
}

impl BatchItem {
    /// Create a new batch item
    pub fn new(
        source_path: PathBuf,
        file_type: FileType,
        output_format: OutputFormat,
        output_path: PathBuf,
        options: BatchItemOptions,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_path,
            file_type,
            output_format,
            output_path,
            options,
            status: BatchItemStatus::Pending,
            progress: 0.0,
            error: None,
        }
    }
}

/// Batch processing queue
///
/// Manages a collection of batch items for sequential processing.
#[derive(Debug, Clone)]
pub struct BatchQueue {
    /// Queue items
    pub items: Vec<BatchItem>,
    /// Index of currently processing item (None if not processing)
    pub current_index: Option<usize>,
}

impl Default for BatchQueue {
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current_index: None,
        }
    }
}

impl BatchQueue {
    /// Create a new empty batch queue
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item to the queue
    pub fn add_item(&mut self, item: BatchItem) {
        self.items.push(item);
    }

    /// Remove an item from the queue by ID
    ///
    /// Returns `true` if the item was found and removed.
    pub fn remove_item(&mut self, id: Uuid) -> bool {
        let initial_len = self.items.len();
        self.items.retain(|item| item.id != id);
        self.items.len() < initial_len
    }

    /// Clear all items from the queue
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
    }

    /// Get the next pending item to process
    ///
    /// Returns the index of the next pending item, or `None` if all items
    /// are processed or there are no items.
    pub fn next_pending(&self) -> Option<usize> {
        self.items
            .iter()
            .position(|item| item.status == BatchItemStatus::Pending)
    }

    /// Get queue statistics
    pub fn statistics(&self) -> QueueStatistics {
        let total = self.items.len();
        let completed = self
            .items
            .iter()
            .filter(|item| matches!(item.status, BatchItemStatus::Completed { .. }))
            .count();
        let failed = self
            .items
            .iter()
            .filter(|item| matches!(item.status, BatchItemStatus::Failed { .. }))
            .count();
        let pending = self
            .items
            .iter()
            .filter(|item| item.status == BatchItemStatus::Pending)
            .count();
        let processing = self
            .items
            .iter()
            .filter(|item| item.status == BatchItemStatus::Processing)
            .count();

        QueueStatistics {
            total,
            completed,
            failed,
            pending,
            processing,
        }
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if queue has any pending items
    pub fn has_pending(&self) -> bool {
        self.items
            .iter()
            .any(|item| item.status == BatchItemStatus::Pending)
    }
}

/// Queue statistics for display in UI
#[derive(Debug, Clone, Copy)]
pub struct QueueStatistics {
    /// Total number of items
    pub total: usize,
    /// Number of completed items
    pub completed: usize,
    /// Number of failed items
    pub failed: usize,
    /// Number of pending items
    pub pending: usize,
    /// Number of items currently processing
    pub processing: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::OutputFormat;
    use img_core::ImageFormat;

    fn create_test_item() -> BatchItem {
        BatchItem::new(
            PathBuf::from("test.png"),
            FileType::Image,
            OutputFormat::Image(ImageFormat::Jpeg),
            PathBuf::from("test.jpg"),
            BatchItemOptions {
                quality: 90,
                mesh_options: None,
            },
        )
    }

    #[test]
    fn test_queue_add_remove() {
        let mut queue = BatchQueue::new();
        let item = create_test_item();
        let id = item.id;

        queue.add_item(item);
        assert_eq!(queue.items.len(), 1);

        assert!(queue.remove_item(id));
        assert_eq!(queue.items.len(), 0);
    }

    #[test]
    fn test_queue_clear() {
        let mut queue = BatchQueue::new();
        queue.add_item(create_test_item());
        queue.add_item(create_test_item());
        assert_eq!(queue.items.len(), 2);

        queue.clear();
        assert_eq!(queue.items.len(), 0);
    }

    #[test]
    fn test_queue_statistics() {
        let mut queue = BatchQueue::new();
        let mut item1 = create_test_item();
        item1.status = BatchItemStatus::Completed {
            output_path: PathBuf::from("out1.jpg"),
        };
        queue.add_item(item1);

        let mut item2 = create_test_item();
        item2.status = BatchItemStatus::Failed {
            error: "Test error".to_string(),
        };
        queue.add_item(item2);

        let item3 = create_test_item();
        queue.add_item(item3);

        let stats = queue.statistics();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.pending, 1);
    }

    #[test]
    fn test_next_pending() {
        let mut queue = BatchQueue::new();
        let mut item1 = create_test_item();
        item1.status = BatchItemStatus::Completed {
            output_path: PathBuf::from("out1.jpg"),
        };
        queue.add_item(item1);

        let item2 = create_test_item();
        queue.add_item(item2);

        assert_eq!(queue.next_pending(), Some(1));
    }
}
