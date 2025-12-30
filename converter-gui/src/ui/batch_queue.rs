// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Batch queue UI component for Simple Image Converter GUI
//!
//! This module provides the UI for managing and displaying the batch conversion queue.

use crate::app::ConverterApp;
use crate::batch_queue::{BatchItem, BatchItemStatus};
use egui::{Color32, RichText, Ui};
use rfd;

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
                        let should_remove = render_queue_item(ui, item, index);
                        if should_remove {
                            items_to_remove.push(item.id);
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
/// Returns `true` if the item should be removed.
fn render_queue_item(ui: &mut Ui, item: &BatchItem, _index: usize) -> bool {
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

            // Remove button
            let mut should_remove = false;
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_remove = !matches!(item.status, BatchItemStatus::Processing);
                    ui.set_enabled(can_remove);
                    if ui.small_button("Remove").clicked() && can_remove {
                        should_remove = true; // Signal that this item should be removed
                    }
                });
            });
            should_remove
        });
    });

    ui.add_space(5.0);
    false // Don't remove by default
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
        queue.add_item(batch_item);
        app.add_message(
            "File added to batch queue".to_string(),
            crate::app::MessageType::Info,
        );
    }
}
