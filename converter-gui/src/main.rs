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

fn main() -> eframe::Result<()> {
    // Configure native options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Simple Image Converter"),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Simple Image Converter",
        options,
        Box::new(|_cc| {
            // Setup custom styles if needed
            Box::new(ConverterApp::default())
        }),
    )
}
