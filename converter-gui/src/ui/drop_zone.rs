// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! File drop zone component for drag-and-drop file selection

use crate::app::{ConverterApp, FileType, InputFormat};
use crate::ui::style;
use common::error::ConversionError;
use common::validation::validate_file_path;
use egui::{Sense, Stroke, Ui};
use img_core::FormatRegistry as ImageFormatRegistry;
use mesh_core::FormatRegistry as MeshFormatRegistry;
use std::path::PathBuf;

/// Render the file drop zone and handle file selection
///
/// This component provides:
/// - Large drop zone area for drag-and-drop
/// - Click-to-browse file selection
/// - Visual feedback for different states
/// - File type detection (image vs mesh)
/// - Security validation
pub fn render_drop_zone(ui: &mut Ui, app: &mut ConverterApp) {
    // Allocate space for drop zone
    // Smaller height when file is selected, larger when empty
    let available_width = ui.available_width();
    let drop_zone_height = if app.source_file.is_some() {
        60.0 // Compact when file selected
    } else {
        200.0 // Large when empty
    };

    // Allocate space FIRST - this reserves the space in the layout
    // This ensures the drop zone takes up space and doesn't overlap with other elements
    // The cursor is automatically advanced after allocation
    let response = ui.allocate_response(
        egui::vec2(available_width, drop_zone_height),
        Sense::click(),
    );
    let drop_zone_rect = response.rect;

    // Check for drag-over state
    let is_drag_over = ui.ctx().input(|i| {
        !i.raw.hovered_files.is_empty()
            && drop_zone_rect.contains(i.pointer.interact_pos().unwrap_or_default())
    });

    // Check for dropped files
    let dropped_files: Vec<PathBuf> = ui.ctx().input(|i| {
        i.raw
            .dropped_files
            .iter()
            .filter_map(|f| f.path.clone())
            .collect()
    });

    // Handle dropped files
    if !dropped_files.is_empty() {
        if let Some(file_path) = dropped_files.first() {
            handle_file_selection(app, file_path.clone());
        }
    }

    // Visual state based on current selection and drag-over
    let (bg_color, border_color, border_width) = if app.source_file.is_some() {
        // File selected - green border
        (
            style::colors::ui::DROP_ZONE_SELECTED_BG,
            style::colors::ui::DROP_ZONE_SELECTED_BORDER,
            style::border::STANDARD,
        )
    } else if is_drag_over {
        // Drag over - blue border
        (
            style::colors::ui::DROP_ZONE_DRAG_BG,
            style::colors::ui::DROP_ZONE_DRAG_BORDER,
            style::border::STANDARD,
        )
    } else {
        // Empty - light gray with thin border
        (
            style::colors::ui::DROP_ZONE_EMPTY_BG,
            style::colors::ui::DROP_ZONE_EMPTY_BORDER,
            style::border::THIN,
        )
    };

    // Draw drop zone background and border
    ui.painter()
        .rect_filled(drop_zone_rect, style::corner_radius::STANDARD, bg_color);
    ui.painter().rect_stroke(
        drop_zone_rect,
        style::corner_radius::STANDARD,
        Stroke::new(border_width, border_color),
    );

    // Handle click on drop zone
    if response.clicked() {
        // Open file browser
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter(
                "Image Files",
                &[
                    "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                ],
            )
            .add_filter(
                "Mesh Files",
                &[
                    "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                ],
            )
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            handle_file_selection(app, file_path);
        }
    }

    // Draw drop zone content INSIDE the allocated rect
    // Use a slightly smaller rect to account for border/padding
    let content_rect = drop_zone_rect.shrink(4.0);
    ui.allocate_ui_at_rect(content_rect, |ui| {
        if app.source_file.is_some() {
            // Compact display when file selected
            ui.horizontal(|ui| {
                ui.add_space(style::spacing::STANDARD);
                ui.label(egui::RichText::new("📁").size(24.0));
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("File Selected").strong());
                    if let Some(ref file) = app.source_file {
                        let file_name = file
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown file");
                        ui.label(file_name);
                    }
                });
            });
        } else {
            // Full display when empty
            ui.vertical_centered(|ui| {
                ui.add_space(style::spacing::LARGE);
                ui.heading("📁 Drag & Drop File Here");
                ui.add_space(style::spacing::STANDARD);
                ui.label("or click to browse");
                ui.add_space(style::spacing::STANDARD);
                if ui
                    .button("Browse Files...")
                    .on_hover_text(
                        "Open file browser to select an image or mesh file (Keyboard: Ctrl+O)",
                    )
                    .clicked()
                {
                    if let Some(file_path) = rfd::FileDialog::new()
                        .add_filter(
                            "Image Files",
                            &[
                                "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                            ],
                        )
                        .add_filter(
                            "Mesh Files",
                            &[
                                "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                            ],
                        )
                        .add_filter("All Files", &["*"])
                        .pick_file()
                    {
                        handle_file_selection(app, file_path);
                    }
                }
                ui.add_space(style::spacing::LARGE);
            });
        }
    });
}

