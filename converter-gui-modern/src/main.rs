// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

mod app;
mod ui;

use app::ModernApp;

fn main() -> eframe::Result<()> {
    // Load settings up-front so we can apply window sizing before the first frame.
    // If settings cannot be loaded, fall back to defaults.
    let settings = converter_gui::settings::AppSettings::load().unwrap_or_else(|e| {
        eprintln!("Failed to load settings: {e}");
        converter_gui::settings::AppSettings::default()
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([settings.window_width, settings.window_height])
            .with_min_inner_size([900.0, 650.0])
            .with_title("Simple Image Converter (Modern)"),
        ..Default::default()
    };

    eframe::run_native(
        "Simple Image Converter (Modern)",
        options,
        Box::new(move |cc| {
            ui::theme::apply_modern_theme(&cc.egui_ctx);
            Box::new(ModernApp::new(settings.clone()))
        }),
    )
}

