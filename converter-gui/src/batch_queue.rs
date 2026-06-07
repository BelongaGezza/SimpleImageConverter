// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Batch processing queue for Simple Image Converter GUI
//!
//! This module provides data structures and management for batch conversion
//! operations, allowing users to queue multiple files for conversion.

use crate::app::{FileType, OutputFormat};
use std::collections::HashSet;
use std::path::PathBuf;
use uuid::Uuid;

/// Maximum number of items allowed in the batch queue
///
/// This limit prevents memory exhaustion attacks where a malicious user
/// could add thousands of items to the queue.
pub const MAX_QUEUE_SIZE: usize = 1000;

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

/// Priority level for batch processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessingPriority {
    /// Low priority - processed last
    ///
    /// Note: This variant is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used when priority selection controls are added to the batch queue UI.
    #[allow(dead_code)]
    Low = 0,
    /// Medium priority - processed after high priority
    Medium = 1,
    /// High priority - processed first
    ///
    /// Note: This variant is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used when priority selection controls are added to the batch queue UI.
    #[allow(dead_code)]
    High = 2,
}

#[allow(clippy::derivable_impls)] // Manual impl needed because default is Medium, not first variant (Low)
impl Default for ProcessingPriority {
    fn default() -> Self {
        Self::Medium
    }
}

/// Options for batch item conversion
#[derive(Debug, Clone)]
pub struct BatchItemOptions {
    /// Quality setting (1-100) for lossy image formats
    pub quality: u8,
    /// Mesh conversion options (if applicable)
    pub mesh_options: Option<MeshOptions>,
    /// Processing priority (High, Medium, Low)
    pub priority: ProcessingPriority,
}

/// Mesh-specific conversion options
#[derive(Debug, Clone, PartialEq, Eq)]
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
/// Manages a collection of batch items for sequential or parallel processing.
#[derive(Debug, Clone)]
pub struct BatchQueue {
    /// Queue items
    pub items: Vec<BatchItem>,
    /// Index of currently processing item (None if not processing)
    /// Used for sequential processing mode
    pub current_index: Option<usize>,
    /// Set of IDs currently being processed (for parallel processing)
    pub processing_ids: HashSet<Uuid>,
    /// Overall progress (0.0 to 1.0)
    pub overall_progress: f32,
    /// Total items processed (for progress calculation)
    pub processed_count: usize,
}

impl Default for BatchQueue {
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current_index: None,
            processing_ids: HashSet::new(),
            overall_progress: 0.0,
            processed_count: 0,
        }
    }
}

impl BatchQueue {
    /// Create a new empty batch queue
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item to the queue
    ///
    /// # Errors
    ///
    /// Returns an error if the queue is full (exceeds MAX_QUEUE_SIZE).
    pub fn add_item(&mut self, item: BatchItem) -> Result<(), String> {
        if self.items.len() >= MAX_QUEUE_SIZE {
            return Err(format!(
                "Queue is full (max {} items). Please remove some items before adding more.",
                MAX_QUEUE_SIZE
            ));
        }
        self.items.push(item);
        Ok(())
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
        self.processing_ids.clear();
        self.overall_progress = 0.0;
        self.processed_count = 0;
    }

    /// Get the next pending item to process
    ///
    /// Returns the index of the next pending item, or `None` if all items
    /// are processed or there are no items.
    #[allow(dead_code)]
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

