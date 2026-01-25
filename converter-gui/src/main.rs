// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Simple Image Converter GUI Application
//!
//! This is the main entry point for the GUI application using egui/eframe.

mod app;
mod batch_queue;
mod conversion;
mod error_messages;
mod format_helpers;
mod history;
mod settings;
mod ui;
mod utils;

#[cfg(feature = "viewer-3d")]
pub mod preview_3d;

use app::ConverterApp;
use settings::AppSettings;

fn main() -> eframe::Result<()> {
    // Load settings up-front so we can apply window sizing before the first frame.
    // If settings cannot be loaded, fall back to defaults.
    let settings = AppSettings::load().unwrap_or_else(|e| {
        eprintln!("Failed to load settings: {e}");
        AppSettings::default()
    });

    // Configure native options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([settings.window_width, settings.window_height])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Simple Image Converter"),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Simple Image Converter",
        options,
        Box::new(move |_cc| {
            // Setup custom styles if needed
            Box::new(ConverterApp::with_settings(settings.clone()))
        }),
    )
}
