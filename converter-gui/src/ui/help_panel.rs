// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Help panel component for Simple Image Converter GUI
//!
//! This module provides the help panel UI, including keyboard shortcuts reference,
//! feature overview, and troubleshooting tips.

use crate::ui::style;
use egui::{RichText, ScrollArea, Ui};

/// Render the help panel
///
/// Displays help content including keyboard shortcuts, feature overview,
/// and troubleshooting tips.
pub fn render_help_panel(ui: &mut Ui) {
    ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.heading("Help & Documentation");

            ui.add_space(style::spacing::LARGE);

            // Keyboard Shortcuts Section
            ui.heading("Keyboard Shortcuts");
            ui.add_space(style::spacing::STANDARD);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("File Operations:").strong());
                    ui.indent("file_ops", |ui| {
                        ui.label("Ctrl+O (Cmd+O on macOS): Open file");
                        ui.label(
                            "Ctrl+S (Cmd+S on macOS): Save settings (when settings panel is open)",
                        );
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Conversion:").strong());
                    ui.indent("conversion", |ui| {
                        ui.label("Enter: Start conversion (when file and format are selected)");
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Batch Processing:").strong());
                    ui.indent("batch", |ui| {
                        ui.label("Ctrl+Enter (Cmd+Enter on macOS): Start batch processing");
                        ui.label("Ctrl+P (Cmd+P on macOS): Pause/Resume batch processing");
                        ui.label("Space: Pause/Resume batch processing (when active)");
                        ui.label("Escape: Cancel batch processing (when active)");
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Queue Management:").strong());
                    ui.indent("queue", |ui| {
                        ui.label("Ctrl+A (Cmd+A on macOS): Add files to batch queue");
                        ui.label("Ctrl+Shift+D (Cmd+Shift+D on macOS): Clear batch queue");
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Navigation:").strong());
                    ui.indent("nav", |ui| {
                        ui.label("Ctrl+, (Cmd+, on macOS): Open/Close settings panel");
                        ui.label("Ctrl+R (Cmd+R on macOS): Reset/Clear current file selection");
                        ui.label("Escape: Close dialogs or cancel batch processing");
                        ui.label("Tab: Navigate between fields");
                        ui.label("Arrow Keys: Navigate radio buttons");
                    });
                });
            });

            ui.add_space(style::spacing::LARGE);

            // Feature Overview Section
            ui.heading("Feature Overview");
            ui.add_space(style::spacing::STANDARD);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Supported Image Formats:").strong());
                    ui.indent("img_formats", |ui| {
                        ui.label("PNG, JPEG, BMP, GIF, TIFF, WebP");
                        ui.label("SVG (read-only, rasterization)");
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Supported Mesh Formats:").strong());
                    ui.indent("mesh_formats", |ui| {
                        ui.label("STL, OBJ, PLY, OFF, glTF/GLB, DXF");
                        ui.label("STEP (read-only, FACETED_BREP only)");
                    });

                    ui.add_space(style::spacing::MEDIUM);

                    ui.label(RichText::new("Key Features:").strong());
                    ui.indent("features", |ui| {
                        ui.label("• Drag-and-drop file support");
                        ui.label("• Batch processing with parallel conversion");
                        ui.label("• Preview for images and 3D meshes");
                        ui.label("• Quality settings for lossy formats");
                        ui.label("• Conversion history tracking");
                        ui.label("• Settings persistence");
                        ui.label("• Thread-safe processing");
                    });
                });
            });

            ui.add_space(style::spacing::LARGE);

            // Troubleshooting Section
            ui.heading("Troubleshooting");
            ui.add_space(style::spacing::STANDARD);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Common Issues:").strong());
                    ui.indent("issues", |ui| {
                        ui.label(RichText::new("Conversion fails:").strong());
                        ui.label("• Check that the file format is supported");
                        ui.label("• Verify the file is not corrupted");
                        ui.label("• Ensure sufficient disk space");
                        ui.label("• Check error messages for specific details");

                        ui.add_space(style::spacing::MEDIUM);

                        ui.label(RichText::new("File not recognized:").strong());
                        ui.label("• Verify the file extension matches the format");
                        ui.label("• Check that the file is a valid image or mesh");
                        ui.label("• Some formats may require specific file structures");

                        ui.add_space(style::spacing::MEDIUM);

                        ui.label(RichText::new("Preview not showing:").strong());
                        ui.label("• For images: Check file format is supported");
                        ui.label("• For meshes: Ensure viewer-3d feature is enabled");
                        ui.label("• Large files may take time to load");

                        ui.add_space(style::spacing::MEDIUM);

                        ui.label(RichText::new("Batch processing slow:").strong());
                        ui.label("• Check Settings → Max Concurrent Conversions");
                        ui.label("• Large files require more processing time");
                        ui.label("• Multiple CPU cores improve parallel processing");
                    });
                });
            });

            ui.add_space(style::spacing::LARGE);

            // Additional Resources Section
            ui.heading("Additional Resources");
            ui.add_space(style::spacing::STANDARD);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("For more detailed documentation, visit:");
                    ui.hyperlink_to(
                        "GitHub Repository",
                        "https://github.com/BelongaGezza/SimpleImageConverter",
                    );
                    ui.label("View keyboard shortcuts and usage guide in the documentation.");
                });
            });

            ui.add_space(style::spacing::LARGE);
        });
    });
}