/// Handle file selection with validation and type detection
///
/// This function is public so it can be called from keyboard shortcuts
pub fn handle_file_selection_internal(app: &mut ConverterApp, file_path: PathBuf) {
    handle_file_selection(app, file_path);
}

/// Handle file selection with validation and type detection
fn handle_file_selection(app: &mut ConverterApp, file_path: PathBuf) {
    // Security: Validate file path
    if let Err(e) = validate_file_path(&file_path) {
        app.add_message(format_user_error(&e), crate::app::MessageType::Error);
        return;
    }

    // Security: Read file with size validation before format detection
    // This prevents DoS attacks from maliciously large files
    use common::io::read_file_bytes_checked;
    use common::limits::ResourceLimits;

    let limits = ResourceLimits::default();
    let file_data = match read_file_bytes_checked(&file_path, &limits) {
        Ok(data) => data,
        Err(e) => {
            app.add_message(format_user_error(&e), crate::app::MessageType::Error);
            return;
        }
    };

    // Detect file type (image vs mesh)
    match detect_file_type(&file_path) {
        Ok(file_type) => {
            app.source_file = Some(file_path.clone());
            app.detected_file_type = Some(file_type);

            // Detect input format with two-stage detection for images (security)
            match file_type {
                FileType::Image => {
                    // Security: Two-stage format detection (extension + magic bytes)
                    match ImageFormatRegistry::detect_two_stage(&file_path, &file_data) {
                        Ok(format) => {
                            app.input_format = Some(InputFormat::Image(format));
                            app.add_message(
                                format!("Image file detected: {:?}", format),
                                crate::app::MessageType::Info,
                            );
                        }
                        Err(e) => {
                            app.add_message(format_user_error(&e), crate::app::MessageType::Error);
                        }
                    }
                }
                FileType::Mesh => {
                    // Note: Mesh formats currently use extension-based detection only
                    // Two-stage detection for mesh formats would require magic bytes support
                    match MeshFormatRegistry::detect_from_path(&file_path) {
                        Ok(format) => {
                            app.input_format = Some(InputFormat::Mesh(format));
                            app.add_message(
                                format!("Mesh file detected: {:?}", format),
                                crate::app::MessageType::Info,
                            );
                        }
                        Err(e) => {
                            app.add_message(format_user_error(&e), crate::app::MessageType::Error);
                        }
                    }
                }
            }

            // Auto-generate output filename (will be updated when format is selected)
            if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
                app.output_filename = stem.to_string();
            }

            // Set default output format based on file type
            match file_type {
                FileType::Image => {
                    // Default to first writable image format (BMP alphabetically)
                    if let Some(first_format) =
                        crate::format_helpers::get_writable_image_formats().first()
                    {
                        app.output_format = Some(crate::app::OutputFormat::Image(*first_format));
                        // Update filename with extension
                        if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
                            let ext = crate::format_helpers::get_format_extension(*first_format);
                            app.output_filename = format!("{}.{}", stem, ext);
                        }
                    }
                }
                FileType::Mesh => {
                    // Default to first writable mesh format (DXF alphabetically)
                    if let Some(first_format) =
                        crate::format_helpers::get_writable_mesh_formats().first()
                    {
                        app.output_format = Some(crate::app::OutputFormat::Mesh(*first_format));
                        // Update filename with extension
                        if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
                            let ext =
                                crate::format_helpers::get_mesh_format_extension(*first_format);
                            app.output_filename = format!("{}.{}", stem, ext);
                        }
                    }
                }
            }

            // Set output directory to same as source file
            if let Some(parent) = file_path.parent() {
                app.output_directory = parent.to_path_buf();
            }
        }
        Err(e) => {
            app.add_message(format_user_error(&e), crate::app::MessageType::Error);
        }
    }
}

/// Detect if a file is an image or mesh format
fn detect_file_type(path: &std::path::Path) -> Result<FileType, ConversionError> {
    // Try image formats first
    if ImageFormatRegistry::detect_from_path(path).is_ok() {
        return Ok(FileType::Image);
    }

    // Try mesh formats
    if MeshFormatRegistry::detect_from_path(path).is_ok() {
        return Ok(FileType::Mesh);
    }

    Err(ConversionError::UnsupportedFormat(format!(
        "Unsupported file type: {}",
        path.display()
    )))
}

/// Format error for user-friendly display
fn format_user_error(error: &ConversionError) -> String {
    crate::error_messages::format_user_message(error)
}
