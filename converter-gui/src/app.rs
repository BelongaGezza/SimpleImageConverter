// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Main application state and window setup for Simple Image Converter GUI

use crate::conversion;
use crate::error_messages;
use crate::ui;
use crate::utils;
use common::limits::ResourceLimits;
use mesh_core::ConversionOptions;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Main application struct implementing eframe::App
///
/// This struct holds all application state for the Simple Image Converter GUI.
/// It manages file selection, format detection, conversion options, and UI feedback.
///
/// # Example
///
/// ```no_run
/// use converter_gui::app::ConverterApp;
///
/// let app = ConverterApp::default();
/// // Use with eframe::run_native()
/// ```
pub struct ConverterApp {
    /// Selected source file path (None if no file selected)
    pub source_file: Option<PathBuf>,
    /// Detected file type (Image or Mesh)
    pub detected_file_type: Option<FileType>,

    /// Input format detected from the source file
    pub input_format: Option<InputFormat>,
    /// Output format selected by the user
    pub output_format: Option<OutputFormat>,

    /// Output filename (auto-generated from source + format, editable)
    pub output_filename: String,
    /// Output directory path (defaults to source file directory)
    pub output_directory: PathBuf,
    /// Quality setting (1-100) for lossy formats (JPEG, WebP)
    pub quality: u8,

    /// List of messages to display in the messages area
    pub messages: Vec<Message>,
    /// Current application status (Ready, Converting, Success, Error)
    pub status: Status,
    /// Thread-safe conversion state for progress tracking
    pub conversion_state: Option<Arc<Mutex<ConversionState>>>,

    /// Whether advanced options panel is visible
    pub show_advanced: bool,
    /// Maximum file size in MB (default: 100)
    pub max_file_size_mb: u64,
    /// Maximum image dimension in pixels (default: 65535)
    pub max_dimension: u32,
    /// Maximum number of vertices for mesh files (default: 10,000,000)
    pub max_vertices: u64,
    /// Maximum number of faces for mesh files (default: 10,000,000)
    pub max_faces: u64,

    /// Mesh conversion options
    /// Coordinate system transform (from, to) - None means no transform
    pub mesh_transform: Option<(mesh_core::CoordinateSystem, mesh_core::CoordinateSystem)>,
    /// Whether to recalculate vertex normals
    pub mesh_recalculate_normals: bool,
    /// Whether to validate mesh integrity
    pub mesh_validate: bool,
}

/// File type detection result
///
/// Represents whether a file is an image or a 3D mesh format.
/// Used to filter available output formats in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// 2D image format (PNG, JPEG, BMP, GIF, TIFF, WebP, SVG)
    Image,
    /// 3D mesh format (STL, OBJ, PLY, OFF, glTF, DXF, STEP)
    Mesh,
}

/// Input format (detected from file)
///
/// Represents the format detected from the source file using two-stage
/// format detection (extension + magic bytes for security).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    /// Detected image format
    Image(img_core::ImageFormat),
    /// Detected mesh format
    Mesh(mesh_core::MeshFormat),
}

/// Output format (selected by user)
///
/// Represents the target format selected by the user for conversion.
/// Only writable formats are available (SVG and STEP are excluded).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Selected image output format
    Image(img_core::ImageFormat),
    /// Selected mesh output format
    Mesh(mesh_core::MeshFormat),
}

/// UI message for display in the messages area
///
/// Messages are displayed to the user with appropriate styling based on
/// the message type (Info, Warning, Error, Success).
#[derive(Debug, Clone)]
pub struct Message {
    /// Message text (user-friendly, sanitized)
    pub text: String,
    /// Message type for visual styling
    pub message_type: MessageType,
}

/// Message type for visual styling
///
/// Determines the color and icon used when displaying messages in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Informational message (blue)
    Info,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Error,
    /// Success message (green)
    Success,
}

/// Application status displayed in the status bar
///
/// Tracks the current state of the application for user feedback.
#[derive(Debug, Clone)]
pub enum Status {
    /// Ready for file selection and conversion
    Ready,
    /// Conversion in progress (with start time for progress tracking)
    Converting { start_time: Instant },
    /// Conversion completed successfully (with output path)
    Success { output_path: PathBuf },
    /// Conversion failed (with user-friendly error message)
    Error { message: String },
}

