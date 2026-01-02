// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Settings persistence for Simple Image Converter GUI
//!
//! This module handles loading and saving application settings to a TOML file.
//! Settings are stored in platform-specific directories:
//! - Windows: `%APPDATA%\SimpleImageConverter\config.toml`
//! - macOS: `~/Library/Application Support/SimpleImageConverter/config.toml`
//! - Linux: `~/.config/simpleimageconverter/config.toml`

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Application settings structure
///
/// Contains all user preferences and application state that should persist
/// across sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Window dimensions (for restoring window size)
    #[serde(default = "default_window_width")]
    pub window_width: f32,
    #[serde(default = "default_window_height")]
    pub window_height: f32,

    /// Default output directory (empty means use source file directory)
    #[serde(default)]
    pub default_output_directory: Option<PathBuf>,

    /// Default quality setting (1-100) for lossy formats
    #[serde(default = "default_quality")]
    pub default_quality: u8,

    /// Whether to show advanced options by default
    #[serde(default)]
    pub show_advanced_options: bool,

    /// Recent files list (max 10)
    #[serde(default)]
    pub recent_files: Vec<PathBuf>,

    /// Whether conversion history is enabled
    #[serde(default = "default_true")]
    pub conversion_history_enabled: bool,

    /// Maximum number of history entries to keep
    #[serde(default = "default_max_history_entries")]
    pub max_history_entries: usize,

    /// Maximum concurrent conversions for parallel batch processing (1-16)
    /// Default: CPU cores (capped at 8)
    /// None means use default (CPU cores)
    #[serde(default)]
    pub max_concurrent_conversions: Option<usize>,
}

fn default_window_width() -> f32 {
    1000.0
}

fn default_window_height() -> f32 {
    700.0
}

fn default_quality() -> u8 {
    90
}

fn default_true() -> bool {
    true
}

fn default_max_history_entries() -> usize {
    50
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            window_width: default_window_width(),
            window_height: default_window_height(),
            default_output_directory: None,
            default_quality: default_quality(),
            show_advanced_options: false,
            recent_files: Vec::new(),
            conversion_history_enabled: default_true(),
            max_history_entries: default_max_history_entries(),
            max_concurrent_conversions: None, // Use default (CPU cores)
        }
    }
}

