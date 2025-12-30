// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Batch queue UI component for Simple Image Converter GUI
//!
//! This module provides the UI for managing and displaying the batch conversion queue.

use crate::app::{ConverterApp, OutputFormat};
use crate::batch_queue::{BatchItem, BatchItemStatus};
use egui::{Color32, RichText, Ui};
use rfd;
use std::path::PathBuf;

/// Type alias for save data tuple to reduce complexity
type SaveData = (
    OutputFormat,
    PathBuf,
    u8,
    Option<crate::batch_queue::MeshOptions>,
    Option<String>,
);

/// Render the batch queue UI panel
///
/// Displays the queue of files waiting to be converted, with controls to
/// add files, remove items, clear the queue, and process all items.
///
/// Note: This function does not render a heading, as it's typically called
/// from within a collapsing header that already provides the title.
pub fn render_batch_queue(ui: &mut Ui, app: &mut ConverterApp) {
    // No heading here - the collapsing header in app.rs provides the title

    // Control buttons
    ui.horizontal(|ui| {
        if ui.button("Add Files...").clicked() {
            // Open multi-file selection dialog
            let mut dialog = rfd::FileDialog::new()
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
                .add_filter("All Files", &["*"]);

            // Set directory if available
            if let Some(dir_str) = app.output_directory.to_str() {
                if let Ok(dir_path) = std::path::PathBuf::from(dir_str).canonicalize() {
                    dialog = dialog.set_directory(dir_path);
                }
            }

            if let Some(selected_files) = dialog.pick_files() {
                // Add each selected file to the batch queue
                for file_path in selected_files {
                    add_file_to_batch_queue(app, file_path);
                }
            }
        }

        if ui.button("Clear Queue").clicked() {
            if let Some(ref mut queue) = app.batch_queue {
                queue.clear();
            }
        }

        let has_pending = app
            .batch_queue
            .as_ref()
            .map(|q| q.has_pending())
            .unwrap_or(false);
        let is_processing = app
            .batch_queue
            .as_ref()
            .and_then(|q| q.current_index)
            .is_some();

        ui.set_enabled(has_pending && !is_processing);
        if ui.button("Process Queue").clicked() {
            if let Err(e) = app.start_batch_processing(ui.ctx().clone()) {
                app.add_message(
                    format!("Failed to start batch processing: {}", e),
                    crate::app::MessageType::Error,
                );
            } else {
                app.add_message(
                    "Batch processing started".to_string(),
                    crate::app::MessageType::Info,
                );
            }
        }
    });

    ui.add_space(10.0);

    // Queue items list
    let queue_empty = app
        .batch_queue
        .as_ref()
        .map(|q| q.is_empty())
        .unwrap_or(true);
    let mut items_to_remove = Vec::new();

    if queue_empty {
        ui.label(
            RichText::new("No files in queue")
                .italics()
                .color(Color32::GRAY),
        );
    } else {
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                if let Some(ref queue) = app.batch_queue {
                    for (index, item) in queue.items.iter().enumerate() {
                        let (should_remove, should_edit_id) = render_queue_item(ui, item, index);
                        if should_remove {
                            items_to_remove.push(item.id);
                        }
                        if let Some(edit_id) = should_edit_id {
                            app.editing_queue_item = Some(edit_id);
                        }
                    }
                }
            });

        ui.add_space(10.0);

        // Queue statistics
        let stats = if let Some(ref queue) = app.batch_queue {
            queue.statistics()
        } else {
            crate::batch_queue::QueueStatistics {
                total: 0,
                completed: 0,
                failed: 0,
                pending: 0,
                processing: 0,
            }
        };
        ui.horizontal(|ui| {
            ui.label(format!("Total: {}", stats.total));
            ui.separator();
            ui.label(format!("Completed: {}", stats.completed));
            ui.separator();
            ui.label(format!("Failed: {}", stats.failed));
            ui.separator();
            ui.label(format!("Pending: {}", stats.pending));
            if stats.processing > 0 {
                ui.separator();
                ui.label(
                    RichText::new(format!("Processing: {}", stats.processing))
                        .color(Color32::from_rgb(100, 150, 255)),
                );
            }
        });
    }

    // Remove items after UI rendering (if any)
    if !items_to_remove.is_empty() {
        for item_id in items_to_remove {
            if let Some(ref mut queue) = app.batch_queue {
                queue.remove_item(item_id);
            }
        }
    }
}

