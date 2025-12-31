// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Settings UI panel for Simple Image Converter GUI
//!
//! This module provides the UI for viewing and editing application settings.

use crate::app::{AutoSaveStatus, ConverterApp};
use crate::settings::AppSettings;
use egui::{Color32, RichText, Ui};

/// Render the settings UI panel
///
/// Displays current settings with options to edit and save.
pub fn render_settings_panel(ui: &mut Ui, app: &mut ConverterApp) {
    ui.heading("Settings");

    // Auto-save status indicator
    ui.horizontal(|ui| {
        match app.settings_auto_save.status {
            AutoSaveStatus::Idle => {
                // No indicator when idle
            }
            AutoSaveStatus::Pending => {
                ui.label(
                    RichText::new("Saving...")
                        .small()
                        .color(Color32::GRAY)
                        .italics(),
                );
            }
            AutoSaveStatus::Saving => {
                ui.label(
                    RichText::new("Saving...")
                        .small()
                        .color(Color32::from_rgb(100, 150, 255)),
                );
            }
            AutoSaveStatus::Saved => {
                ui.label(
                    RichText::new("✓ Saved")
                        .small()
                        .color(Color32::from_rgb(50, 200, 50)),
                );
            }
            AutoSaveStatus::Error => {
                ui.label(
                    RichText::new("✗ Save failed")
                        .small()
                        .color(Color32::from_rgb(200, 50, 50)),
                );
            }
        }
    });

    ui.add_space(10.0);

    let mut should_browse_dir = false;

    if let Some(ref mut settings) = app.settings {
        // General settings
        ui.collapsing("General", |ui| {
            ui.add_space(5.0);

            // Default output directory
            ui.horizontal(|ui| {
                ui.label("Default Output Directory:");
                if let Some(ref dir) = settings.default_output_directory {
                    ui.label(dir.to_string_lossy().chars().take(50).collect::<String>());
                } else {
                    ui.label(
                        RichText::new("(Use source file directory)")
                            .italics()
                            .color(egui::Color32::GRAY),
                    );
                }
                if ui.button("Browse...")
                    .on_hover_text("Select the default directory for converted files")
                    .clicked()
                {
                    should_browse_dir = true;
                }
            });

            ui.add_space(10.0);

            // Default quality
            ui.horizontal(|ui| {
                ui.label("Default Quality:")
                    .on_hover_text("Default image quality setting for lossy formats (1-100). Higher values = better quality but larger files.");
                let mut quality = settings.default_quality;
                let response = ui.add(egui::Slider::new(&mut quality, 1..=100));
                ui.label(format!("{}", quality));
                if response.changed() {
                    settings.default_quality = quality;
                    app.settings_auto_save.mark_changed();
                }
            });

            ui.add_space(10.0);

            // Show advanced options
            let mut show_advanced = settings.show_advanced_options;
            let response = ui.checkbox(&mut show_advanced, "Show Advanced Options by Default")
                .on_hover_text("Automatically expand the Advanced Options section when a file is selected");
            if response.changed() {
                settings.show_advanced_options = show_advanced;
                app.settings_auto_save.mark_changed();
            }
        });

        ui.add_space(10.0);

        // Conversion settings
        ui.collapsing("Conversion", |ui| {
            ui.add_space(5.0);

            // Conversion history
            let mut history_enabled = settings.conversion_history_enabled;
            let response = ui.checkbox(&mut history_enabled, "Enable Conversion History")
                .on_hover_text("Track conversion history for easy access to previously converted files");
            if response.changed() {
                settings.conversion_history_enabled = history_enabled;
                app.settings_auto_save.mark_changed();
            }

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Max History Entries:")
                    .on_hover_text("Maximum number of conversion history entries to keep. Older entries are automatically removed.");
                let mut max_entries = settings.max_history_entries;
                let response = ui.add(egui::Slider::new(&mut max_entries, 10..=1000));
                ui.label(format!("{}", max_entries));
                if response.changed() {
                    settings.max_history_entries = max_entries;
                    app.settings_auto_save.mark_changed();
                }
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
            // Manual save button (still available as backup)
            if ui
                .button("Save")
                .on_hover_text("Manually save settings to disk (settings are also auto-saved)")
                .clicked()
            {
                if let Some(ref settings) = app.settings {
                    app.settings_auto_save.set_saving();
                    if let Err(e) = settings.save() {
                        app.settings_auto_save.set_error();
                        app.add_message(
                            format!("Failed to save settings: {}", e),
                            crate::app::MessageType::Error,
                        );
                    } else {
                        app.settings_auto_save.set_saved();
                        app.add_message(
                            "Settings saved successfully".to_string(),
                            crate::app::MessageType::Success,
                        );
                    }
                }
            }

            if ui
                .button("Reset to Defaults")
                .on_hover_text(
                    "Reset all settings to their default values. This action cannot be undone.",
                )
                .clicked()
            {
                app.settings = Some(AppSettings::default());
                app.settings_auto_save.mark_changed();
                app.add_message(
                    "Settings reset to defaults".to_string(),
                    crate::app::MessageType::Info,
                );
            }
        });
    } else {
        ui.label(
            RichText::new("Settings not loaded")
                .italics()
                .color(egui::Color32::GRAY),
        );
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
