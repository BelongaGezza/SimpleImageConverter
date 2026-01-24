// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Batch queue UI component for Simple Image Converter GUI
//!
//! This module provides the UI for managing and displaying the batch conversion queue.

use crate::app::{ConverterApp, OutputFormat};
use crate::batch_queue::{BatchItem, BatchItemStatus};
use crate::ui::style;
use egui::{RichText, Ui};
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
        if ui
            .button("Add Files...")
            .on_hover_text("Add one or more files to the batch conversion queue")
            .clicked()
        {
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

        if ui
            .button("Clear Queue")
            .on_hover_text("Remove all items from the batch queue. This action cannot be undone.")
            .clicked()
        {
            app.confirmation_dialog = Some(crate::app::ConfirmationDialog::ClearQueue);
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
            .is_some()
            || app
                .batch_queue
                .as_ref()
                .map(|q| !q.processing_ids.is_empty())
                .unwrap_or(false);
        let is_processing_active = app.batch_processing_state.is_some();
        let is_paused = app.is_batch_processing_paused();

        // Process Queue button
        ui.set_enabled(has_pending && !is_processing);
        if ui
            .button("Process Queue")
            .on_hover_text(if has_pending && !is_processing {
                "Start processing all pending items in the queue"
            } else if is_processing {
                "Processing is already in progress"
            } else {
                "No pending items in queue"
            })
            .clicked()
        {
            if let Err(e) = app.start_batch_processing(ui.ctx().clone()) {
                app.add_message(
                    format!("Cannot start batch processing: {}. Please check that there are items in the queue.", e),
                    crate::app::MessageType::Error,
                );
            } else {
                app.add_message(
                    "Batch processing started".to_string(),
                    crate::app::MessageType::Info,
                );
            }
        }

        // Pause/Resume button (only enabled when processing is active)
        ui.set_enabled(is_processing_active);
        if is_paused {
            if ui
                .button("▶ Resume")
                .on_hover_text("Resume paused batch processing (Press Space)")
                .clicked()
            {
                if let Err(e) = app.resume_batch_processing() {
                    app.add_message(
                        format!("Cannot resume batch processing: {}. Please start batch processing first.", e),
                        crate::app::MessageType::Error,
                    );
                } else {
                    app.add_message(
                        "Batch processing resumed".to_string(),
                        crate::app::MessageType::Info,
                    );
                }
            }
        } else if ui
            .button("⏸ Pause")
            .on_hover_text("Pause batch processing (Press Space)")
            .clicked()
        {
            if let Err(e) = app.pause_batch_processing() {
                app.add_message(
                    format!("Cannot pause batch processing: {}. Please start batch processing first.", e),
                    crate::app::MessageType::Error,
                );
            } else {
                app.add_message(
                    "Batch processing paused".to_string(),
                    crate::app::MessageType::Info,
                );
            }
        }

        // Cancel button (only enabled when processing is active)
        ui.set_enabled(is_processing_active);
        if ui
            .button("⏹ Cancel")
            .on_hover_text("Cancel batch processing. Items currently processing will finish, but pending items will be cancelled (Press Escape)")
            .clicked()
        {
            app.confirmation_dialog = Some(crate::app::ConfirmationDialog::CancelBatchProcessing);
        }
    });

    ui.add_space(style::spacing::STANDARD);

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
                .color(style::colors::ui::PLACEHOLDER_TEXT),
        );
    } else {
        // Use ScrollArea which provides automatic virtual scrolling for performance
        // Only visible items are rendered, so this efficiently handles large queues (100+ items)
        egui::ScrollArea::vertical()
            .max_height(style::scroll::BATCH_QUEUE_MAX_HEIGHT)
            .show(ui, |ui| {
                if let Some(ref queue) = app.batch_queue {
                    // Pre-allocate Vec capacity to reduce allocations for large queues
                    let estimated_removals = queue.items.len() / 10; // Estimate ~10% removals
                    items_to_remove.reserve(estimated_removals);

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

        ui.add_space(style::spacing::STANDARD);

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
        let is_processing_active = app.batch_processing_state.is_some();
        let is_paused = app.is_batch_processing_paused();

        // Optimize string formatting: reuse format strings to reduce allocations
        ui.horizontal(|ui| {
            // Pre-format labels to reduce allocations
            let total_label = format!("Total: {}", stats.total);
            let completed_label = format!("Completed: {}", stats.completed);
            let failed_label = format!("Failed: {}", stats.failed);
            let pending_label = format!("Pending: {}", stats.pending);

            ui.label(total_label);
            ui.separator();
            ui.label(completed_label);
            ui.separator();
            ui.label(failed_label);
            ui.separator();
            ui.label(pending_label);
            if stats.processing > 0 {
                ui.separator();
                // Show concurrent count: "Processing X/Y items"
                let processing_label =
                    format!("Processing {}/{} items", stats.processing, stats.total);
                let processing_color = if is_paused {
                    style::colors::batch_queue::PAUSED
                } else {
                    style::colors::batch_queue::PROCESSING
                };
                ui.label(RichText::new(processing_label).color(processing_color));
            }
        });

        // Show processing status and estimated time (when processing is active)
        if is_processing_active {
            ui.add_space(style::spacing::MEDIUM);
            ui.horizontal(|ui| {
                // Processing status indicator
                if is_paused {
                    ui.label(
                        RichText::new("⏸ Processing paused")
                            .color(style::colors::batch_queue::PAUSED)
                            .strong(),
                    );
                } else if stats.processing > 0 {
                    ui.label(
                        RichText::new("⚙️ Processing...")
                            .color(style::colors::batch_queue::PROCESSING)
                            .strong(),
                    );
                }

                // Show remaining items count
                // Note: Estimated time calculation would require tracking item completion times
                // For now, showing remaining count provides useful feedback
                if stats.processing > 0 && stats.total > 0 {
                    let remaining_count = stats.pending + stats.processing;
                    if remaining_count > 0 {
                        ui.separator();
                        ui.label(
                            RichText::new(format!("{} items remaining", remaining_count))
                                .small()
                                .color(style::colors::ui::SECONDARY_TEXT),
                        );
                    }
                }
            });
        }
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
    let mut result = (false, None);
    ui.group(|ui| {
        ui.vertical(|ui| {
            // File name and format
            ui.horizontal(|ui| {
                // Status icon
                let (icon, color) = match &item.status {
                    BatchItemStatus::Pending => ("⏳", style::colors::batch_queue::PENDING),
                    BatchItemStatus::Processing => ("⚙️", style::colors::batch_queue::PROCESSING),
                    BatchItemStatus::Completed { .. } => {
                        (style::icons::SUCCESS, style::colors::batch_queue::COMPLETED)
                    }
                    BatchItemStatus::Failed { .. } => {
                        (style::icons::ERROR, style::colors::batch_queue::FAILED)
                    }
                    BatchItemStatus::Cancelled => ("⊘", style::colors::batch_queue::CANCELLED),
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

            ui.add_space(style::spacing::MEDIUM);

            // Status and progress
            ui.horizontal(|ui| {
                match &item.status {
                    BatchItemStatus::Pending => {
                        ui.label(
                            RichText::new("Status: Pending")
                                .color(style::colors::batch_queue::PENDING),
                        );
                    }
                    BatchItemStatus::Processing => {
                        ui.label(
                            RichText::new("Status: Processing...")
                                .color(style::colors::batch_queue::PROCESSING),
                        );
                        // Progress bar
                        ui.add(egui::ProgressBar::new(item.progress).show_percentage());
                    }
                    BatchItemStatus::Completed { output_path } => {
                        ui.label(
                            RichText::new("Status: Completed")
                                .color(style::colors::batch_queue::COMPLETED),
                        );
                        if let Some(output_name) = output_path.file_name().and_then(|n| n.to_str())
                        {
                            ui.label(
                                RichText::new(format!("→ {}", output_name))
                                    .small()
                                    .color(style::colors::ui::SECONDARY_TEXT),
                            );
                        }
                    }
                    BatchItemStatus::Failed { error } => {
                        ui.label(
                            RichText::new("Status: Failed")
                                .color(style::colors::batch_queue::FAILED),
                        );
                        ui.label(
                            RichText::new(error)
                                .small()
                                .color(style::colors::batch_queue::FAILED),
                        );
                    }
                    BatchItemStatus::Cancelled => {
                        ui.label(
                            RichText::new("Status: Cancelled")
                                .color(style::colors::batch_queue::CANCELLED),
                        );
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
                    if ui
                        .small_button("Remove")
                        .on_hover_text(if can_remove {
                            "Remove this item from the queue"
                        } else {
                            "Cannot remove item while processing"
                        })
                        .clicked()
                        && can_remove
                    {
                        should_remove = true; // Signal that this item should be removed
                    }

                    ui.set_enabled(can_edit);
                    if ui
                        .small_button("Edit")
                        .on_hover_text(if can_edit {
                            "Edit this item's output format and options"
                        } else {
                            "Cannot edit item while processing"
                        })
                        .clicked()
                        && can_edit
                    {
                        should_edit = true; // Signal that this item should be edited
                    }
                });
            });
            if should_edit {
                result = (false, Some(item.id));
            } else {
                result = (should_remove, None);
            }
        });
    });

    ui.add_space(style::spacing::MEDIUM);
    result
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

                ui.add_space(style::spacing::STANDARD);

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

                ui.add_space(style::spacing::STANDARD);

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

                ui.add_space(style::spacing::STANDARD);

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
                    ui.add_space(style::spacing::STANDARD);
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

                ui.add_space(style::spacing::LARGE);

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

                            // Validate output path directory exists and is writable
                            let output_dir_valid = if let Some(parent) = output_path.parent() {
                                common::validation::validate_directory_path(parent).is_ok()
                            } else {
                                false
                            };

                            // Validate path is not in system directory
                            let not_system_dir =
                                crate::utils::validate_output_path_not_system(&output_path).is_ok();

                            if output_dir_valid && not_system_dir {
                                should_save = true;
                                save_data = Some((
                                    selected_format,
                                    output_path,
                                    quality,
                                    mesh_options.clone(),
                                    None,
                                ));
                            } else {
                                // Store error message to show after closure
                                let error_msg = if !output_dir_valid {
                                    "Invalid output directory or directory does not exist"
                                        .to_string()
                                } else {
                                    "Output path is in a system directory".to_string()
                                };
                                save_data = Some((
                                    selected_format,
                                    output_path,
                                    quality,
                                    mesh_options.clone(),
                                    Some(error_msg),
                                ));
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
                                    priority: crate::batch_queue::ProcessingPriority::Medium, // Preserve existing priority
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
                            format!(
                                "Invalid output path: {}. Please choose a valid path.",
                                error_msg
                            ),
                            crate::app::MessageType::Error,
                        );
                    }
                }
            });
        });
}

