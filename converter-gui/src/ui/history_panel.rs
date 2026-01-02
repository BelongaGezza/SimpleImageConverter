// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Conversion history UI panel for Simple Image Converter GUI
//!
//! This module provides the UI for displaying and managing conversion history.

use crate::app::ConverterApp;
use crate::history::ConversionEntry;
use crate::ui::style;
use common::validation::validate_file_path;
use egui::{RichText, Ui};

/// Render the conversion history UI panel
///
/// Displays recent conversion operations with options to clear history
/// and open output files.
pub fn render_history_panel(ui: &mut Ui, app: &mut ConverterApp) {
    ui.heading("Conversion History");

    ui.add_space(style::spacing::STANDARD);

    let history_empty = app
        .history
        .as_ref()
        .map(|h| h.entries.is_empty())
        .unwrap_or(true);

    if history_empty {
        ui.label(
            RichText::new("No conversion history")
                .italics()
                .color(style::colors::ui::PLACEHOLDER_TEXT),
        );
    } else {
        // Clear history button with confirmation
        ui.horizontal(|ui| {
            if ui
                .button("Clear History")
                .on_hover_text(
                    "Remove all entries from conversion history. This action cannot be undone.",
                )
                .clicked()
            {
                app.confirmation_dialog = Some(crate::app::ConfirmationDialog::ClearHistory);
            }
        });

        ui.add_space(style::spacing::STANDARD);

        // History entries list
        let mut entries_to_remove = Vec::new();
        let mut open_output_path: Option<std::path::PathBuf> = None;
        let mut open_output_filename = String::new();

        if let Some(ref history) = app.history {
            egui::ScrollArea::vertical()
                .max_height(style::scroll::BATCH_QUEUE_MAX_HEIGHT)
                .show(ui, |ui| {
                    for (index, entry) in history.entries.iter().enumerate() {
                        let (should_remove, should_open) = render_history_entry(ui, entry, index);
                        if should_remove {
                            entries_to_remove.push(index);
                        }
                        if should_open {
                            open_output_path = Some(entry.output_path.clone());
                            open_output_filename = entry.output_filename();
                        }
                    }
                });
        }

        // Handle opening output file (outside the closure to avoid borrow issues)
        if let Some(output_path) = open_output_path {
            // Security: Validate path before opening (defense-in-depth)
            // Even though paths come from trusted source (previous conversions),
            // validation adds an extra security layer
            if let Err(e) = validate_file_path(&output_path) {
                app.add_message(
                    format!("Cannot open file: {}", e),
                    crate::app::MessageType::Error,
                );
            } else if output_path.exists() {
                match open::that(&output_path) {
                    Ok(_) => {
                        app.add_message(
                            format!("Opened: {}", open_output_filename),
                            crate::app::MessageType::Info,
                        );
                    }
                    Err(e) => {
                        app.add_message(
                            format!("Failed to open file: {}", e),
                            crate::app::MessageType::Error,
                        );
                    }
                }
            } else {
                app.add_message(
                    format!("Output file not found: {}", open_output_filename),
                    crate::app::MessageType::Error,
                );
            }
        }

        // Remove entries after iteration (in reverse order to maintain indices)
        for &index in entries_to_remove.iter().rev() {
            if let Some(ref mut history) = app.history {
                history.remove_entry(index);
            }
        }
    }
}

/// Render a single history entry
///
/// Returns a tuple: (should_remove, should_open_output)
/// - `should_remove`: true if the entry should be removed
/// - `should_open_output`: true if the output file should be opened
fn render_history_entry(ui: &mut Ui, entry: &ConversionEntry, _index: usize) -> (bool, bool) {
    let mut should_remove = false;
    let mut should_open = false;

    ui.group(|ui| {
        ui.vertical(|ui| {
            // File conversion info
            ui.horizontal(|ui| {
                // Success/failure status indicator (not a checkbox - just shows status)
                let (icon, color) = if entry.success {
                    (style::icons::SUCCESS, style::colors::message::SUCCESS)
                } else {
                    (style::icons::ERROR, style::colors::message::ERROR)
                };
                // Use a label with tooltip to clarify this is a status indicator
                ui.label(RichText::new(icon).size(16.0).color(color))
                    .on_hover_text(if entry.success {
                        "Conversion successful"
                    } else {
                        "Conversion failed"
                    });

                // Source and output
                ui.label(RichText::new(entry.source_filename()).strong());
                ui.label("→");
                ui.label(RichText::new(entry.output_format.clone()).small());
            });

            ui.add_space(style::spacing::MEDIUM);

            // Timestamp
            ui.label(
                RichText::new(entry.formatted_timestamp())
                    .small()
                    .color(style::colors::ui::SECONDARY_TEXT),
            );

            // Error message if failed
            if !entry.success {
                if let Some(ref error) = entry.error {
                    ui.label(
                        RichText::new(error)
                            .small()
                            .color(style::colors::message::ERROR),
                    );
                }
            }

            // Actions
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Add padding to the right of buttons (left side in RTL layout)
                    ui.add_space(style::spacing::STANDARD);

                    if entry.success
                        && ui
                            .small_button("Open Output")
                            .on_hover_text(
                                "Open the converted output file in the default application",
                            )
                            .clicked()
                    {
                        should_open = true; // Signal that output should be opened
                    }
                    if ui
                        .small_button("Remove")
                        .on_hover_text("Remove this entry from conversion history")
                        .clicked()
                    {
                        should_remove = true; // Signal that this entry should be removed
                    }
                });
            });
        });
    });

    ui.add_space(style::spacing::MEDIUM);
    (should_remove, should_open)
}
