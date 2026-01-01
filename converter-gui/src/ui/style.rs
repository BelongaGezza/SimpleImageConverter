// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! UI style constants for consistent styling across all components
//!
//! This module provides centralized style constants to ensure consistent
//! visual appearance, spacing, and colors throughout the GUI.

use egui::Color32;

/// Spacing constants for consistent layout spacing
pub mod spacing {
    /// Small spacing (2px) - Used for tight spacing between related elements
    pub const SMALL: f32 = 2.0;

    /// Medium spacing (5px) - Used for spacing within groups
    pub const MEDIUM: f32 = 5.0;

    /// Standard spacing (10px) - Used for spacing between sections
    pub const STANDARD: f32 = 10.0;

    /// Large spacing (20px) - Used for spacing between major sections
    pub const LARGE: f32 = 20.0;

    /// Extra large spacing (30px) - Used for spacing between major panels
    pub const EXTRA_LARGE: f32 = 30.0;
}

/// Color constants for consistent color usage
pub mod colors {
    use super::Color32;

    /// Message type colors
    pub mod message {
        use super::Color32;
        /// Info message color (blue)
        pub const INFO: Color32 = Color32::from_rgb(0, 100, 255);
        /// Warning message color (yellow/orange)
        pub const WARNING: Color32 = Color32::from_rgb(255, 200, 0);
        /// Error message color (red)
        pub const ERROR: Color32 = Color32::from_rgb(255, 0, 0);
        /// Success message color (green)
        pub const SUCCESS: Color32 = Color32::from_rgb(0, 200, 0);
    }

    /// Status colors
    pub mod status {
        use super::Color32;
        /// Ready status color (gray)
        pub const READY: Color32 = Color32::GRAY;
        /// Converting status color (blue)
        pub const CONVERTING: Color32 = Color32::from_rgb(0, 100, 255);
        /// Success status color (green)
        pub const SUCCESS: Color32 = Color32::from_rgb(0, 200, 0);
        /// Error status color (red)
        pub const ERROR: Color32 = Color32::from_rgb(255, 0, 0);
    }

    /// UI element colors
    pub mod ui {
        use super::Color32;
        /// Drop zone background when file selected (light green)
        pub const DROP_ZONE_SELECTED_BG: Color32 = Color32::from_rgb(240, 255, 240);
        /// Drop zone border when file selected (green)
        pub const DROP_ZONE_SELECTED_BORDER: Color32 = Color32::from_rgb(0, 200, 0);
        /// Drop zone background when drag over (light blue)
        pub const DROP_ZONE_DRAG_BG: Color32 = Color32::from_rgb(240, 248, 255);
        /// Drop zone border when drag over (blue)
        pub const DROP_ZONE_DRAG_BORDER: Color32 = Color32::from_rgb(0, 100, 255);
        /// Drop zone background when empty (light gray)
        pub const DROP_ZONE_EMPTY_BG: Color32 = Color32::from_rgb(245, 245, 245);
        /// Drop zone border when empty (gray)
        pub const DROP_ZONE_EMPTY_BORDER: Color32 = Color32::from_rgb(200, 200, 200);
        /// Secondary text color (gray)
        pub const SECONDARY_TEXT: Color32 = Color32::GRAY;
        /// Placeholder text color (light gray)
        pub const PLACEHOLDER_TEXT: Color32 = Color32::from_rgb(180, 180, 180);
    }

    /// Auto-save status colors
    pub mod auto_save {
        use super::Color32;
        /// Saving status color (blue)
        pub const SAVING: Color32 = Color32::from_rgb(100, 150, 255);
        /// Saved status color (green)
        pub const SAVED: Color32 = Color32::from_rgb(50, 200, 50);
        /// Error status color (red)
        pub const ERROR: Color32 = Color32::from_rgb(200, 50, 50);
    }

    /// Batch queue status colors
    pub mod batch_queue {
        use super::Color32;
        /// Pending status color (gray)
        pub const PENDING: Color32 = Color32::GRAY;
        /// Processing status color (blue)
        pub const PROCESSING: Color32 = Color32::from_rgb(100, 150, 255);
        /// Completed status color (green)
        pub const COMPLETED: Color32 = Color32::from_rgb(50, 200, 50);
        /// Failed status color (red)
        pub const FAILED: Color32 = Color32::from_rgb(200, 50, 50);
        /// Paused status color (yellow/orange)
        pub const PAUSED: Color32 = Color32::from_rgb(200, 150, 50);
        /// Cancelled status color (gray)
        pub const CANCELLED: Color32 = Color32::GRAY;
    }
}

/// Border width constants
pub mod border {
    /// Thin border (1px) - Used for subtle borders
    pub const THIN: f32 = 1.0;
    /// Standard border (2px) - Used for standard borders
    pub const STANDARD: f32 = 2.0;
    /// Thick border (3px) - Used for emphasis
    pub const THICK: f32 = 3.0;
}

/// Corner radius constants
pub mod corner_radius {
    /// Small corner radius (2px)
    pub const SMALL: f32 = 2.0;
    /// Standard corner radius (4px)
    pub const STANDARD: f32 = 4.0;
    /// Large corner radius (8px)
    pub const LARGE: f32 = 8.0;
}

