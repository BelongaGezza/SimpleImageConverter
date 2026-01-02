// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Conversion history tracking for Simple Image Converter GUI
//!
//! This module tracks recent conversions for user reference and quick access
//! to previously converted files.

#![allow(dead_code)] // Many items reserved for future use

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Conversion history structure
///
/// Tracks recent conversion operations with metadata for display and access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionHistory {
    /// History entries (most recent first)
    pub entries: Vec<ConversionEntry>,
    /// Maximum number of entries to keep
    pub max_entries: usize,
}

/// A single conversion history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionEntry {
    /// Timestamp when conversion was performed
    pub timestamp: DateTime<Utc>,
    /// Source file path
    pub source_path: PathBuf,
    /// Output file path
    pub output_path: PathBuf,
    /// Input format (detected)
    pub input_format: String,
    /// Output format (selected)
    pub output_format: String,
    /// Whether conversion was successful
    pub success: bool,
    /// Error message if conversion failed
    pub error: Option<String>,
}

impl Default for ConversionHistory {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 50,
        }
    }
}

impl ConversionHistory {
    /// Create a new empty history
    #[allow(dead_code)] // Reserved for future use
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries: max_entries.clamp(10, 1000),
        }
    }

    /// Add a new conversion entry
    ///
    /// The entry is added to the front (most recent first).
    /// If the history exceeds max_entries, oldest entries are removed.
    pub fn add_entry(&mut self, entry: ConversionEntry) {
        self.entries.insert(0, entry);
        // Trim to max_entries
        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }

    /// Clear all history entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get recent entries (most recent first)
    ///
    /// Returns up to `limit` most recent entries.
    #[allow(dead_code)] // Reserved for future use
    pub fn recent_entries(&self, limit: usize) -> Vec<&ConversionEntry> {
        self.entries.iter().take(limit).collect()
    }

    /// Get all entries (most recent first)
    #[allow(dead_code)] // Reserved for future use
    pub fn all_entries(&self) -> &[ConversionEntry] {
        &self.entries
    }

    /// Remove a specific entry by index
    ///
    /// Returns `true` if the entry was found and removed.
    pub fn remove_entry(&mut self, index: usize) -> bool {
        if index < self.entries.len() {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }
}

impl ConversionEntry {
    /// Create a new conversion entry
    pub fn new(
        source_path: PathBuf,
        output_path: PathBuf,
        input_format: String,
        output_format: String,
        success: bool,
        error: Option<String>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            source_path,
            output_path,
            input_format,
            output_format,
            success,
            error,
        }
    }

    /// Format timestamp for display
    pub fn formatted_timestamp(&self) -> String {
        self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Get source filename for display
    pub fn source_filename(&self) -> String {
        self.source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    /// Get output filename for display
    #[allow(dead_code)] // Reserved for future use
    pub fn output_filename(&self) -> String {
        self.output_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entry() -> ConversionEntry {
        ConversionEntry::new(
            PathBuf::from("test.png"),
            PathBuf::from("test.jpg"),
            "PNG".to_string(),
            "JPEG".to_string(),
            true,
            None,
        )
    }

    #[test]
    fn test_history_add_entry() {
        let mut history = ConversionHistory::new(10);
        history.add_entry(create_test_entry());
        assert_eq!(history.entries.len(), 1);
    }

    #[test]
    fn test_history_max_entries() {
        let mut history = ConversionHistory::new(10);
        for _ in 0..20 {
            history.add_entry(create_test_entry());
        }
        assert_eq!(history.entries.len(), 10);
    }

    #[test]
    fn test_history_clear() {
        let mut history = ConversionHistory::new(10);
        history.add_entry(create_test_entry());
        history.add_entry(create_test_entry());
        assert_eq!(history.entries.len(), 2);

        history.clear();
        assert_eq!(history.entries.len(), 0);
    }

    #[test]
    fn test_history_recent_entries() {
        let mut history = ConversionHistory::new(10);
        for _ in 0..5 {
            history.add_entry(create_test_entry());
        }

        let recent = history.recent_entries(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_entry_formatted_timestamp() {
        let entry = create_test_entry();
        let formatted = entry.formatted_timestamp();
        assert!(!formatted.is_empty());
    }
}
