// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Options panel component

use crate::app::{ConverterApp, FileType, OutputFormat};
use crate::format_helpers::format_supports_quality;
use crate::ui::style;
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
            ui.label("Output Filename:")
                .on_hover_text("The name of the output file. The extension will be updated automatically when you change the format.");
            ui.text_edit_singleline(&mut app.output_filename)
                .on_hover_text("Edit the output filename. The extension will be updated automatically when you change the format.");
        });
        ui.add_space(style::spacing::MEDIUM);

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

            if ui
                .button("Browse...")
                .on_hover_text("Select the output directory where converted files will be saved")
                .clicked()
            {
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
        ui.add_space(style::spacing::STANDARD);

        // Quality slider (only for lossy image formats)
        if let Some(OutputFormat::Image(format)) = app.output_format {
            if format_supports_quality(format) {
                ui.horizontal(|ui| {
                    ui.label(format!("Quality (1-100): {}", app.quality))
                        .on_hover_text("Image quality setting. Higher values = better quality but larger file size. Lower values = smaller files but reduced quality.");
                    ui.add(egui::Slider::new(&mut app.quality, 1..=100))
                        .on_hover_text("Adjust image quality (1-100). Higher = better quality, larger files. Lower = smaller files, reduced quality.");
                });
                ui.add_space(style::spacing::STANDARD);
            }
        }

        // Mesh conversion options (only for mesh files)
        if let Some(FileType::Mesh) = app.detected_file_type {
            ui.separator();
            ui.heading("Mesh Options:");
            ui.add_space(style::spacing::MEDIUM);

            // Transform options (radio buttons)
            ui.label("Coordinate System Transform:")
                .on_hover_text("Transform the mesh coordinate system during conversion. Useful when converting between different 3D software formats.");
            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut app.mesh_transform, None, "None")
                    .on_hover_text("No coordinate system transformation")
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
                    .on_hover_text("Transform from Z-up to Y-up coordinate system")
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
                    .on_hover_text("Transform from Y-up to Z-up coordinate system")
                    .clicked()
                {
                    app.mesh_transform = Some((CoordinateSystem::YUp, CoordinateSystem::ZUp));
                }
            });
            ui.add_space(style::spacing::MEDIUM);

            // Recalculate normals checkbox
            ui.checkbox(&mut app.mesh_recalculate_normals, "Recalculate Normals")
                .on_hover_text("Recalculate vertex normals for the mesh. This may improve rendering quality.");
            ui.add_space(style::spacing::MEDIUM);

            // Validate checkbox
            ui.checkbox(&mut app.mesh_validate, "Validate Mesh")
                .on_hover_text("Validate mesh integrity before conversion. This checks for common mesh errors.");
            ui.add_space(style::spacing::STANDARD);
        }

        // Advanced options (collapsible)
        ui.collapsing("Advanced Options", |ui| {
            ui.add_space(style::spacing::MEDIUM);

            // Max file size
            ui.horizontal(|ui| {
                ui.label("Max File Size (MB):")
                    .on_hover_text("Maximum file size limit in megabytes. Files larger than this will be rejected for security.");
                ui.add(egui::Slider::new(&mut app.max_file_size_mb, 1..=1024))
                    .on_hover_text("Adjust maximum file size limit (1-1024 MB)");
                ui.label(format!("{} MB", app.max_file_size_mb));
            });

            // Max dimension (images only)
            if let Some(crate::app::FileType::Image) = app.detected_file_type {
                ui.horizontal(|ui| {
                    ui.label("Max Dimension (pixels):")
                        .on_hover_text("Maximum image dimension in pixels. Images larger than this will be rejected.");
                    ui.add(egui::Slider::new(&mut app.max_dimension, 1000..=65535))
                        .on_hover_text("Adjust maximum image dimension (1000-65535 pixels)");
                    ui.label(format!("{} px", app.max_dimension));
                });
            }

            // Max vertices/faces (meshes only)
            if let Some(crate::app::FileType::Mesh) = app.detected_file_type {
                ui.horizontal(|ui| {
                    ui.label("Max Vertices:")
                        .on_hover_text("Maximum number of vertices allowed in mesh files. Meshes with more vertices will be rejected.");
                    ui.add(egui::Slider::new(&mut app.max_vertices, 1000..=10_000_000))
                        .on_hover_text("Adjust maximum vertex count (1000-10,000,000)");
                    ui.label(format!("{}", app.max_vertices));
                });

                ui.horizontal(|ui| {
                    ui.label("Max Faces:")
                        .on_hover_text("Maximum number of faces allowed in mesh files. Meshes with more faces will be rejected.");
                    ui.add(egui::Slider::new(&mut app.max_faces, 1000..=10_000_000))
                        .on_hover_text("Adjust maximum face count (1000-10,000,000)");
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