/// Thread-safe conversion state for progress tracking
///
/// This struct is wrapped in `Arc<Mutex<>>` to allow safe sharing between
/// the conversion thread and the UI thread.
#[derive(Debug)]
pub struct ConversionState {
    /// Current conversion status
    pub status: ConversionStatus,
    /// Conversion progress (0.0 to 1.0)
    pub progress: f32,
    /// Status message for display
    pub message: String,
}

/// Conversion status for thread communication
///
/// Used by the conversion thread to communicate status updates to the UI thread.
#[derive(Debug, Clone)]
pub enum ConversionStatus {
    /// Ready to start conversion
    Ready,
    /// Conversion in progress (with start time)
    Converting { start_time: Instant },
    /// Conversion completed successfully (with output path)
    Success { output_path: PathBuf },
    /// Conversion failed (with error message)
    Error { message: String },
}

impl Default for ConverterApp {
    fn default() -> Self {
        Self {
            source_file: None,
            detected_file_type: None,
            input_format: None,
            output_format: None,
            output_filename: String::new(),
            output_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            quality: 90,
            messages: Vec::new(),
            status: Status::Ready,
            conversion_state: None,
            show_advanced: false,
            max_file_size_mb: 100,
            max_dimension: 65535,
            max_vertices: 10_000_000,
            max_faces: 10_000_000,
            mesh_transform: None,
            mesh_recalculate_normals: false,
            mesh_validate: false,
        }
    }
}

impl eframe::App for ConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check conversion state and update UI if conversion completed
        let conversion_completed = if let Some(ref conversion_state) = self.conversion_state {
            let state = conversion_state.lock().unwrap();
            match &state.status {
                ConversionStatus::Success { output_path } => {
                    // Conversion completed successfully - extract data before dropping lock
                    let filename = output_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("file")
                        .to_string();
                    let output_path = output_path.clone();
                    Some(Ok((filename, output_path)))
                }
                ConversionStatus::Error { message } => {
                    // Conversion failed - extract message before dropping lock
                    Some(Err(message.clone()))
                }
                ConversionStatus::Converting { .. } => {
                    // Update status to show conversion in progress
                    if !matches!(self.status, Status::Converting { .. }) {
                        self.status = Status::Converting {
                            start_time: Instant::now(),
                        };
                    }
                    // Request repaint to update progress indicator
                    ctx.request_repaint();
                    None
                }
                ConversionStatus::Ready => {
                    // Conversion thread is ready but hasn't started yet
                    // This shouldn't happen, but handle it gracefully
                    None
                }
            }
        } else {
            None
        };

        // Handle conversion completion outside the lock
        if let Some(result) = conversion_completed {
            match result {
                Ok((filename, output_path)) => {
                    self.add_message(
                        format!("File converted successfully: {}", filename),
                        MessageType::Success,
                    );
                    self.status = Status::Success { output_path };
                }
                Err(message) => {
                    self.add_message(message.clone(), MessageType::Error);
                    self.status = Status::Error { message };
                }
            }
            // Clear conversion state
            self.conversion_state = None;
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open File...").clicked() {
                        // TODO: Implement file browser
                        ui.close_menu();
                    }
                    if ui.button("Clear").clicked() {
                        self.reset();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Preferences...").clicked() {
                        // TODO: Implement preferences
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // TODO: Implement about dialog
                        ui.close_menu();
                    }
                });
            });
        });

        // Messages area at bottom (above status bar)
        // Note: Bottom panels stack from bottom to top, so define messages first (it will be above status bar)
        egui::TopBottomPanel::bottom("messages_panel")
            .resizable(true)
            .min_height(80.0)
            .max_height(200.0)
            .default_height(100.0)
            .show(ctx, |ui| {
                ui::messages::render_messages(ui, self);
            });

        // Status bar at very bottom (fixed height, no resizing)
        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.set_height(25.0); // Fixed height for status bar
                ui::status_bar::render_status_bar(ui, self);
            });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Simple Image Converter");
                ui.add_space(10.0);

                // File drop zone
                ui::drop_zone::render_drop_zone(ui, self);

                // Ensure proper spacing after drop zone - this moves cursor down
                ui.add_space(20.0);

                // Source file display (compact)
                if let Some(ref file) = self.source_file {
                    ui.horizontal(|ui| {
                        ui.label("Source File:");
                        ui.label(
                            file.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown"),
                        );
                    });
                    ui.add_space(10.0);
                }

                // Format selection and Options panel side-by-side
                // Use available width and ensure no overlap
                ui.horizontal(|ui| {
                    let available_width = ui.available_width();
                    let spacing = 10.0;

                    // Calculate widths: format selector gets 40%, options gets 60% (with spacing)
                    let format_width = (available_width * 0.4).clamp(200.0, 300.0);
                    let options_width = available_width - format_width - spacing;

                    // Left side: Format selection (fixed proportional width)
                    ui.vertical(|ui| {
                        ui.set_width(format_width);
                        ui::format_selector::render_format_selector(ui, self);
                    });

                    ui.add_space(spacing);

                    // Right side: Options panel (takes remaining space)
                    ui.vertical(|ui| {
                        ui.set_width(options_width);
                        ui::options_panel::render_options_panel(ui, self);
                    });
                });

                ui.add_space(10.0);

                // Action buttons
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").clicked() {
                            self.reset();
                        }

                        let can_convert = self.source_file.is_some()
                            && self.output_format.is_some()
                            && !matches!(self.status, Status::Converting { .. });

                        ui.set_enabled(can_convert);
                        if ui.button("Convert").clicked() {
                            if let Err(e) = self.start_conversion(ctx.clone()) {
                                self.add_message(
                                    format!("Could not start conversion: {}", e),
                                    MessageType::Error,
                                );
                            }
                        }
                    });
                });
            });
        });
    }
}

