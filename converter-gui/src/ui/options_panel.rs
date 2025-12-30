// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Options panel component

use crate::app::{ConverterApp, FileType, OutputFormat};
use crate::format_helpers::format_supports_quality;
use common::validation::validate_directory_path;
use egui::Ui;
use mesh_core::CoordinateSystem;

/// Render options panel
///
/// This component provides:
/// - Output filename field (editable)
/// - Output location browser
/// - Quality slider (for lossy image formats only)
/// - Advanced options (collapsible)
pub fn render_options_panel(ui: &mut Ui, app: &mut ConverterApp) {
    ui.group(|ui| {
        ui.heading("Options:");

        // Output filename
        ui.horizontal(|ui| {
            ui.label("Output Filename:");
            ui.text_edit_singleline(&mut app.output_filename);
        });
        ui.add_space(5.0);

        // Output location
        ui.horizontal(|ui| {
            ui.label("Output Location:");

            // Display current directory (truncated if too long)
            let dir_display = if app.output_directory.to_string_lossy().len() > 50 {
                format!(
                    "...{}",
                    &app.output_directory.to_string_lossy()
                        [app.output_directory.to_string_lossy().len() - 47..]
                )
            } else {
                app.output_directory.to_string_lossy().to_string()
            };
            ui.label(&dir_display);

            if ui.button("Browse...").clicked() {
                if let Some(selected_dir) = rfd::FileDialog::new().pick_folder() {
                    // Security: Validate the selected directory using proper directory validation
                    if validate_directory_path(&selected_dir).is_ok() {
                        app.output_directory = selected_dir;
                        app.add_message(
                            format!("Output location set to: {}", dir_display),
                            crate::app::MessageType::Info,
                        );
                    } else {
                        app.add_message(
                            "Invalid output location selected.".to_string(),
                            crate::app::MessageType::Error,
                        );
                    }
                }
            }
        });
        ui.add_space(10.0);

        // Quality slider (only for lossy image formats)
        if let Some(OutputFormat::Image(format)) = app.output_format {
            if format_supports_quality(format) {
                ui.horizontal(|ui| {
                    ui.label(format!("Quality (1-100): {}", app.quality));
                    ui.add(egui::Slider::new(&mut app.quality, 1..=100));
                });
                ui.add_space(10.0);
            }
        }

        // Mesh conversion options (only for mesh files)
        if let Some(FileType::Mesh) = app.detected_file_type {
            ui.separator();
            ui.heading("Mesh Options:");
            ui.add_space(5.0);

            // Transform options (radio buttons)
            ui.label("Coordinate System Transform:");
            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut app.mesh_transform, None, "None")
                    .clicked()
                {
                    app.mesh_transform = None;
                }
                if ui
                    .radio_value(
                        &mut app.mesh_transform,
                        Some((CoordinateSystem::ZUp, CoordinateSystem::YUp)),
                        "Z-up → Y-up",
                    )
                    .clicked()
                {
                    app.mesh_transform = Some((CoordinateSystem::ZUp, CoordinateSystem::YUp));
                }
                if ui
                    .radio_value(
                        &mut app.mesh_transform,
                        Some((CoordinateSystem::YUp, CoordinateSystem::ZUp)),
                        "Y-up → Z-up",
                    )
                    .clicked()
                {
                    app.mesh_transform = Some((CoordinateSystem::YUp, CoordinateSystem::ZUp));
                }
            });
            ui.add_space(5.0);

            // Recalculate normals checkbox
            ui.checkbox(&mut app.mesh_recalculate_normals, "Recalculate Normals");
            ui.add_space(5.0);

            // Validate checkbox
            ui.checkbox(&mut app.mesh_validate, "Validate Mesh");
            ui.add_space(10.0);
        }

        // Advanced options (collapsible)
        ui.collapsing("Advanced Options", |ui| {
            ui.add_space(5.0);

            // Max file size
            ui.horizontal(|ui| {
                ui.label("Max File Size (MB):");
                ui.add(egui::Slider::new(&mut app.max_file_size_mb, 1..=1024));
                ui.label(format!("{} MB", app.max_file_size_mb));
            });

            // Max dimension (images only)
            if let Some(crate::app::FileType::Image) = app.detected_file_type {
                ui.horizontal(|ui| {
                    ui.label("Max Dimension (pixels):");
                    ui.add(egui::Slider::new(&mut app.max_dimension, 1000..=65535));
                    ui.label(format!("{} px", app.max_dimension));
                });
            }

            // Max vertices/faces (meshes only)
            if let Some(crate::app::FileType::Mesh) = app.detected_file_type {
                ui.horizontal(|ui| {
                    ui.label("Max Vertices:");
                    ui.add(egui::Slider::new(&mut app.max_vertices, 1000..=10_000_000));
                    ui.label(format!("{}", app.max_vertices));
                });

                ui.horizontal(|ui| {
                    ui.label("Max Faces:");
                    ui.add(egui::Slider::new(&mut app.max_faces, 1000..=10_000_000));
                    ui.label(format!("{}", app.max_faces));
                });
            }

            // Warning if limits are increased beyond defaults
            if app.max_file_size_mb > 100 {
                ui.label(
                    egui::RichText::new(
                        "⚠ Warning: Large file size limit may cause performance issues.",
                    )
                    .color(egui::Color32::YELLOW),
                );
            }
        });
    });
}
