// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Settings UI panel for Simple Image Converter GUI
//!
//! This module provides the UI for viewing and editing application settings.

use crate::app::ConverterApp;
use crate::settings::AppSettings;
use egui::{RichText, Ui};

/// Render the settings UI panel
///
/// Displays current settings with options to edit and save.
pub fn render_settings_panel(ui: &mut Ui, app: &mut ConverterApp) {
    ui.heading("Settings");

    ui.add_space(10.0);

    if let Some(ref mut settings) = app.settings {
        // General settings
        ui.collapsing("General", |ui| {
            ui.add_space(5.0);

            // Default output directory
            ui.horizontal(|ui| {
                ui.label("Default Output Directory:");
                if let Some(ref dir) = settings.default_output_directory {
                    ui.label(
                        dir.to_string_lossy()
                            .chars()
                            .take(50)
                            .collect::<String>(),
                    );
                } else {
                    ui.label(RichText::new("(Use source file directory)").italics().color(egui::Color32::GRAY));
                }
                if ui.button("Browse...").clicked() {
                    // TODO: Open directory picker
                }
            });

            ui.add_space(10.0);

            // Default quality
            ui.horizontal(|ui| {
                ui.label("Default Quality:");
                ui.add(egui::Slider::new(&mut settings.default_quality, 1..=100));
                ui.label(format!("{}", settings.default_quality));
            });

            ui.add_space(10.0);

            // Show advanced options
            ui.checkbox(&mut settings.show_advanced_options, "Show Advanced Options by Default");
        });

        ui.add_space(10.0);

        // Conversion settings
        ui.collapsing("Conversion", |ui| {
            ui.add_space(5.0);

            // Conversion history
            ui.checkbox(
                &mut settings.conversion_history_enabled,
                "Enable Conversion History",
            );

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Max History Entries:");
                ui.add(egui::Slider::new(&mut settings.max_history_entries, 10..=1000));
                ui.label(format!("{}", settings.max_history_entries));
            });
        });

        ui.add_space(10.0);

        // Settings file location
        ui.collapsing("About", |ui| {
            ui.add_space(5.0);
            if let Ok(config_path) = AppSettings::config_path() {
                ui.label("Settings File:");
                ui.label(
                    RichText::new(config_path.to_string_lossy().to_string())
                        .small()
                        .color(egui::Color32::GRAY),
                );
            }
        });

        ui.add_space(20.0);

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                if let Some(ref settings) = app.settings {
                    if let Err(e) = settings.save() {
                        app.add_message(
                            format!("Failed to save settings: {}", e),
                            crate::app::MessageType::Error,
                        );
                    } else {
                        app.add_message(
                            "Settings saved successfully".to_string(),
                            crate::app::MessageType::Success,
                        );
                    }
                }
            }

            if ui.button("Reset to Defaults").clicked() {
                app.settings = Some(AppSettings::default());
                app.add_message(
                    "Settings reset to defaults".to_string(),
                    crate::app::MessageType::Info,
                );
            }
        });
    } else {
        ui.label(RichText::new("Settings not loaded").italics().color(egui::Color32::GRAY));
        if ui.button("Load Settings").clicked() {
            match AppSettings::load() {
                Ok(settings) => {
                    app.settings = Some(settings);
                    app.add_message(
                        "Settings loaded successfully".to_string(),
                        crate::app::MessageType::Success,
                    );
                }
                Err(e) => {
                    app.add_message(
                        format!("Failed to load settings: {}", e),
                        crate::app::MessageType::Error,
                    );
                }
            }
        }
    }
}

