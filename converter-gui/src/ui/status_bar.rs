// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Status bar component

use crate::app::{ConverterApp, Status};
use egui::{Color32, Ui};

/// Render status bar
///
/// Displays the current operation status at the bottom of the window:
/// - Ready: Gray
/// - Converting: Blue (with progress indicator if > 30 seconds)
/// - Success: Green (with output path)
/// - Error: Red (with error message)
pub fn render_status_bar(ui: &mut Ui, app: &ConverterApp) {
    ui.horizontal(|ui| {
        ui.separator();

        let (status_text, status_color) = match &app.status {
            Status::Ready => ("Ready".to_string(), Color32::GRAY),
            Status::Converting { start_time } => {
                let elapsed = start_time.elapsed();
                if elapsed.as_secs() > 30 {
                    (
                        format!("Converting... ({} seconds)", elapsed.as_secs()),
                        Color32::from_rgb(0, 100, 255),
                    )
                } else {
                    ("Converting...".to_string(), Color32::from_rgb(0, 100, 255))
                }
            }
            Status::Success { output_path } => {
                // Sanitize path for display
                let path_display = sanitize_path_for_display(output_path);
                (
                    format!("Conversion complete: {}", path_display),
                    Color32::from_rgb(0, 200, 0),
                )
            }
            Status::Error { message } => (message.clone(), Color32::from_rgb(255, 0, 0)),
        };

        ui.label(egui::RichText::new(&status_text).color(status_color));

        // Show progress indicator for long conversions
        if let Status::Converting { start_time } = &app.status {
            let elapsed = start_time.elapsed();
            if elapsed.as_secs() > 30 {
                ui.spinner();
                ui.label(
                    egui::RichText::new("Processing...")
                        .small()
                        .color(Color32::from_rgb(0, 100, 255)),
                );
            }
        }
    });
}

/// Sanitize path for display (remove user directory, truncate)
fn sanitize_path_for_display(path: &std::path::Path) -> String {
    // Try to get relative path from home directory
    if let Ok(home) = std::env::var("USERPROFILE") {
        if let Ok(relative) = path.strip_prefix(&home) {
            let relative_str = relative.display().to_string();
            if relative_str.len() <= 60 {
                return relative_str;
            }
        }
    }

    // Try to get relative path from current directory
    if let Ok(current_dir) = std::env::current_dir() {
        if let Ok(relative) = path.strip_prefix(&current_dir) {
            let relative_str = relative.display().to_string();
            if relative_str.len() <= 60 {
                return relative_str;
            }
        }
    }

    // Truncate if too long
    let path_str = path.display().to_string();
    if path_str.len() > 60 {
        format!("...{}", &path_str[path_str.len() - 57..])
    } else {
        path_str
    }
}