impl AppSettings {
    /// Load settings from the configuration file
    ///
    /// If the file doesn't exist or is corrupted, returns default settings.
    ///
    /// # Returns
    ///
    /// `Ok(AppSettings)` if settings loaded successfully, or default settings
    /// if file doesn't exist or is corrupted.
    ///
    /// # Errors
    ///
    /// Returns an error if the config directory cannot be created or accessed.
    #[allow(clippy::result_large_err)]
    pub fn load() -> Result<Self, SettingsError> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // File doesn't exist - return defaults
            return Ok(Self::default());
        }

        // Try to read and parse the file
        let content =
            std::fs::read_to_string(&config_path).map_err(|e| SettingsError::ReadFailed {
                path: config_path.clone(),
                source: e,
            })?;

        // Parse TOML
        let settings: AppSettings = toml::from_str(&content).map_err(|e| {
            // If parsing fails, return defaults (file is corrupted)
            SettingsError::ParseFailed {
                path: config_path,
                source: e,
            }
        })?;

        // Validate settings
        let settings = settings.validate()?;

        Ok(settings)
    }

    /// Save settings to the configuration file
    ///
    /// Creates the config directory if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the config directory cannot be created or
    /// the file cannot be written.
    #[allow(clippy::result_large_err)]
    pub fn save(&self) -> Result<(), SettingsError> {
        let config_path = Self::config_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SettingsError::WriteFailed {
                path: config_path.clone(),
                source: e,
            })?;
        }

        // Validate before saving
        let validated = self.clone().validate()?;

        // Serialize to TOML
        let content = toml::to_string_pretty(&validated)
            .map_err(|e| SettingsError::SerializeFailed { source: e })?;

        // Write to file
        std::fs::write(&config_path, content).map_err(|e| SettingsError::WriteFailed {
            path: config_path.clone(),
            source: e,
        })?;

        // Set file permissions (Unix only) - read/write for owner, read-only for others
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&config_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o644); // rw-r--r--
                let _ = std::fs::set_permissions(&config_path, perms);
                // Note: We ignore errors here as permissions are not critical for functionality
                // and may fail in some environments (e.g., read-only filesystem)
            }
        }

        Ok(())
    }

    /// Get the platform-specific configuration file path
    ///
    /// # Returns
    ///
    /// Path to the configuration file for the current platform.
    ///
    /// # Errors
    ///
    /// Returns an error if the platform-specific directory cannot be determined.
    #[allow(clippy::result_large_err)]
    pub fn config_path() -> Result<PathBuf, SettingsError> {
        let project_dir =
            directories::ProjectDirs::from("com", "SimpleImageConverter", "SimpleImageConverter")
                .ok_or(SettingsError::NoConfigDir)?;

        let config_dir = project_dir.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    /// Validate settings and return corrected version
    ///
    /// Ensures all values are within valid ranges and limits.
    #[allow(clippy::result_large_err)]
    fn validate(self) -> Result<Self, SettingsError> {
        // Validate quality (1-100)
        let default_quality = self.default_quality.clamp(1, 100);

        // Validate window dimensions (minimum sizes)
        let window_width = self.window_width.max(800.0);
        let window_height = self.window_height.max(600.0);

        // Limit recent files to 10
        let recent_files = self.recent_files.into_iter().take(10).collect();

        // Validate max history entries (reasonable limit)
        let max_history_entries = self.max_history_entries.clamp(10, 1000);

        // Validate max concurrent conversions (1-16 range)
        let max_concurrent_conversions = self.max_concurrent_conversions.map(|v| v.clamp(1, 16));

        Ok(Self {
            window_width,
            window_height,
            default_output_directory: self.default_output_directory,
            default_quality,
            show_advanced_options: self.show_advanced_options,
            recent_files,
            conversion_history_enabled: self.conversion_history_enabled,
            max_history_entries,
            max_concurrent_conversions,
        })
    }

    /// Add a file to recent files list
    ///
    /// If the file is already in the list, it's moved to the front.
    /// The list is limited to 10 files.
    #[allow(dead_code)] // Reserved for future use
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Limit to 10
        self.recent_files.truncate(10);
    }
}

/// Settings error types
#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Cannot determine configuration directory")]
    NoConfigDir,

    #[error("Failed to read settings file at {path:?}: {source}")]
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse settings file at {path:?}: {source}")]
    ParseFailed {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("Failed to serialize settings: {source}")]
    SerializeFailed { source: toml::ser::Error },

    #[error("Failed to write settings file at {path:?}: {source}")]
    WriteFailed {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.window_width, 1000.0);
        assert_eq!(settings.window_height, 700.0);
        assert_eq!(settings.default_quality, 90);
        assert_eq!(settings.max_history_entries, 50);
    }

    #[test]
    fn test_settings_validation() {
        let settings = AppSettings {
            default_quality: 150,      // Invalid
            window_width: 500.0,       // Too small
            max_history_entries: 5000, // Too large
            ..Default::default()
        };

        let validated = settings.validate().unwrap();
        assert_eq!(validated.default_quality, 100); // Clamped
        assert_eq!(validated.window_width, 800.0); // Clamped to minimum
        assert_eq!(validated.max_history_entries, 1000); // Clamped to max
    }

    #[test]
    fn test_recent_files_limit() {
        let mut settings = AppSettings::default();
        for i in 0..15 {
            settings.add_recent_file(PathBuf::from(format!("file{}.png", i)));
        }
        assert_eq!(settings.recent_files.len(), 10);
    }

    #[test]
    fn test_recent_files_deduplication() {
        let mut settings = AppSettings::default();
        let path = PathBuf::from("test.png");
        settings.add_recent_file(path.clone());
        settings.add_recent_file(PathBuf::from("other.png"));
        settings.add_recent_file(path.clone());
        assert_eq!(settings.recent_files.len(), 2);
        assert_eq!(settings.recent_files[0], path); // Most recent is first
    }
}