impl ConverterApp {
    /// Add a message to the messages list
    ///
    /// Messages are automatically limited to the last 50 to prevent memory issues.
    /// Older messages are removed when the limit is exceeded.
    ///
    /// # Arguments
    ///
    /// * `text` - The message text (should be user-friendly and sanitized)
    /// * `message_type` - The message type for visual styling
    ///
    /// # Example
    ///
    /// ```
    /// use converter_gui::app::{ConverterApp, MessageType};
    ///
    /// let mut app = ConverterApp::default();
    /// app.add_message("File converted successfully".to_string(), MessageType::Success);
    /// ```
    pub fn add_message(&mut self, text: String, message_type: MessageType) {
        self.messages.push(Message { text, message_type });
        // Keep only last 50 messages to prevent memory issues
        if self.messages.len() > 50 {
            self.messages.remove(0);
        }
    }

    /// Clear all messages from the messages list
    ///
    /// # Example
    ///
    /// ```
    /// use converter_gui::app::ConverterApp;
    ///
    /// let mut app = ConverterApp::default();
    /// app.clear_messages();
    /// ```
    #[allow(dead_code)] // May be used for "Clear Messages" button in future UI enhancement
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    /// Reset application state to default values
    ///
    /// Clears all file selections, format selections, messages, and resets
    /// all options to their default values. This is called when the user
    /// clicks the "Clear" button.
    ///
    /// # Example
    ///
    /// ```
    /// use converter_gui::app::ConverterApp;
    ///
    /// let mut app = ConverterApp::default();
    /// // ... user makes selections ...
    /// app.reset(); // Reset to initial state
    /// ```
    pub fn reset(&mut self) {
        self.source_file = None;
        self.detected_file_type = None;
        self.input_format = None;
        self.output_format = None;
        self.output_filename = String::new();
        self.output_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.quality = 90;
        self.messages.clear();
        self.status = Status::Ready;
        self.conversion_state = None;
        self.show_advanced = false;
    }