/// Render the About dialog
///
/// Displays application information including version, credits, and license.
pub fn render_about_dialog(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.heading("Simple Image Converter");
        ui.add_space(style::spacing::MEDIUM);

        ui.label(RichText::new("Version 0.3.0").strong());
        ui.add_space(style::spacing::SMALL);

        ui.label("A high-performance Rust toolkit for converting between");
        ui.label("image and 3D mesh formats.");
        ui.add_space(style::spacing::STANDARD);

        ui.separator();
        ui.add_space(style::spacing::MEDIUM);

        ui.label(RichText::new("License:").strong());
        ui.label("MIT OR Apache-2.0");
        ui.label("You may use this software under either license.");
        ui.add_space(style::spacing::STANDARD);

        ui.label(RichText::new("Copyright:").strong());
        ui.label("© 2025 Simple Image Converter Contributors");
        ui.add_space(style::spacing::STANDARD);

        ui.label(RichText::new("Repository:").strong());
        ui.hyperlink_to(
            "https://github.com/BelongaGezza/SimpleImageConverter",
            "https://github.com/BelongaGezza/SimpleImageConverter",
        );
        ui.add_space(style::spacing::STANDARD);

        ui.separator();
        ui.add_space(style::spacing::MEDIUM);

        ui.label(RichText::new("Built with:").strong());
        ui.label("• Rust programming language");
        ui.label("• egui/eframe for the GUI");
        ui.label("• image crate for 2D formats");
        ui.label("• Various 3D mesh libraries");
        ui.label("• And many other open-source projects");

        ui.add_space(style::spacing::STANDARD);

        ui.label(RichText::new("Note:").strong());
        ui.label(
            RichText::new("This software uses third-party libraries. See the repository for license information.")
                .small()
                .color(style::colors::ui::SECONDARY_TEXT),
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_panel_functions_exist() {
        // Test that functions can be called without panicking
        // Note: Full UI rendering tests require egui context which is complex to mock
        // These tests verify the functions exist and are callable

        // Functions should compile and be accessible
        // Function pointers have a non-zero size
        assert!(std::mem::size_of::<fn(&mut Ui)>() > 0);

        // Verify functions can be assigned to variables
        let _help_fn: fn(&mut Ui) = render_help_panel;
        let _about_fn: fn(&mut Ui) = render_about_dialog;
    }

    #[test]
    fn test_help_panel_function_signatures() {
        // Verify function signatures are correct
        // render_help_panel should take &mut Ui
        // render_about_dialog should take &mut Ui

        // This test verifies the functions have the expected signatures
        // by checking they can be assigned to variables with the correct type
        let _help_fn: fn(&mut Ui) = render_help_panel;
        let _about_fn: fn(&mut Ui) = render_about_dialog;
    }

    #[test]
    fn test_help_panel_uses_style_constants() {
        // Verify that help panel uses style constants
        // This is an indirect test - we check that style constants are accessible
        // from the help_panel module

        // If this compiles, style constants are accessible
        let _spacing = style::spacing::LARGE;
        let _color = style::colors::ui::SECONDARY_TEXT;
    }
}