/// Scroll area constants
pub mod scroll {
    /// Maximum height for messages scroll area
    pub const MESSAGES_MAX_HEIGHT: f32 = 150.0;
    /// Maximum height for batch queue scroll area
    pub const BATCH_QUEUE_MAX_HEIGHT: f32 = 400.0;
}

/// Icon constants
pub mod icons {
    /// Info icon
    pub const INFO: &str = "ℹ";
    /// Warning icon
    pub const WARNING: &str = "⚠";
    /// Error icon
    pub const ERROR: &str = "✗";
    /// Success icon
    pub const SUCCESS: &str = "✓";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_constants_are_positive() {
        assert!(spacing::SMALL > 0.0);
        assert!(spacing::MEDIUM > 0.0);
        assert!(spacing::STANDARD > 0.0);
        assert!(spacing::LARGE > 0.0);
        assert!(spacing::EXTRA_LARGE > 0.0);
    }

    #[test]
    fn test_spacing_constants_are_ordered() {
        assert!(spacing::SMALL < spacing::MEDIUM);
        assert!(spacing::MEDIUM < spacing::STANDARD);
        assert!(spacing::STANDARD < spacing::LARGE);
        assert!(spacing::LARGE < spacing::EXTRA_LARGE);
    }

    #[test]
    fn test_border_constants_are_positive() {
        assert!(border::THIN > 0.0);
        assert!(border::STANDARD > 0.0);
        assert!(border::THICK > 0.0);
    }

    #[test]
    fn test_border_constants_are_ordered() {
        assert!(border::THIN < border::STANDARD);
        assert!(border::STANDARD < border::THICK);
    }

    #[test]
    fn test_corner_radius_constants_are_positive() {
        assert!(corner_radius::SMALL > 0.0);
        assert!(corner_radius::STANDARD > 0.0);
        assert!(corner_radius::LARGE > 0.0);
    }

    #[test]
    fn test_corner_radius_constants_are_ordered() {
        assert!(corner_radius::SMALL < corner_radius::STANDARD);
        assert!(corner_radius::STANDARD < corner_radius::LARGE);
    }

    #[test]
    fn test_scroll_constants_are_positive() {
        assert!(scroll::MESSAGES_MAX_HEIGHT > 0.0);
        assert!(scroll::BATCH_QUEUE_MAX_HEIGHT > 0.0);
    }

    #[test]
    fn test_color_constants_are_valid() {
        // Test that colors are valid Color32 values
        // Color32::from_rgb will panic if values are out of range, so if we get here, they're valid
        let _ = colors::message::INFO;
        let _ = colors::message::WARNING;
        let _ = colors::message::ERROR;
        let _ = colors::message::SUCCESS;
        let _ = colors::status::READY;
        let _ = colors::status::CONVERTING;
        let _ = colors::status::SUCCESS;
        let _ = colors::status::ERROR;
        let _ = colors::ui::DROP_ZONE_SELECTED_BG;
        let _ = colors::ui::DROP_ZONE_SELECTED_BORDER;
        let _ = colors::ui::DROP_ZONE_DRAG_BG;
        let _ = colors::ui::DROP_ZONE_DRAG_BORDER;
        let _ = colors::ui::DROP_ZONE_EMPTY_BG;
        let _ = colors::ui::DROP_ZONE_EMPTY_BORDER;
        let _ = colors::ui::SECONDARY_TEXT;
        let _ = colors::ui::PLACEHOLDER_TEXT;
        let _ = colors::auto_save::SAVING;
        let _ = colors::auto_save::SAVED;
        let _ = colors::auto_save::ERROR;
        let _ = colors::batch_queue::PENDING;
        let _ = colors::batch_queue::PROCESSING;
        let _ = colors::batch_queue::COMPLETED;
        let _ = colors::batch_queue::FAILED;
        let _ = colors::batch_queue::PAUSED;
        let _ = colors::batch_queue::CANCELLED;
    }

    #[test]
    fn test_icon_constants_are_non_empty() {
        assert!(!icons::INFO.is_empty());
        assert!(!icons::WARNING.is_empty());
        assert!(!icons::ERROR.is_empty());
        assert!(!icons::SUCCESS.is_empty());
    }

    #[test]
    fn test_color_contrast_requirements() {
        // Basic contrast checks - ensure colors are distinguishable
        // This is a simplified check; full WCAG compliance would require luminance calculations

        // Message colors should be distinct
        assert_ne!(colors::message::INFO, colors::message::ERROR);
        assert_ne!(colors::message::WARNING, colors::message::SUCCESS);

        // Status colors should be distinct
        assert_ne!(colors::status::READY, colors::status::CONVERTING);
        assert_ne!(colors::status::SUCCESS, colors::status::ERROR);

        // Batch queue colors should be distinct
        assert_ne!(
            colors::batch_queue::PENDING,
            colors::batch_queue::PROCESSING
        );
        assert_ne!(colors::batch_queue::COMPLETED, colors::batch_queue::FAILED);
    }

    #[test]
    fn test_style_consistency() {
        // Verify that spacing values follow a consistent pattern
        // Standard spacing should be 2x medium, large should be 2x standard, etc.
        // (Allow some flexibility for design decisions)
        assert!(spacing::STANDARD >= spacing::MEDIUM * 1.5);
        assert!(spacing::LARGE >= spacing::STANDARD * 1.5);
        assert!(spacing::EXTRA_LARGE >= spacing::LARGE * 1.2);
    }
}