    /// Start conversion in a background thread
    ///
    /// This method spawns a thread to perform the conversion, keeping the UI
    /// responsive during the operation. The conversion state is shared between
    /// the thread and the UI using `Arc<Mutex<>>`.
    ///
    /// # Arguments
    ///
    /// * `ctx` - egui context for requesting UI repaints during conversion
    ///
    /// # Returns
    ///
    /// `Ok(())` if conversion thread was started successfully, or an error
    /// if required state is missing or invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Source file is not selected
    /// - Output format is not selected
    /// - Output path cannot be constructed
    ///
    /// # Example
    ///
    /// ```no_run
    /// use converter_gui::app::ConverterApp;
    /// use egui::Context;
    ///
    /// let mut app = ConverterApp::default();
    /// // ... set source file and output format ...
    /// // app.start_conversion(ctx)?;
    /// ```
    pub fn start_conversion(&mut self, ctx: egui::Context) -> Result<(), String> {
        // Validate required state
        let source_file = self
            .source_file
            .as_ref()
            .ok_or("No source file selected.")?;
        let output_format = self.output_format.ok_or("No output format selected.")?;

        // Build output path
        let output_path = self.output_directory.join(&self.output_filename);
        if output_path.file_name().is_none() {
            return Err("Invalid output filename.".to_string());
        }

        // Validate output filename
        if let Some(filename) = output_path.file_name().and_then(|n| n.to_str()) {
            utils::validate_output_filename(filename)
                .map_err(|e| format!("Invalid output filename: {}", e))?;
        } else {
            return Err("Invalid output filename.".to_string());
        }

        // Build resource limits (convert u64 to usize where needed)
        let max_file_size_bytes = (self.max_file_size_mb as usize)
            .saturating_mul(1024)
            .saturating_mul(1024);
        let limits = ResourceLimits::builder()
            .max_file_size(max_file_size_bytes)
            .max_image_dimension(self.max_dimension)
            .max_vertices(self.max_vertices.min(usize::MAX as u64) as usize)
            .max_faces(self.max_faces.min(usize::MAX as u64) as usize)
            .build();

        // Create conversion state
        let conversion_state = Arc::new(Mutex::new(ConversionState {
            status: ConversionStatus::Ready,
            progress: 0.0,
            message: String::new(),
        }));
        self.conversion_state = Some(conversion_state.clone());

        // Clone data for thread
        let source_file = source_file.clone();
        let output_path = output_path.clone();
        let quality = self.quality;
        let mesh_transform = self.mesh_transform;
        let mesh_recalculate_normals = self.mesh_recalculate_normals;
        let mesh_validate = self.mesh_validate;

        // Spawn conversion thread
        thread::spawn(move || {
            // Update state to converting
            {
                let mut state = conversion_state.lock().unwrap();
                state.status = ConversionStatus::Converting {
                    start_time: Instant::now(),
                };
                state.progress = 0.1;
                state.message = "Starting conversion...".to_string();
            }
            ctx.request_repaint();

            // Perform conversion based on file type
            let result = match output_format {
                OutputFormat::Image(img_format) => {
                    // Image conversion
                    {
                        let mut state = conversion_state.lock().unwrap();
                        state.progress = 0.3;
                        state.message = "Converting image...".to_string();
                    }
                    ctx.request_repaint();

                    conversion::convert_image(
                        &source_file,
                        &output_path,
                        img_format,
                        quality,
                        &limits,
                    )
                    .map_err(|e| error_messages::format_user_message(&e))
                }
                OutputFormat::Mesh(mesh_format) => {
                    // Mesh conversion
                    {
                        let mut state = conversion_state.lock().unwrap();
                        state.progress = 0.3;
                        state.message = "Converting mesh...".to_string();
                    }
                    ctx.request_repaint();

                    let options = ConversionOptions {
                        transform: mesh_transform,
                        recalculate_normals: mesh_recalculate_normals,
                        validate: mesh_validate,
                    };

                    conversion::convert_mesh(
                        &source_file,
                        &output_path,
                        mesh_format,
                        options,
                        &limits,
                    )
                    .map_err(|e| error_messages::format_user_message(&e))
                }
            };

            // Update conversion state with result
            {
                let mut state = conversion_state.lock().unwrap();
                match result {
                    Ok(output_path) => {
                        state.status = ConversionStatus::Success { output_path };
                        state.progress = 1.0;
                        state.message = "Conversion completed successfully.".to_string();
                    }
                    Err(error_msg) => {
                        state.status = ConversionStatus::Error { message: error_msg };
                        state.progress = 0.0;
                    }
                }
            }
            ctx.request_repaint();
        });

        Ok(())
    }
}