/// Render a single queue item
///
/// Returns a tuple: (should_remove, should_edit_id)
/// - should_remove: true if the item should be removed
/// - should_edit_id: Some(id) if the item should be edited, None otherwise
fn render_queue_item(ui: &mut Ui, item: &BatchItem, _index: usize) -> (bool, Option<uuid::Uuid>) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            // File name and format
            ui.horizontal(|ui| {
                // Status icon
                let (icon, color) = match &item.status {
                    BatchItemStatus::Pending => ("⏳", Color32::GRAY),
                    BatchItemStatus::Processing => ("⚙️", Color32::from_rgb(100, 150, 255)),
                    BatchItemStatus::Completed { .. } => ("✓", Color32::from_rgb(50, 200, 50)),
                    BatchItemStatus::Failed { .. } => ("✗", Color32::from_rgb(200, 50, 50)),
                    BatchItemStatus::Cancelled => ("⊘", Color32::GRAY),
                };
                ui.label(RichText::new(icon).size(16.0).color(color));

                // File name
                let filename = item
                    .source_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");
                ui.label(RichText::new(filename).strong());

                ui.label("→");

                // Output format
                let format_name = match &item.output_format {
                    crate::app::OutputFormat::Image(fmt) => format!("{:?}", fmt),
                    crate::app::OutputFormat::Mesh(fmt) => format!("{:?}", fmt),
                };
                ui.label(format_name);
            });

            ui.add_space(5.0);

            // Status and progress
            ui.horizontal(|ui| {
                match &item.status {
                    BatchItemStatus::Pending => {
                        ui.label(RichText::new("Status: Pending").color(Color32::GRAY));
                    }
                    BatchItemStatus::Processing => {
                        ui.label(
                            RichText::new("Status: Processing...")
                                .color(Color32::from_rgb(100, 150, 255)),
                        );
                        // Progress bar
                        ui.add(egui::ProgressBar::new(item.progress).show_percentage());
                    }
                    BatchItemStatus::Completed { output_path } => {
                        ui.label(
                            RichText::new("Status: Completed")
                                .color(Color32::from_rgb(50, 200, 50)),
                        );
                        if let Some(output_name) = output_path.file_name().and_then(|n| n.to_str())
                        {
                            ui.label(
                                RichText::new(format!("→ {}", output_name))
                                    .small()
                                    .color(Color32::GRAY),
                            );
                        }
                    }
                    BatchItemStatus::Failed { error } => {
                        ui.label(
                            RichText::new("Status: Failed").color(Color32::from_rgb(200, 50, 50)),
                        );
                        ui.label(
                            RichText::new(error)
                                .small()
                                .color(Color32::from_rgb(200, 50, 50)),
                        );
                    }
                    BatchItemStatus::Cancelled => {
                        ui.label(RichText::new("Status: Cancelled").color(Color32::GRAY));
                    }
                }
            });

            // Action buttons (Edit and Remove)
            let mut should_remove = false;
            let mut should_edit = false;
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_edit = matches!(item.status, BatchItemStatus::Pending);
                    let can_remove = !matches!(item.status, BatchItemStatus::Processing);

                    ui.set_enabled(can_remove);
                    if ui.small_button("Remove").clicked() && can_remove {
                        should_remove = true; // Signal that this item should be removed
                    }

                    ui.set_enabled(can_edit);
                    if ui.small_button("Edit").clicked() && can_edit {
                        should_edit = true; // Signal that this item should be edited
                    }
                });
            });
            if should_edit {
                return (false, Some(item.id));
            }
            (should_remove, None)
        });
    });

    ui.add_space(5.0);
    (false, None) // Don't remove or edit by default
}

