// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Format selection UI component

use crate::app::{ConverterApp, FileType, OutputFormat};
use crate::format_helpers::{
    get_format_extension, get_image_format_name, get_mesh_format_extension, get_mesh_format_name,
    get_writable_image_formats, get_writable_mesh_formats,
};
use egui::Ui;
use std::path::PathBuf;

/// Render format selection radio buttons
///
/// This component displays radio buttons for selecting the output format.
/// Formats are filtered based on the detected file type (image vs mesh),
/// and read-only formats (SVG, STEP) are excluded.
pub fn render_format_selector(ui: &mut Ui, app: &mut ConverterApp) {
    ui.group(|ui| {
        ui.heading("Output Format:");

        // Only show format selector if a file is selected
        if let Some(file_type) = app.detected_file_type {
            match file_type {
                FileType::Image => {
                    render_image_formats(ui, app);
                }
                FileType::Mesh => {
                    render_mesh_formats(ui, app);
                }
            }
        } else {
            ui.label("Select a file to choose output format");
        }
    });
}

/// Render image format radio buttons
fn render_image_formats(ui: &mut Ui, app: &mut ConverterApp) {
    let formats = get_writable_image_formats();

    // Set default format if none selected
    if app.output_format.is_none() {
        if let Some(first_format) = formats.first() {
            app.output_format = Some(OutputFormat::Image(*first_format));
            update_output_filename(app);
        }
    }

    // Render radio buttons with improved spacing
    for format in &formats {
        let format_name = get_image_format_name(*format);
        let tooltip = format!(
            "Convert to {} format. Click to select this output format. The output filename will be updated automatically.",
            format_name
        );

        let response = ui
            .radio_value(
                &mut app.output_format,
                Some(OutputFormat::Image(*format)),
                format_name,
            )
            .on_hover_text(tooltip);

        if response.changed() {
            update_output_filename(app);
        }
        ui.add_space(crate::ui::style::spacing::SMALL); // Add small spacing between radio buttons
    }
}

/// Render mesh format radio buttons
fn render_mesh_formats(ui: &mut Ui, app: &mut ConverterApp) {
    let formats = get_writable_mesh_formats();

    // Set default format if none selected
    if app.output_format.is_none() {
        if let Some(first_format) = formats.first() {
            app.output_format = Some(OutputFormat::Mesh(*first_format));
            update_output_filename(app);
        }
    }

    // Render radio buttons with improved spacing
    for format in &formats {
        let format_name = get_mesh_format_name(*format);
        let tooltip = format!(
            "Convert to {} format. Click to select this output format. The output filename will be updated automatically.",
            format_name
        );

        let response = ui
            .radio_value(
                &mut app.output_format,
                Some(OutputFormat::Mesh(*format)),
                format_name,
            )
            .on_hover_text(tooltip);

        if response.changed() {
            update_output_filename(app);
        }
        ui.add_space(crate::ui::style::spacing::SMALL); // Add small spacing between radio buttons
    }
}

/// Update output filename extension when format changes
fn update_output_filename(app: &mut ConverterApp) {
    if let Some(ref source_file) = app.source_file {
        if let Some(stem) = source_file.file_stem().and_then(|s| s.to_str()) {
            let extension = match app.output_format {
                Some(OutputFormat::Image(format)) => get_format_extension(format),
                Some(OutputFormat::Mesh(format)) => get_mesh_format_extension(format),
                None => return,
            };

            // Create new filename with new extension
            let mut new_filename = PathBuf::from(stem);
            new_filename.set_extension(extension);

            // Update output filename (just the name, not full path)
            if let Some(name) = new_filename.file_name().and_then(|n| n.to_str()) {
                app.output_filename = name.to_string();
            }
        }
    }
}