    /// Get a mutable reference to an item by ID
    ///
    /// Returns `None` if the item is not found or is not in a state that allows editing.
    pub fn get_item_mut(&mut self, id: Uuid) -> Option<&mut BatchItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Get a reference to an item by ID
    pub fn get_item(&self, id: Uuid) -> Option<&BatchItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Mark item as processing (thread-safe for parallel processing)
    ///
    /// Returns `true` if the item was successfully marked as processing.
    pub fn mark_processing(&mut self, id: Uuid) -> bool {
        // Check if already processing
        if self.processing_ids.contains(&id) {
            return false;
        }

        // Find item and update status
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            if item.status == BatchItemStatus::Pending {
                item.status = BatchItemStatus::Processing;
                self.processing_ids.insert(id);
                return true;
            }
        }
        false
    }

    /// Update item status (thread-safe for parallel processing)
    pub fn update_item_status(&mut self, id: Uuid, status: BatchItemStatus, progress: f32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = status.clone();
            item.progress = progress;

            // Remove from processing set if completed/failed
            match status {
                BatchItemStatus::Completed { .. } | BatchItemStatus::Failed { .. } => {
                    self.processing_ids.remove(&id);
                    self.processed_count += 1;
                }
                _ => {}
            }

            // Update overall progress
            self.update_overall_progress();
        }
    }

    /// Get pending items (for parallel processing)
    ///
    /// Returns up to `limit` pending item IDs that are not currently processing.
    /// Items are sorted by priority (High first, then Medium, then Low).
    pub fn get_pending_items(&self, limit: usize) -> Vec<Uuid> {
        let mut pending: Vec<_> = self
            .items
            .iter()
            .filter(|item| {
                item.status == BatchItemStatus::Pending && !self.processing_ids.contains(&item.id)
            })
            .map(|item| (item.id, item.options.priority))
            .collect();

        // Sort by priority (High first, then Medium, then Low)
        pending.sort_by_key(|b| std::cmp::Reverse(b.1));

        pending.into_iter().take(limit).map(|(id, _)| id).collect()
    }

    /// Update overall progress based on completed/failed items
    fn update_overall_progress(&mut self) {
        let total = self.items.len();
        if total > 0 {
            self.overall_progress = self.processed_count as f32 / total as f32;
        } else {
            self.overall_progress = 0.0;
        }
    }

    /// Update an item's output format and regenerate output path
    ///
    /// Returns `true` if the item was found and updated.
    pub fn update_item_format(
        &mut self,
        id: Uuid,
        output_format: crate::app::OutputFormat,
    ) -> bool {
        if let Some(item) = self.get_item_mut(id) {
            // Only allow editing pending items
            if !matches!(item.status, BatchItemStatus::Pending) {
                return false;
            }

            item.output_format = output_format;

            // Regenerate output path with new extension
            if let Some(stem) = item.source_path.file_stem().and_then(|s| s.to_str()) {
                let ext = match output_format {
                    crate::app::OutputFormat::Image(fmt) => {
                        crate::format_helpers::get_format_extension(fmt)
                    }
                    crate::app::OutputFormat::Mesh(fmt) => {
                        crate::format_helpers::get_mesh_format_extension(fmt)
                    }
                };
                item.output_path = item
                    .source_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(format!("{}.{}", stem, ext));
            }

            true
        } else {
            false
        }
    }

    /// Update an item's output path
    ///
    /// Returns `true` if the item was found and updated.
    pub fn update_item_output_path(&mut self, id: Uuid, output_path: PathBuf) -> bool {
        if let Some(item) = self.get_item_mut(id) {
            // Only allow editing pending items
            if !matches!(item.status, BatchItemStatus::Pending) {
                return false;
            }

            item.output_path = output_path;
            true
        } else {
            false
        }
    }

    /// Update an item's options (quality, mesh options)
    ///
    /// Returns `true` if the item was found and updated.
    pub fn update_item_options(&mut self, id: Uuid, options: BatchItemOptions) -> bool {
        if let Some(item) = self.get_item_mut(id) {
            // Only allow editing pending items
            if !matches!(item.status, BatchItemStatus::Pending) {
                return false;
            }

            item.options = options;
            true
        } else {
            false
        }
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
                priority: ProcessingPriority::Medium,
            },
        )
    }

    #[test]
    fn test_queue_add_remove() {
        let mut queue = BatchQueue::new();
        let item = create_test_item();
        let id = item.id;

        queue.add_item(item).unwrap();
        assert_eq!(queue.items.len(), 1);

        assert!(queue.remove_item(id));
        assert_eq!(queue.items.len(), 0);
    }

    #[test]
    fn test_queue_clear() {
        let mut queue = BatchQueue::new();
        queue.add_item(create_test_item()).unwrap();
        queue.add_item(create_test_item()).unwrap();
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
        queue.add_item(item1).unwrap();

        let mut item2 = create_test_item();
        item2.status = BatchItemStatus::Failed {
            error: "Test error".to_string(),
        };
        queue.add_item(item2).unwrap();

        let item3 = create_test_item();
        queue.add_item(item3).unwrap();

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
        queue.add_item(item1).unwrap();

        let item2 = create_test_item();
        queue.add_item(item2).unwrap();

        assert_eq!(queue.next_pending(), Some(1));
    }

    #[test]
    fn test_queue_size_limit() {
        let mut queue = BatchQueue::new();

        // Fill queue to limit
        for _ in 0..MAX_QUEUE_SIZE {
            queue.add_item(create_test_item()).unwrap();
        }
        assert_eq!(queue.items.len(), MAX_QUEUE_SIZE);

        // Attempting to add one more should fail
        let result = queue.add_item(create_test_item());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Queue is full"));
        assert_eq!(queue.items.len(), MAX_QUEUE_SIZE); // Size unchanged
    }
}