/// Render the queue item editing dialog
///
/// Displays a modal dialog for editing a queue item's output format, path, and options.
pub fn render_edit_dialog(ui: &mut Ui, app: &mut ConverterApp) {
    let Some(editing_id) = app.editing_queue_item else {
        return; // No item being edited
    };

    // Extract item data before the closure to avoid borrowing issues
    let (file_type, output_format, output_path, options, source_path) = {
        let Some(ref queue) = app.batch_queue else {
            app.editing_queue_item = None;
            return;
        };

        let Some(item) = queue.get_item(editing_id) else {
            app.editing_queue_item = None;
            return;
        };

        // Only allow editing pending items
        if !matches!(item.status, BatchItemStatus::Pending) {
            app.editing_queue_item = None;
            return;
        }

        // Clone necessary data
        (
            item.file_type,
            item.output_format,
            item.output_path.clone(),
            item.options.clone(),
            item.source_path.clone(),
        )
    };

    // Create modal dialog
    egui::Window::new("Edit Queue Item")
        .collapsible(false)
        .resizable(true)
        .default_width(500.0)
        .show(ui.ctx(), |ui| {
            ui.vertical(|ui| {
                // Source file (read-only)
                ui.horizontal(|ui| {
                    ui.label("Source File:");
                    let filename = source_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");
                    ui.label(RichText::new(filename).strong());
                });

                ui.add_space(10.0);

                // Output format selection
                ui.label("Output Format:");
                let mut selected_format = output_format;
                let format_changed = match file_type {
                    crate::app::FileType::Image => {
                        let formats = crate::format_helpers::get_writable_image_formats();
                        let mut changed = false;
                        ui.horizontal_wrapped(|ui| {
                            for format in formats {
                                let format_enum = crate::app::OutputFormat::Image(format);
                                let is_selected = matches!(
                                    selected_format,
                                    crate::app::OutputFormat::Image(f) if f == format
                                );
                                let label = crate::format_helpers::get_image_format_name(format);
                                if ui.selectable_label(is_selected, label).clicked() {
                                    selected_format = format_enum;
                                    changed = true;
                                }
                            }
                        });
                        changed
                    }
                    crate::app::FileType::Mesh => {
                        let formats = crate::format_helpers::get_writable_mesh_formats();
                        let mut changed = false;
                        ui.horizontal_wrapped(|ui| {
                            for format in formats {
                                let format_enum = crate::app::OutputFormat::Mesh(format);
                                let is_selected = matches!(
                                    selected_format,
                                    crate::app::OutputFormat::Mesh(f) if f == format
                                );
                                let label = crate::format_helpers::get_mesh_format_name(format);
                                if ui.selectable_label(is_selected, label).clicked() {
                                    selected_format = format_enum;
                                    changed = true;
                                }
                            }
                        });
                        changed
                    }
                };

                ui.add_space(10.0);

                // Output path
                ui.label("Output Path:");
                let mut output_path_str = output_path.to_string_lossy().to_string();
                let output_path_response = ui.text_edit_singleline(&mut output_path_str);
                let output_path_changed = output_path_response.changed();

                ui.horizontal(|ui| {
                    if ui.button("Browse...").clicked() {
                        let mut dialog = rfd::FileDialog::new().set_file_name(
                            output_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("output"),
                        );

                        if let Some(parent) = output_path.parent() {
                            if let Ok(canonical) = parent.canonicalize() {
                                dialog = dialog.set_directory(canonical);
                            }
                        }

                        if let Some(selected_path) = dialog.save_file() {
                            output_path_str = selected_path.to_string_lossy().to_string();
                        }
                    }
                });

                ui.add_space(10.0);

                // Quality (for image formats)
                let mut quality = options.quality;
                let mut quality_changed = false;
                if matches!(file_type, crate::app::FileType::Image) {
                    if let crate::app::OutputFormat::Image(img_fmt) = selected_format {
                        if crate::format_helpers::format_supports_quality(img_fmt) {
                            ui.label("Quality:");
                            let quality_response = ui.add(egui::Slider::new(&mut quality, 1..=100));
                            ui.label(format!("{}", quality));
                            quality_changed = quality_response.changed();
                        }
                    }
                }

                // Mesh options (for mesh formats)
                let mut mesh_options = options.mesh_options.clone();
                let mut mesh_options_changed = false;
                if matches!(file_type, crate::app::FileType::Mesh) {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.label("Mesh Options:");

                    if let Some(ref mut opts) = mesh_options {
                        let mut recalc = opts.recalculate_normals;
                        let recalc_response = ui.checkbox(&mut recalc, "Recalculate Normals");
                        if recalc_response.changed() {
                            opts.recalculate_normals = recalc;
                            mesh_options_changed = true;
                        }

                        let mut validate = opts.validate;
                        let validate_response = ui.checkbox(&mut validate, "Validate Mesh");
                        if validate_response.changed() {
                            opts.validate = validate;
                            mesh_options_changed = true;
                        }
                    }
                }

                ui.add_space(20.0);

                // Dialog buttons - use a mutable flag to track actions outside closure
                let mut should_cancel = false;
                let mut should_save = false;
                let mut save_data: Option<SaveData> = None;

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        should_cancel = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Save").clicked() {
                            // Validate and prepare save data
                            let output_path = PathBuf::from(&output_path_str);

                            // Validate output path
                            match common::validation::validate_file_path(&output_path) {
                                Ok(()) => {
                                    should_save = true;
                                    save_data = Some((
                                        selected_format,
                                        output_path,
                                        quality,
                                        mesh_options.clone(),
                                        None,
                                    ));
                                }
                                Err(e) => {
                                    // Store error message to show after closure
                                    save_data = Some((
                                        selected_format,
                                        output_path,
                                        quality,
                                        mesh_options.clone(),
                                        Some(e.to_string()),
                                    ));
                                }
                            }
                        }
                    });
                });

                // Handle actions after closure to avoid borrowing issues
                if should_cancel {
                    app.editing_queue_item = None;
                }

                if let Some((
                    selected_format_val,
                    output_path_val,
                    quality_val,
                    mesh_options_val,
                    validation_error,
                )) = save_data
                {
                    if should_save {
                        // Update queue item
                        if let Some(ref mut queue) = app.batch_queue {
                            // Update format (this also updates output path extension)
                            if format_changed {
                                queue.update_item_format(editing_id, selected_format_val);
                            }

                            // Update output path if changed
                            if output_path_changed {
                                queue.update_item_output_path(editing_id, output_path_val.clone());
                            }

                            // Update options if changed
                            if quality_changed || mesh_options_changed {
                                let new_options = crate::batch_queue::BatchItemOptions {
                                    quality: quality_val,
                                    mesh_options: mesh_options_val,
                                };
                                queue.update_item_options(editing_id, new_options);
                            }

                            app.add_message(
                                "Queue item updated".to_string(),
                                crate::app::MessageType::Success,
                            );
                        }

                        app.editing_queue_item = None;
                    } else if let Some(error_msg) = validation_error {
                        // Validation failed
                        app.add_message(
                            format!("Invalid output path: {}", error_msg),
                            crate::app::MessageType::Error,
                        );
                    }
                }
            });
        });
}