/// Add a file to the batch queue with automatic format detection
pub fn add_file_to_batch_queue(app: &mut ConverterApp, file_path: std::path::PathBuf) {
    use common::validation::validate_file_path;
    use img_core::FormatRegistry as ImageFormatRegistry;
    use mesh_core::FormatRegistry as MeshFormatRegistry;

    // Validate file path (security)
    if let Err(e) = validate_file_path(&file_path) {
        use crate::error_messages::format_user_message;
        app.add_message(format_user_message(&e), crate::app::MessageType::Error);
        return;
    }

    // Detect file type
    let file_type = if ImageFormatRegistry::detect_from_path(&file_path).is_ok() {
        crate::app::FileType::Image
    } else if MeshFormatRegistry::detect_from_path(&file_path).is_ok() {
        crate::app::FileType::Mesh
    } else {
        // Get just the filename for the error message (sanitized)
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        app.add_message(
            format!(
                "File type not supported: {}. Please use a supported image or mesh format.",
                filename
            ),
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
                    "No writable image formats available. Please check your installation."
                        .to_string(),
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
                    "No writable mesh formats available. Please check your installation."
                        .to_string(),
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
            "Cannot determine output filename. Please ensure the file has a valid name."
                .to_string(),
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
            priority: crate::batch_queue::ProcessingPriority::Medium,
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