/// Add a file to the batch queue with automatic format detection
fn add_file_to_batch_queue(app: &mut ConverterApp, file_path: std::path::PathBuf) {
    use common::validation::validate_file_path;
    use img_core::FormatRegistry as ImageFormatRegistry;
    use mesh_core::FormatRegistry as MeshFormatRegistry;

    // Validate file path (security)
    if let Err(e) = validate_file_path(&file_path) {
        app.add_message(
            format!("Invalid file path: {}", e),
            crate::app::MessageType::Error,
        );
        return;
    }

    // Detect file type
    let file_type = if ImageFormatRegistry::detect_from_path(&file_path).is_ok() {
        crate::app::FileType::Image
    } else if MeshFormatRegistry::detect_from_path(&file_path).is_ok() {
        crate::app::FileType::Mesh
    } else {
        app.add_message(
            format!("Unsupported file type: {}", file_path.display()),
            crate::app::MessageType::Error,
        );
        return;
    };

    // Determine output format (default to first writable format for the file type)
    let output_format = match file_type {
        crate::app::FileType::Image => {
            if let Some(first_format) = crate::format_helpers::get_writable_image_formats().first()
            {
                crate::app::OutputFormat::Image(*first_format)
            } else {
                app.add_message(
                    "No writable image formats available".to_string(),
                    crate::app::MessageType::Error,
                );
                return;
            }
        }
        crate::app::FileType::Mesh => {
            if let Some(first_format) = crate::format_helpers::get_writable_mesh_formats().first() {
                crate::app::OutputFormat::Mesh(*first_format)
            } else {
                app.add_message(
                    "No writable mesh formats available".to_string(),
                    crate::app::MessageType::Error,
                );
                return;
            }
        }
    };

    // Generate output path
    let output_path = if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
        let ext = match output_format {
            crate::app::OutputFormat::Image(fmt) => {
                crate::format_helpers::get_format_extension(fmt)
            }
            crate::app::OutputFormat::Mesh(fmt) => {
                crate::format_helpers::get_mesh_format_extension(fmt)
            }
        };
        file_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(format!("{}.{}", stem, ext))
    } else {
        app.add_message(
            "Cannot determine output filename".to_string(),
            crate::app::MessageType::Error,
        );
        return;
    };

    // Create batch item
    let batch_item = crate::batch_queue::BatchItem::new(
        file_path,
        file_type,
        output_format,
        output_path,
        crate::batch_queue::BatchItemOptions {
            quality: app.quality,
            mesh_options: if matches!(file_type, crate::app::FileType::Mesh) {
                Some(crate::batch_queue::MeshOptions {
                    transform: app.mesh_transform,
                    recalculate_normals: app.mesh_recalculate_normals,
                    validate: app.mesh_validate,
                })
            } else {
                None
            },
        },
    );

    // Add to queue
    if let Some(ref mut queue) = app.batch_queue {
        match queue.add_item(batch_item) {
            Ok(()) => {
                app.add_message(
                    "File added to batch queue".to_string(),
                    crate::app::MessageType::Info,
                );
            }
            Err(e) => {
                app.add_message(e, crate::app::MessageType::Error);
            }
        }
    }
}
