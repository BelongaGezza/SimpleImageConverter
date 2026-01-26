// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Main application state and window setup for Simple Image Converter GUI

use crate::conversion;
use crate::error_messages;
use crate::ui;
use crate::utils;
use common::limits::ResourceLimits;
use mesh_core::ConversionOptions;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use uuid::Uuid;

#[cfg(feature = "viewer-3d")]
use crate::preview_3d;

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

    /// Batch processing queue (None if not initialized)
    pub batch_queue: Option<crate::batch_queue::BatchQueue>,

    /// Shared batch queue used while background batch processing is active.
    ///
    /// The worker thread updates this queue; the UI thread snapshots it into `batch_queue`
    /// each frame for rendering. This avoids holding locks in hot UI code while still
    /// reflecting real-time progress.
    pub batch_queue_shared: Option<Arc<Mutex<crate::batch_queue::BatchQueue>>>,

    /// Application settings (loaded from config file)
    pub settings: Option<crate::settings::AppSettings>,

    /// Conversion history
    pub history: Option<crate::history::ConversionHistory>,

    /// Preview cache for images
    pub preview_cache: Option<std::sync::Arc<std::sync::Mutex<crate::ui::preview::PreviewCache>>>,

    /// Whether settings panel is visible
    pub show_settings_panel: bool,

    /// Whether preview panel is expanded
    pub show_preview: bool,

    /// Whether About dialog is visible
    pub show_about_dialog: bool,

    /// Whether Help panel is visible
    pub show_help_panel: bool,

    /// Auto-save state for settings
    /// Tracks when settings were last changed and auto-save status
    pub settings_auto_save: SettingsAutoSave,

    /// Queue item editing dialog state
    /// Tracks which queue item is being edited (None if dialog is closed)
    pub editing_queue_item: Option<Uuid>,

    /// Draft state for the queue item edit dialog.
    ///
    /// This must be stored in the app state (not local variables) so that edits persist
    /// across frames in egui's immediate-mode UI. Changes are only committed on "Save".
    pub editing_queue_item_draft: Option<QueueItemEditDraft>,

    /// Confirmation dialog state
    /// Tracks which confirmation dialog should be shown
    pub confirmation_dialog: Option<ConfirmationDialog>,

    /// Batch processing state (pause/cancel flags)
    /// Shared with batch processing thread for thread-safe control
    pub batch_processing_state: Option<Arc<BatchProcessingState>>,

    /// 3D viewer state for mesh preview (only available with viewer-3d feature)
    #[cfg(feature = "viewer-3d")]
    pub viewer_3d: Option<Arc<Mutex<preview_3d::Viewer3D>>>,

    /// Track which mesh file is currently loaded in the 3D viewer (for reload detection)
    #[cfg(feature = "viewer-3d")]
    pub viewer_3d_loaded_file: Option<PathBuf>,
}

/// Draft state for the "Edit Queue Item" dialog.
///
/// This is separate from the underlying queue item; it allows users to cancel/escape
/// without committing changes.
#[derive(Debug, Clone)]
pub struct QueueItemEditDraft {
    pub id: Uuid,
    pub output_format: OutputFormat,
    /// Editable output path as a string (keeps partially-typed values stable).
    pub output_path_str: String,
    pub quality: u8,
    pub mesh_options: Option<crate::batch_queue::MeshOptions>,
    /// Preserve existing priority (not editable in the dialog).
    pub priority: crate::batch_queue::ProcessingPriority,
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
    #[allow(dead_code)] // Reserved for future use
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

/// Batch processing state for pause/resume/cancel control
///
/// This struct uses atomic flags for thread-safe pause/cancel state management.
/// It is shared between the UI thread and batch processing thread via Arc.
#[derive(Debug)]
pub struct BatchProcessingState {
    /// Whether batch processing is paused
    pub paused: AtomicBool,
    /// Whether batch processing should be cancelled
    pub cancelled: AtomicBool,
}

impl BatchProcessingState {
    /// Create a new batch processing state
    pub fn new() -> Self {
        Self {
            paused: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
        }
    }

    /// Check if processing is paused
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Acquire)
    }

    /// Check if processing is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Pause processing
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn pause(&self) {
        self.paused.store(true, Ordering::Release);
    }

    /// Resume processing
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn resume(&self) {
        self.paused.store(false, Ordering::Release);
    }

    /// Cancel processing
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Reset state (for new batch processing session)
    pub fn reset(&self) {
        self.paused.store(false, Ordering::Release);
        self.cancelled.store(false, Ordering::Release);
    }
}

impl Default for BatchProcessingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Conversion status for thread communication
///
/// Used by the conversion thread to communicate status updates to the UI thread.
#[derive(Debug, Clone)]
pub enum ConversionStatus {
    /// Ready to start conversion
    Ready,
    /// Conversion in progress (with start time)
    Converting {
        #[allow(dead_code)] // Reserved for future progress tracking
        start_time: Instant,
    },
    /// Conversion completed successfully (with output path)
    Success { output_path: PathBuf },
    /// Conversion failed (with error message)
    Error { message: String },
}

/// Auto-save state for settings
///
/// Tracks when settings were last changed and manages debounced auto-save.
#[derive(Debug, Clone)]
pub struct SettingsAutoSave {
    /// Timestamp when settings were last changed (None if no changes)
    pub last_changed: Option<Instant>,
    /// Current auto-save status
    pub status: AutoSaveStatus,
    /// Timestamp when status was set to Saved/Error (for auto-reset)
    pub status_set_time: Option<Instant>,
}

/// Auto-save status for visual feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoSaveStatus {
    /// No pending changes
    Idle,
    /// Settings changed, waiting for debounce
    Pending,
    /// Currently saving
    Saving,
    /// Successfully saved
    Saved,
    /// Save failed
    Error,
}

/// Confirmation dialog type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationDialog {
    /// Clear all selections
    ClearAll,
    /// Clear batch queue
    ClearQueue,
    /// Clear conversion history
    ClearHistory,
    /// Cancel batch processing
    CancelBatchProcessing,
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
            batch_queue: Some(crate::batch_queue::BatchQueue::new()),
            batch_queue_shared: None,
            settings: None, // Will be loaded on startup
            history: Some(crate::history::ConversionHistory::default()),
            preview_cache: Some(std::sync::Arc::new(std::sync::Mutex::new(
                crate::ui::preview::PreviewCache::new(),
            ))),
            show_settings_panel: false,
            show_preview: true, // Preview expanded by default
            show_about_dialog: false,
            show_help_panel: false,
            settings_auto_save: SettingsAutoSave {
                last_changed: None,
                status: AutoSaveStatus::Idle,
                status_set_time: None,
            },
            editing_queue_item: None,
            editing_queue_item_draft: None,
            confirmation_dialog: None,
            batch_processing_state: None,
            #[cfg(feature = "viewer-3d")]
            viewer_3d: Some(Arc::new(Mutex::new(preview_3d::Viewer3D::new()))),
            #[cfg(feature = "viewer-3d")]
            viewer_3d_loaded_file: None,
        }
    }
}

impl SettingsAutoSave {
    /// Mark settings as changed (resets debounce timer)
    pub fn mark_changed(&mut self) {
        self.last_changed = Some(Instant::now());
        self.status = AutoSaveStatus::Pending;
        self.status_set_time = None;
    }

    /// Check if debounce period has elapsed and trigger save if needed
    ///
    /// Returns `true` if save should be triggered.
    ///
    /// Performance: Uses 500ms debounce to batch rapid settings changes,
    /// reducing disk I/O and improving UI responsiveness.
    pub fn should_save(&self) -> bool {
        if let Some(last_changed) = self.last_changed {
            // Debounce period: 500ms (optimal balance between responsiveness and performance)
            const DEBOUNCE_MS: u64 = 500;
            last_changed.elapsed().as_millis() >= DEBOUNCE_MS as u128
                && self.status == AutoSaveStatus::Pending
        } else {
            false
        }
    }

    /// Mark as currently saving
    pub fn set_saving(&mut self) {
        self.status = AutoSaveStatus::Saving;
        self.status_set_time = None;
    }

    /// Mark as successfully saved
    pub fn set_saved(&mut self) {
        self.last_changed = None;
        self.status = AutoSaveStatus::Saved;
        self.status_set_time = Some(Instant::now());
    }

    /// Mark as error
    pub fn set_error(&mut self) {
        self.status = AutoSaveStatus::Error;
        self.status_set_time = Some(Instant::now());
    }

    /// Check if status should be reset to idle (after showing Saved/Error for 2 seconds)
    pub fn should_reset_status(&self) -> bool {
        if let Some(status_set_time) = self.status_set_time {
            matches!(self.status, AutoSaveStatus::Saved | AutoSaveStatus::Error)
                && status_set_time.elapsed().as_secs() >= 2
        } else {
            false
        }
    }

    /// Reset to idle (called after showing saved/error status)
    pub fn reset_to_idle(&mut self) {
        if self.last_changed.is_none() {
            self.status = AutoSaveStatus::Idle;
            self.status_set_time = None;
        }
    }
}

impl eframe::App for ConverterApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Best-effort settings flush to avoid losing recent changes when the user closes the
        // window via OS controls (e.g., the window "X" button).
        if let Err(e) = self.save_settings() {
            // We can't reliably surface UI messages during shutdown.
            eprintln!("{e}");
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Performance: egui automatically optimizes redraws - only updates when necessary
        // (mouse movement, window resize, or explicit request_repaint() calls)

        // Load settings on first update if not already loaded
        if self.settings.is_none() {
            self.load_settings();
        }

        // Persist window dimensions (for restore on next launch).
        self.maybe_update_window_size(ctx);

        // Keep batch queue UI in sync with background processing.
        self.sync_batch_queue_from_thread();

        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Handle settings auto-save debounce (500ms debounce reduces disk I/O)
        if self.settings_auto_save.should_save() {
            if let Some(ref settings) = self.settings {
                self.settings_auto_save.set_saving();
                match settings.save() {
                    Ok(()) => {
                        self.settings_auto_save.set_saved();
                        // Auto-save succeeded - no message needed (visual indicator shows it)
                    }
                    Err(e) => {
                        self.settings_auto_save.set_error();
                        self.add_message(
                            format!("Failed to auto-save settings: {}", e),
                            MessageType::Error,
                        );
                    }
                }
            }
        }

        // Reset auto-save status after showing saved/error for a short time
        if self.settings_auto_save.should_reset_status() {
            self.settings_auto_save.reset_to_idle();
        }

        // Sync batch queue updates from processing thread
        // (Queue updates happen in thread, but we need to sync the main queue)
        // This is handled by the batch processing thread updating the Arc<Mutex<>> queue

        // Check conversion state and update UI if conversion completed
        let conversion_completed = if let Some(ref conversion_state) = self.conversion_state {
            let state = conversion_state.lock().unwrap_or_else(|poisoned| {
                eprintln!("Conversion state mutex poisoned, using potentially inconsistent data");
                poisoned.into_inner()
            });
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
                    let output_path_clone = output_path.clone();
                    self.status = Status::Success {
                        output_path: output_path_clone.clone(),
                    };

                    // Add to conversion history
                    if let (Some(source_file), Some(input_format), Some(output_format)) = (
                        self.source_file.as_ref(),
                        self.input_format,
                        self.output_format,
                    ) {
                        if let Some(ref mut history) = self.history {
                            let input_format_str = match input_format {
                                InputFormat::Image(fmt) => format!("{:?}", fmt),
                                InputFormat::Mesh(fmt) => format!("{:?}", fmt),
                            };
                            let output_format_str = match output_format {
                                OutputFormat::Image(fmt) => format!("{:?}", fmt),
                                OutputFormat::Mesh(fmt) => format!("{:?}", fmt),
                            };

                            let entry = crate::history::ConversionEntry::new(
                                (*source_file).clone(),
                                output_path_clone,
                                input_format_str,
                                output_format_str,
                                true,
                                None,
                            );
                            history.add_entry(entry);
                        }
                    }
                }
                Err(message) => {
                    let message_clone = message.clone();
                    self.add_message(message_clone.clone(), MessageType::Error);
                    self.status = Status::Error {
                        message: message_clone.clone(),
                    };

                    // Add failed conversion to history
                    if let (Some(source_file), Some(input_format), Some(output_format)) = (
                        self.source_file.as_ref(),
                        self.input_format,
                        self.output_format,
                    ) {
                        if let Some(ref mut history) = self.history {
                            let input_format_str = match input_format {
                                InputFormat::Image(fmt) => format!("{:?}", fmt),
                                InputFormat::Mesh(fmt) => format!("{:?}", fmt),
                            };
                            let output_format_str = match output_format {
                                OutputFormat::Image(fmt) => format!("{:?}", fmt),
                                OutputFormat::Mesh(fmt) => format!("{:?}", fmt),
                            };

                            let entry = crate::history::ConversionEntry::new(
                                (*source_file).clone(),
                                PathBuf::new(), // No output path for failed conversions
                                input_format_str,
                                output_format_str,
                                false,
                                Some(message_clone),
                            );
                            history.add_entry(entry);
                        }
                    }
                }
            }
            // Clear conversion state
            self.conversion_state = None;
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button("Open File...")
                        .on_hover_text("Open file browser to select an image or mesh file (Keyboard: Ctrl+O / Cmd+O)")
                        .clicked()
                    {
                        if let Some(file_path) = rfd::FileDialog::new()
                            .add_filter(
                                "Supported Files",
                                &[
                                    // Images
                                    "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp",
                                    "svg",
                                    // Meshes
                                    "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step",
                                    "stp",
                                ],
                            )
                            .add_filter(
                                "Image Files",
                                &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg"],
                            )
                            .add_filter(
                                "Mesh Files",
                                &["stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp"],
                            )
                            .add_filter("All Files", &["*"])
                            .pick_file()
                        {
                            crate::ui::drop_zone::handle_file_selection_internal(self, file_path);
                        }
                        ui.close_menu();
                    }
                    if ui
                        .button("Clear")
                        .on_hover_text("Clear all selections and reset to initial state (Keyboard: Ctrl+R / Cmd+R)")
                        .clicked()
                    {
                        // Note: Confirmation is handled in the main UI, not in menu
                        self.reset();
                        ui.close_menu();
                    }
                    ui.separator();
                    let save_button = if self.show_settings_panel {
                        ui.button("Save Settings")
                            .on_hover_text("Save current settings to disk (Keyboard: Ctrl+S / Cmd+S, requires settings panel to be open)")
                    } else {
                        ui.add_enabled(false, egui::Button::new("Save Settings"))
                            .on_hover_text("Open settings panel first (Edit → Preferences)")
                    };
                    if save_button.clicked() {
                        if let Err(e) = self.save_settings() {
                            self.add_message(
                                format!("Failed to save settings: {}", e),
                                MessageType::Error,
                            );
                        } else {
                            self.add_message("Settings saved".to_string(), MessageType::Success);
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .button("Exit")
                        .on_hover_text("Exit the application. Settings will be saved automatically.")
                        .clicked()
                    {
                        // Save settings before exiting
                        if let Err(e) = self.save_settings() {
                            eprintln!("Failed to save settings on exit: {}", e);
                        }
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui
                        .button("Preferences...")
                        .on_hover_text("Open settings panel to configure application preferences")
                        .clicked()
                    {
                        // Toggle settings panel visibility
                        self.show_settings_panel = !self.show_settings_panel;
                        if self.show_settings_panel {
                            self.add_message("Settings panel opened".to_string(), MessageType::Info);
                        } else {
                            self.add_message("Settings panel closed".to_string(), MessageType::Info);
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui
                        .button("Help & Documentation")
                        .on_hover_text("Show help panel with quick start guide, shortcuts, and troubleshooting (F1)")
                        .clicked()
                    {
                        self.show_help_panel = true;
                        ui.close_menu();
                    }
                    if ui
                        .button("About")
                        .on_hover_text("Show information about Simple Image Converter")
                        .clicked()
                    {
                        self.show_about_dialog = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .button("Source Code")
                        .on_hover_text("Open the GitHub repository in your default browser")
                        .clicked()
                    {
                        // Open GitHub repository in default browser
                        let repo_url = "https://github.com/BelongaGezza/SimpleImageConverter";
                        if let Err(e) = open::that(repo_url) {
                            self.add_message(
                                format!("Failed to open repository: {}", e),
                                MessageType::Error,
                            );
                        } else {
                            self.add_message(
                                "Opening repository in browser...".to_string(),
                                MessageType::Info,
                            );
                        }
                        ui.close_menu();
                    }
                    if ui
                        .button("License")
                        .on_hover_text("View the project license (MIT OR Apache-2.0) in your browser")
                        .clicked()
                    {
                        // Open GitHub license page in default browser
                        // Since the project uses dual license (MIT OR Apache-2.0),
                        // we'll link to the main repository where both licenses are available
                        let license_url = "https://github.com/BelongaGezza/SimpleImageConverter/blob/main/LICENSE-MIT";
                        if let Err(e) = open::that(license_url) {
                            self.add_message(
                                format!("Failed to open license: {}", e),
                                MessageType::Error,
                            );
                        } else {
                            self.add_message(
                                "Opening license in browser...".to_string(),
                                MessageType::Info,
                            );
                        }
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

        // Main content area with scrolling support
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
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
                            ui.add_space(15.0);
                        }

                        // Preview panel (full width, above Format/Options)
                        // Auto-expand preview when file is selected
                        if self.source_file.is_some() && !self.show_preview {
                            self.show_preview = true;
                        }
                        // Use CollapsingHeader with default_open state
                        let header = egui::CollapsingHeader::new("Preview")
                            .default_open(self.show_preview)
                            .show(ui, |ui| {
                                ui.add_space(5.0);
                                if let Some(ref source_file) = self.source_file {
                                    match self.detected_file_type {
                                        Some(crate::app::FileType::Image) => {
                                            if let Some(ref cache) = self.preview_cache {
                                                // Try to show image preview
                                                let limits = ResourceLimits::default();
                                                match crate::ui::preview::get_or_generate_preview(
                                                    source_file,
                                                    400,
                                                    300,
                                                    &limits,
                                                    cache,
                                                ) {
                                                    Ok(preview_data) => {
                                                        let texture = ui.ctx().load_texture(
                                                            "preview",
                                                            preview_data.image.clone(),
                                                            Default::default(),
                                                        );
                                                        ui.image((
                                                            texture.id(),
                                                            texture.size_vec2(),
                                                        ));
                                                        ui.add_space(5.0);
                                                        ui.label(format!(
                                                            "Original: {}x{}",
                                                            preview_data.original_width,
                                                            preview_data.original_height
                                                        ));
                                                    }
                                                    Err(_) => {
                                                        ui.label("Preview not available");
                                                    }
                                                }
                                            }
                                        }
                                        Some(crate::app::FileType::Mesh) => {
                                            #[cfg(feature = "viewer-3d")]
                                            {
                                                // Try to show 3D viewer if available
                                                if let Some(ref viewer_arc) = self.viewer_3d {
                                                    // Handle mutex lock error gracefully
                                                    if let Ok(mut viewer) = viewer_arc.lock() {
                                                        // Load mesh if not already loaded or if file changed
                                                    let should_load = match &self.viewer_3d_loaded_file {
                                                        Some(loaded_path) if loaded_path == source_file => {
                                                            // Same file already loaded, no need to reload
                                                            false
                                                        }
                                                        _ => {
                                                            // Different file or no file loaded, need to load
                                                            true
                                                        }
                                                    };

                                                    if should_load {
                                                        // Load mesh for 3D viewer
                                                        let limits = ResourceLimits::default();
                                                        match std::fs::read(source_file) {
                                                            Ok(input_data) => {
                                                                match mesh_core::FormatRegistry::detect_two_stage(source_file, &input_data) {
                                                                    Ok(format) => {
                                                                        let mesh_limits = ResourceLimits::builder()
                                                                            .max_file_size(limits.max_file_size)
                                                                            .max_vertices(limits.max_vertices)
                                                                            .max_faces(limits.max_faces)
                                                                            .build();
                                                                        match mesh_core::FormatRegistry::get_reader_with_limits(format, mesh_limits) {
                                                                            Ok(reader) => {
                                                                                match reader.read(&input_data) {
                                                                                    Ok(mesh) => {
                                                                                        viewer.set_mesh(Arc::new(mesh));
                                                                                        // Track that this file is now loaded
                                                                                        self.viewer_3d_loaded_file = Some(source_file.clone());
                                                                                    }
                                                                                    Err(e) => {
                                                                                        // Show error but don't crash
                                                                                        ui.label(format!("Failed to load mesh: {}", e));
                                                                                        self.viewer_3d_loaded_file = None;
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(e) => {
                                                                                ui.label(format!("Failed to get mesh reader: {}", e));
                                                                                self.viewer_3d_loaded_file = None;
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => {
                                                                        ui.label(format!("Failed to detect mesh format: {}", e));
                                                                        self.viewer_3d_loaded_file = None;
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                ui.label(format!("Failed to read file: {}", e));
                                                                self.viewer_3d_loaded_file = None;
                                                            }
                                                        }
                                                    }

                                                    // Render mode controls
                                                    ui.horizontal(|ui| {
                                                        ui.label("Render Mode:");
                                                        if ui.selectable_label(
                                                            viewer.render_mode() == preview_3d::RenderMode::Solid,
                                                            "Solid"
                                                        ).clicked() {
                                                            viewer.set_render_mode(preview_3d::RenderMode::Solid);
                                                        }
                                                        if ui.selectable_label(
                                                            viewer.render_mode() == preview_3d::RenderMode::Wireframe,
                                                            "Wireframe"
                                                        ).clicked() {
                                                            viewer.set_render_mode(preview_3d::RenderMode::Wireframe);
                                                        }

                                                        ui.add_space(10.0);

                                                        // Camera reset button
                                                        if ui.button("Reset Camera").clicked() {
                                                            viewer.reset_camera();
                                                        }
                                                    });

                                                    ui.add_space(5.0);

                                                    // Allocate space for 3D viewer
                                                    let viewer_size = egui::Vec2::new(
                                                        ui.available_width(),
                                                        400.0_f32.min(ui.available_height() * 0.6)
                                                    );

                                                    // Render 3D viewer
                                                    let _response = viewer.render(ui, viewer_size, frame);

                                                    // Show mesh info below viewer (if mesh is loaded)
                                                    if viewer.has_mesh() {
                                                        // Get mesh metadata for display
                                                        let limits = ResourceLimits::default();
                                                        if let Ok(metadata) = crate::ui::preview::get_mesh_metadata(
                                                            source_file,
                                                            &limits,
                                                        ) {
                                                            ui.add_space(5.0);
                                                            ui.separator();
                                                            ui.add_space(5.0);
                                                            ui.label(format!("Format: {:?}", metadata.format));
                                                            ui.label(format!("Vertices: {}", metadata.vertex_count));
                                                            ui.label(format!("Faces: {}", metadata.face_count));
                                                            ui.label(format!("Normals: {}", if metadata.has_normals { "Yes" } else { "No" }));
                                                            ui.label(format!("UVs: {}", if metadata.has_uvs { "Yes" } else { "No" }));
                                                        }
                                                    }
                                                } else {
                                                    // Mutex lock failed - show error and fallback
                                                    ui.label("Viewer unavailable: internal error");
                                                    self.add_message(
                                                        "Failed to access 3D viewer due to internal error".to_string(),
                                                        crate::app::MessageType::Error,
                                                    );
                                                }
                                                } else {
                                                    // Fallback: show mesh metadata
                                                    let limits = ResourceLimits::default();
                                                    match crate::ui::preview::get_mesh_metadata(
                                                        source_file,
                                                        &limits,
                                                    ) {
                                                        Ok(metadata) => {
                                                            ui.label(format!("Format: {:?}", metadata.format));
                                                            ui.label(format!("Vertices: {}", metadata.vertex_count));
                                                            ui.label(format!("Faces: {}", metadata.face_count));
                                                            ui.label(format!("Normals: {}", if metadata.has_normals { "Yes" } else { "No" }));
                                                            ui.label(format!("UVs: {}", if metadata.has_uvs { "Yes" } else { "No" }));
                                                        }
                                                        Err(_) => {
                                                            ui.label("Mesh metadata not available");
                                                        }
                                                    }
                                                }
                                            } // Close cfg(feature = "viewer-3d") block

                                            #[cfg(not(feature = "viewer-3d"))]
                                            {
                                                // Show mesh metadata preview (fallback when viewer-3d feature not enabled)
                                                let limits = ResourceLimits::default();
                                                match crate::ui::preview::get_mesh_metadata(
                                                    source_file,
                                                    &limits,
                                                ) {
                                                    Ok(metadata) => {
                                                        ui.label(format!("Format: {:?}", metadata.format));
                                                        ui.label(format!("Vertices: {}", metadata.vertex_count));
                                                        ui.label(format!("Faces: {}", metadata.face_count));
                                                        ui.label(format!("Normals: {}", if metadata.has_normals { "Yes" } else { "No" }));
                                                        ui.label(format!("UVs: {}", if metadata.has_uvs { "Yes" } else { "No" }));
                                                    }
                                                    Err(_) => {
                                                        ui.label("Mesh metadata not available");
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            ui.label("No file selected for preview");
                                        }
                                    }
                                } else {
                                    ui.label("No file selected for preview");
                                }
                            });
                        // Update state when header is clicked - toggle the state
                        if header.header_response.clicked() {
                            self.show_preview = !self.show_preview;
                        }

                        ui.add_space(15.0);

                        // Format selection and Options panel side-by-side (below Preview)
                        ui.horizontal(|ui| {
                            let available_width = ui.available_width();
                            let spacing = 15.0;
                            let format_width = (available_width * 0.4).clamp(200.0, 300.0);
                            let options_width = available_width - format_width - spacing;

                            // Format selection
                            ui.vertical(|ui| {
                                ui.set_width(format_width);
                                ui::format_selector::render_format_selector(ui, self);
                            });

                            ui.add_space(spacing);

                            // Options panel
                            ui.vertical(|ui| {
                                ui.set_width(options_width);
                                ui::options_panel::render_options_panel(ui, self);
                            });
                        });

                        ui.add_space(15.0);

                        // Batch queue panel (collapsible) - REMOVED DUPLICATE
                        ui.collapsing("Batch Processing Queue", |ui| {
                            ui::batch_queue::render_batch_queue(ui, self);
                            // Render edit dialog if an item is being edited
                            ui::batch_queue::render_edit_dialog(ui, self);
                        });

                        ui.add_space(20.0);

                        // Settings panel (if enabled)
                        if self.show_settings_panel {
                            egui::CollapsingHeader::new("Settings")
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui::settings_panel::render_settings_panel(ui, self);
                                });
                            ui.add_space(15.0);
                        }

                        // Conversion history panel (collapsible)
                        ui.collapsing("Conversion History", |ui| {
                            ui::history_panel::render_history_panel(ui, self);
                        });

                        ui.add_space(15.0);

                        // Action buttons
                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    // Add padding to the right of buttons (left side in RTL layout)
                                    ui.add_space(10.0);

                    if ui
                        .button("Clear")
                        .on_hover_text("Clear all selections and reset to initial state (Keyboard: Ctrl+R / Cmd+R)")
                        .clicked()
                    {
                        self.confirmation_dialog = Some(ConfirmationDialog::ClearAll);
                    }

                                    let can_convert = self.source_file.is_some()
                                        && self.output_format.is_some()
                                        && !matches!(self.status, Status::Converting { .. });

                                    ui.set_enabled(can_convert);
                                    if ui
                                        .button("Convert")
                                        .on_hover_text(if can_convert {
                                            "Start conversion (Keyboard: Enter)"
                                        } else if self.source_file.is_none() {
                                            "Select a file first"
                                        } else if self.output_format.is_none() {
                                            "Select an output format first"
                                        } else {
                                            "Conversion in progress"
                                        })
                                        .clicked()
                                    {
                                        if let Err(e) = self.start_conversion(ctx.clone()) {
                                            self.add_message(
                                                format!("Could not start conversion: {}", e),
                                                MessageType::Error,
                                            );
                                        }
                                    }
                                },
                            );
                        });
                    }); // Close vertical layout
                }); // Close ScrollArea
        });

        // Render confirmation dialogs
        self.render_confirmation_dialogs(ctx);

        // Render About dialog
        if self.show_about_dialog {
            egui::Window::new("About Simple Image Converter")
                .collapsible(false)
                .resizable(false)
                .default_width(500.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui::help_panel::render_about_dialog(ui);
                    ui.add_space(crate::ui::style::spacing::STANDARD);
                    ui.separator();
                    ui.add_space(crate::ui::style::spacing::MEDIUM);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.show_about_dialog = false;
                        }
                    });
                });
        }

        // Render Help panel
        egui::Window::new("Help & Documentation")
            .open(&mut self.show_help_panel)
            .collapsible(true)
            .resizable(true)
            .default_width(700.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                ui::help_panel::render_help_panel(ui);
            });
    }
}

impl ConverterApp {
    /// Construct an app instance using pre-loaded settings.
    ///
    /// This is used by `main.rs` so window sizing can be applied before the first frame,
    /// while keeping settings application logic centralized.
    pub fn with_settings(settings: crate::settings::AppSettings) -> Self {
        let mut app = Self {
            settings: Some(settings.clone()),
            ..Self::default()
        };
        app.apply_settings(&settings);
        app
    }

    fn apply_settings(&mut self, settings: &crate::settings::AppSettings) {
        // Apply settings to app state
        if let Some(ref default_dir) = settings.default_output_directory {
            self.output_directory = default_dir.clone();
        }
        self.quality = settings.default_quality;
        self.show_advanced = settings.show_advanced_options;

        // Apply history sizing
        if let Some(ref mut history) = self.history {
            history.max_entries = settings.max_history_entries;
            history.entries.truncate(history.max_entries);
        }
    }

    fn maybe_update_window_size(&mut self, ctx: &egui::Context) {
        let Some(ref mut settings) = self.settings else {
            return;
        };

        let rect = ctx.screen_rect();
        let width = rect.width();
        let height = rect.height();

        // Only treat meaningful changes as an update (avoid churn from fractional pixel jitter).
        let changed = (settings.window_width - width).abs() > 1.0
            || (settings.window_height - height).abs() > 1.0;
        if changed {
            settings.window_width = width;
            settings.window_height = height;
            self.settings_auto_save.mark_changed();
        }
    }

    fn sync_batch_queue_from_thread(&mut self) {
        let Some(ref shared) = self.batch_queue_shared else {
            return;
        };

        let snapshot = {
            let guard = shared.lock().unwrap_or_else(|poisoned| {
                eprintln!(
                    "Batch queue mutex poisoned during UI sync, using potentially inconsistent data"
                );
                poisoned.into_inner()
            });
            guard.clone()
        };

        self.batch_queue = Some(snapshot.clone());

        // Cleanup processing state once the queue is fully settled.
        let is_done = snapshot.processing_ids.is_empty() && !snapshot.has_pending();
        if is_done && self.batch_processing_state.is_some() {
            let was_cancelled = self
                .batch_processing_state
                .as_ref()
                .map(|s| s.is_cancelled())
                .unwrap_or(false);

            self.batch_processing_state = None;
            self.batch_queue_shared = None;

            if was_cancelled {
                self.add_message(
                    "Batch processing finished (cancelled)".to_string(),
                    MessageType::Info,
                );
            } else {
                self.add_message(
                    "Batch processing completed".to_string(),
                    MessageType::Success,
                );
            }
        }
    }

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
    /// Clear all messages from the messages list
    ///
    /// This is called from the UI when the user clicks the "Clear" button
    /// in the messages area.
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
        self.show_preview = true; // Reset preview to expanded state
        #[cfg(feature = "viewer-3d")]
        {
            // Clear 3D viewer state
            if let Some(ref viewer_arc) = self.viewer_3d {
                if let Ok(mut viewer) = viewer_arc.lock() {
                    // Create a new empty viewer to reset state
                    *viewer = preview_3d::Viewer3D::new();
                }
            }
            self.viewer_3d_loaded_file = None;
        }
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
        // Prevent multiple concurrent conversions
        if matches!(self.status, Status::Converting { .. }) {
            return Err("Conversion already in progress.".to_string());
        }

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
                let mut state = conversion_state.lock().unwrap_or_else(|poisoned| {
                    eprintln!(
                        "Conversion state mutex poisoned, using potentially inconsistent data"
                    );
                    poisoned.into_inner()
                });
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
                        let mut state = conversion_state.lock().unwrap_or_else(|poisoned| {
                            eprintln!("Conversion state mutex poisoned, using potentially inconsistent data");
                            poisoned.into_inner()
                        });
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
                        let mut state = conversion_state.lock().unwrap_or_else(|poisoned| {
                            eprintln!("Conversion state mutex poisoned, using potentially inconsistent data");
                            poisoned.into_inner()
                        });
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
                let mut state = conversion_state.lock().unwrap_or_else(|poisoned| {
                    eprintln!(
                        "Conversion state mutex poisoned, using potentially inconsistent data"
                    );
                    poisoned.into_inner()
                });
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

    /// Start batch queue processing
    ///
    /// This method spawns a thread to process all pending items in the batch queue
    /// in parallel (for images) or sequentially (for meshes). The UI remains responsive during processing.
    ///
    /// # Arguments
    ///
    /// * `ctx` - egui context for requesting UI repaints during processing
    ///
    /// # Returns
    ///
    /// `Ok(())` if batch processing was started, or an error if the queue is empty
    /// or already processing.
    pub fn start_batch_processing(&mut self, ctx: egui::Context) -> Result<(), String> {
        let queue_arc = if let Some(ref queue) = self.batch_queue {
            if !queue.has_pending() {
                return Err("No pending items in queue".to_string());
            }
            if queue.current_index.is_some() || !queue.processing_ids.is_empty() {
                return Err("Batch processing already in progress".to_string());
            }
            // Create Arc<Mutex<>> for thread-safe queue access
            Arc::new(Mutex::new(queue.clone()))
        } else {
            return Err("Batch queue not initialized".to_string());
        };

        // Store Arc reference in app state so the UI can sync progress updates.
        self.batch_queue_shared = Some(queue_arc.clone());

        // Create or reset batch processing state
        let processing_state = if let Some(ref state) = self.batch_processing_state {
            // Reset existing state
            state.reset();
            state.clone()
        } else {
            // Create new state
            let state = Arc::new(BatchProcessingState::new());
            self.batch_processing_state = Some(state.clone());
            state
        };

        // Get max concurrent conversions from settings
        let max_concurrent = self
            .settings
            .as_ref()
            .and_then(|s| s.max_concurrent_conversions)
            .unwrap_or_else(|| {
                // Default to CPU cores, capped at 8
                num_cpus::get().min(8)
            })
            .max(1); // Ensure at least 1

        // Build resource limits
        let max_file_size_bytes = (self.max_file_size_mb as usize)
            .saturating_mul(1024)
            .saturating_mul(1024);
        let limits = ResourceLimits::builder()
            .max_file_size(max_file_size_bytes)
            .max_image_dimension(self.max_dimension)
            .max_vertices(self.max_vertices.min(usize::MAX as u64) as usize)
            .max_faces(self.max_faces.min(usize::MAX as u64) as usize)
            .build();

        // Store Arc reference in app state for thread-safe updates
        let queue_arc_for_thread = queue_arc.clone();
        let processing_state_for_thread = processing_state.clone();

        // Spawn batch processing thread
        let ctx_clone = ctx.clone();
        std::thread::spawn(move || {
            // Process items in parallel batches
            loop {
                // Check for cancellation
                if processing_state_for_thread.is_cancelled() {
                    // Mark all pending items as cancelled
                    let mut queue = queue_arc_for_thread.lock().unwrap_or_else(|poisoned| {
                        eprintln!("Queue mutex poisoned, using potentially inconsistent data");
                        poisoned.into_inner()
                    });
                    for item in queue.items.iter_mut() {
                        if item.status == crate::batch_queue::BatchItemStatus::Pending {
                            item.status = crate::batch_queue::BatchItemStatus::Cancelled;
                        }
                    }
                    break;
                }

                // Wait if paused
                while processing_state_for_thread.is_paused()
                    && !processing_state_for_thread.is_cancelled()
                {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }

                // Check for cancellation again after pause
                if processing_state_for_thread.is_cancelled() {
                    let mut queue = queue_arc_for_thread.lock().unwrap_or_else(|poisoned| {
                        eprintln!("Queue mutex poisoned, using potentially inconsistent data");
                        poisoned.into_inner()
                    });
                    for item in queue.items.iter_mut() {
                        if item.status == crate::batch_queue::BatchItemStatus::Pending {
                            item.status = crate::batch_queue::BatchItemStatus::Cancelled;
                        }
                    }
                    break;
                }

                // Get batch of pending items (up to max_concurrent)
                let pending_ids: Vec<uuid::Uuid> = {
                    let queue = queue_arc_for_thread.lock().unwrap_or_else(|poisoned| {
                        eprintln!("Queue mutex poisoned, using potentially inconsistent data");
                        poisoned.into_inner()
                    });
                    queue.get_pending_items(max_concurrent)
                };

                if pending_ids.is_empty() {
                    // No more pending items
                    break;
                }

                // Process items in parallel using rayon
                pending_ids.par_iter().for_each(|&id| {
                    // Check for cancellation before processing each item
                    if !processing_state_for_thread.is_cancelled() {
                        Self::process_batch_item_parallel(
                            queue_arc_for_thread.clone(),
                            id,
                            &limits,
                            ctx_clone.clone(),
                            processing_state_for_thread.clone(),
                        );
                    }
                });

                // Request UI repaint after batch
                ctx_clone.request_repaint();
            }
        });

        // Sync with thread-safe queue
        if let Some(ref mut queue) = self.batch_queue {
            *queue = queue_arc
                .lock()
                .unwrap_or_else(|poisoned| {
                    eprintln!(
                        "Queue mutex poisoned during sync, using potentially inconsistent data"
                    );
                    poisoned.into_inner()
                })
                .clone();
        }

        Ok(())
    }

    /// Pause batch processing
    ///
    /// Pauses the current batch processing operation. Processing can be resumed
    /// by calling `resume_batch_processing()`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if processing was paused, or an error if no processing is active.
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be called when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn pause_batch_processing(&self) -> Result<(), String> {
        if let Some(ref state) = self.batch_processing_state {
            state.pause();
            Ok(())
        } else {
            Err("No batch processing active".to_string())
        }
    }

    /// Resume batch processing
    ///
    /// Resumes a paused batch processing operation.
    ///
    /// # Returns
    ///
    /// `Ok(())` if processing was resumed, or an error if no processing is active.
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be called when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn resume_batch_processing(&self) -> Result<(), String> {
        if let Some(ref state) = self.batch_processing_state {
            state.resume();
            Ok(())
        } else {
            Err("No batch processing active".to_string())
        }
    }

    /// Cancel batch processing
    ///
    /// Cancels the current batch processing operation. Items currently being
    /// processed will finish, but pending items will be marked as cancelled.
    ///
    /// # Returns
    ///
    /// `Ok(())` if processing was cancelled, or an error if no processing is active.
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be called when pause/resume controls are added to the batch queue UI.
    #[allow(dead_code)]
    pub fn cancel_batch_processing(&self) -> Result<(), String> {
        if let Some(ref state) = self.batch_processing_state {
            state.cancel();
            Ok(())
        } else {
            Err("No batch processing active".to_string())
        }
    }

    /// Check if batch processing is paused
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used to display pause state in the batch queue UI.
    #[allow(dead_code)]
    pub fn is_batch_processing_paused(&self) -> bool {
        self.batch_processing_state
            .as_ref()
            .map(|s| s.is_paused())
            .unwrap_or(false)
    }

    /// Check if batch processing is cancelled
    ///
    /// Note: This method is kept for future UI integration (Sprint 10 Task 2.1).
    /// It will be used to display cancellation state in the batch queue UI.
    #[allow(dead_code)]
    pub fn is_batch_processing_cancelled(&self) -> bool {
        self.batch_processing_state
            .as_ref()
            .map(|s| s.is_cancelled())
            .unwrap_or(false)
    }

    /// Process a batch item in parallel (thread-safe)
    ///
    /// This method is called from parallel workers to process individual items.
    /// It handles thread-safe queue updates and conversion execution.
    fn process_batch_item_parallel(
        queue: Arc<Mutex<crate::batch_queue::BatchQueue>>,
        id: uuid::Uuid,
        limits: &ResourceLimits,
        ctx: egui::Context,
        processing_state: Arc<BatchProcessingState>,
    ) {
        // Check for cancellation before starting
        if processing_state.is_cancelled() {
            // Mark item as cancelled
            let mut guard = queue.lock().unwrap_or_else(|poisoned| {
                eprintln!(
                    "Queue mutex poisoned in cancel check, using potentially inconsistent data"
                );
                poisoned.into_inner()
            });
            if let Some(item) = guard.items.iter_mut().find(|i| i.id == id) {
                if item.status == crate::batch_queue::BatchItemStatus::Pending {
                    item.status = crate::batch_queue::BatchItemStatus::Cancelled;
                }
            }
            return;
        }
        // Mark as processing (thread-safe)
        let can_process = {
            let mut guard = queue.lock().unwrap_or_else(|poisoned| {
                eprintln!(
                    "Queue mutex poisoned in mark_processing, using potentially inconsistent data"
                );
                poisoned.into_inner()
            });
            guard.mark_processing(id)
        };

        if !can_process {
            return; // Already processing or not found
        }

        // Get item data (clone to avoid holding lock during conversion)
        let item_data = {
            let guard = queue.lock().unwrap_or_else(|poisoned| {
                eprintln!("Queue mutex poisoned in get_item, using potentially inconsistent data");
                poisoned.into_inner()
            });
            guard.get_item(id).cloned()
        };

        if let Some(item) = item_data {
            // Perform conversion (no lock held)
            let result = match item.output_format {
                OutputFormat::Image(img_format) => conversion::convert_image(
                    &item.source_path,
                    &item.output_path,
                    img_format,
                    item.options.quality,
                    limits,
                )
                .map_err(|e| error_messages::format_user_message(&e)),
                OutputFormat::Mesh(mesh_format) => {
                    let mesh_options = item
                        .options
                        .mesh_options
                        .as_ref()
                        .map(|m| mesh_core::ConversionOptions {
                            transform: m.transform,
                            recalculate_normals: m.recalculate_normals,
                            validate: m.validate,
                        })
                        .unwrap_or_default();

                    conversion::convert_mesh(
                        &item.source_path,
                        &item.output_path,
                        mesh_format,
                        mesh_options,
                        limits,
                    )
                    .map_err(|e| error_messages::format_user_message(&e))
                }
            };

            // Update status (thread-safe) - single lock acquisition for efficiency
            {
                let mut guard = queue.lock().unwrap_or_else(|poisoned| {
                    eprintln!("Queue mutex poisoned in update_status, using potentially inconsistent data");
                    poisoned.into_inner()
                });

                // Update item status and error field in single operation to minimize lock contention
                match result {
                    Ok(output_path) => {
                        // Set error field first if item exists
                        if let Some(item) = guard.get_item_mut(id) {
                            item.error = None;
                        }
                        // Update status (this also handles processing_ids and progress)
                        guard.update_item_status(
                            id,
                            crate::batch_queue::BatchItemStatus::Completed { output_path },
                            1.0,
                        );
                    }
                    Err(error_msg) => {
                        // Set error field first if item exists
                        if let Some(item) = guard.get_item_mut(id) {
                            item.error = Some(error_msg.clone());
                        }
                        // Update status (this also handles processing_ids and progress)
                        guard.update_item_status(
                            id,
                            crate::batch_queue::BatchItemStatus::Failed { error: error_msg },
                            0.0,
                        );
                    }
                }
            }

            // Request UI repaint
            ctx.request_repaint();
        }
    }

    /// Internal helper method for processing a batch item (sequential mode - kept for compatibility)
    #[allow(dead_code)]
    fn process_batch_item_internal(
        item: &mut crate::batch_queue::BatchItem,
        limits: &ResourceLimits,
    ) -> Result<PathBuf, String> {
        // Update item status to processing
        item.status = crate::batch_queue::BatchItemStatus::Processing;
        item.progress = 0.1;

        // Perform conversion based on file type
        let result = match item.output_format {
            OutputFormat::Image(img_format) => {
                item.progress = 0.3;
                conversion::convert_image(
                    &item.source_path,
                    &item.output_path,
                    img_format,
                    item.options.quality,
                    limits,
                )
                .map_err(|e| error_messages::format_user_message(&e))
            }
            OutputFormat::Mesh(mesh_format) => {
                item.progress = 0.3;
                let mesh_options = item
                    .options
                    .mesh_options
                    .as_ref()
                    .map(|m| mesh_core::ConversionOptions {
                        transform: m.transform,
                        recalculate_normals: m.recalculate_normals,
                        validate: m.validate,
                    })
                    .unwrap_or_default();

                conversion::convert_mesh(
                    &item.source_path,
                    &item.output_path,
                    mesh_format,
                    mesh_options,
                    limits,
                )
                .map_err(|e| error_messages::format_user_message(&e))
            }
        };

        // Return result (status update happens in caller)
        result
    }

    /// Load settings from configuration file
    ///
    /// Called on application startup to restore user preferences.
    pub fn load_settings(&mut self) {
        match crate::settings::AppSettings::load() {
            Ok(settings) => {
                self.settings = Some(settings.clone());
                self.apply_settings(&settings);
            }
            Err(e) => {
                // Settings file doesn't exist or is corrupted - use defaults
                self.add_message(format!("Using default settings: {}", e), MessageType::Info);
                let settings = crate::settings::AppSettings::default();
                self.settings = Some(settings.clone());
                self.apply_settings(&settings);
            }
        }
    }

    /// Handle keyboard shortcuts
    ///
    /// Processes common keyboard shortcuts for application actions.
    /// Uses Command key on macOS, Ctrl key on Windows/Linux.
    ///
    /// IMPORTANT: Use `key_pressed()` not `keys_down.contains()` to avoid false triggers
    /// when modifier keys are held down alone.
    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let modifiers = ctx.input(|i| i.modifiers);

        // Helper: Check for platform-appropriate modifier (Command on macOS, Ctrl on Windows/Linux)
        let cmd_or_ctrl = modifiers.command || modifiers.ctrl;

        // If a text input has focus, avoid stealing common editing keys (e.g., Ctrl+A, Space).
        // This preserves platform conventions and prevents surprising behavior while typing.
        let wants_keyboard_input = ctx.wants_keyboard_input();

        // Ctrl+O / Cmd+O: Open file
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::O)) {
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter(
                    "Supported Files",
                    &[
                        // Images
                        "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                        // Meshes
                        "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                    ],
                )
                .add_filter(
                    "Image Files",
                    &[
                        "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                    ],
                )
                .add_filter(
                    "Mesh Files",
                    &[
                        "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                    ],
                )
                .add_filter("All Files", &["*"])
                .pick_file()
            {
                crate::ui::drop_zone::handle_file_selection_internal(self, file_path);
            }
        }

        // Ctrl+S / Cmd+S: Save settings (if settings panel is visible)
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::S)) && self.show_settings_panel {
            if let Err(e) = self.save_settings() {
                self.add_message(
                    format!("Failed to save settings: {}", e),
                    MessageType::Error,
                );
            } else {
                self.add_message("Settings saved".to_string(), MessageType::Success);
            }
        }

        // Ctrl+R / Cmd+R: Reset/Clear
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::R)) {
            self.reset();
        }

        // Ctrl+A / Cmd+A: Add files to batch queue
        // Respect "Select All" when the user is typing in a text field.
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::A)) && !wants_keyboard_input {
            let mut dialog = rfd::FileDialog::new()
                .add_filter(
                    "Supported Files",
                    &[
                        // Images
                        "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                        // Meshes
                        "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                    ],
                )
                .add_filter(
                    "Image Files",
                    &[
                        "png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg",
                    ],
                )
                .add_filter(
                    "Mesh Files",
                    &[
                        "stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp",
                    ],
                )
                .add_filter("All Files", &["*"]);

            // Set directory if available
            if let Some(dir_str) = self.output_directory.to_str() {
                if let Ok(dir_path) = std::path::PathBuf::from(dir_str).canonicalize() {
                    dialog = dialog.set_directory(dir_path);
                }
            }

            if let Some(selected_files) = dialog.pick_files() {
                for file_path in selected_files {
                    crate::ui::batch_queue::add_file_to_batch_queue(self, file_path);
                }
            }
        }

        // Ctrl+Shift+D / Cmd+Shift+D: Clear batch queue
        if cmd_or_ctrl && modifiers.shift && ctx.input(|i| i.key_pressed(egui::Key::D)) {
            if let Some(ref queue) = self.batch_queue {
                if !queue.items.is_empty() {
                    self.confirmation_dialog = Some(ConfirmationDialog::ClearQueue);
                }
            }
        }

        // Ctrl+Enter / Cmd+Enter: Start batch processing
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            if let Some(ref queue) = self.batch_queue {
                if queue.has_pending() {
                    if let Err(e) = self.start_batch_processing(ctx.clone()) {
                        self.add_message(
                            format!("Cannot start batch processing: {}. Please check that there are items in the queue.", e),
                            MessageType::Error,
                        );
                    } else {
                        self.add_message("Batch processing started".to_string(), MessageType::Info);
                    }
                }
            }
        }

        // Ctrl+P / Cmd+P: Pause/Resume batch processing
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::P)) {
            let is_paused = self.is_batch_processing_paused();
            if is_paused {
                if let Err(e) = self.resume_batch_processing() {
                    self.add_message(
                        format!("Cannot resume batch processing: {}. Please start batch processing first.", e),
                        MessageType::Error,
                    );
                } else {
                    self.add_message("Batch processing resumed".to_string(), MessageType::Info);
                }
            } else if let Err(e) = self.pause_batch_processing() {
                self.add_message(
                    format!(
                        "Cannot pause batch processing: {}. Please start batch processing first.",
                        e
                    ),
                    MessageType::Error,
                );
            } else {
                self.add_message("Batch processing paused".to_string(), MessageType::Info);
            }
        }

        // Space: Pause/Resume batch processing (when processing is active)
        // Respect typing space in text fields while processing is active.
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) && !wants_keyboard_input {
            let is_processing_active = self.batch_processing_state.is_some();
            if is_processing_active {
                let is_paused = self.is_batch_processing_paused();
                if is_paused {
                    if let Err(e) = self.resume_batch_processing() {
                        self.add_message(
                            format!("Cannot resume batch processing: {}. Please start batch processing first.", e),
                            MessageType::Error,
                        );
                    } else {
                        self.add_message("Batch processing resumed".to_string(), MessageType::Info);
                    }
                } else if let Err(e) = self.pause_batch_processing() {
                    self.add_message(
                        format!("Cannot pause batch processing: {}. Please start batch processing first.", e),
                        MessageType::Error,
                    );
                } else {
                    self.add_message("Batch processing paused".to_string(), MessageType::Info);
                }
            }
        }

        // Escape: Close dialogs or cancel batch processing
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            // Close edit dialog if open
            if self.editing_queue_item.is_some() {
                self.editing_queue_item = None;
                self.editing_queue_item_draft = None;
            } else if self.batch_processing_state.is_some() {
                // Cancel batch processing if active
                if let Err(e) = self.cancel_batch_processing() {
                    self.add_message(
                        format!("Cannot cancel batch processing: {}. Please start batch processing first.", e),
                        MessageType::Error,
                    );
                } else {
                    self.add_message("Batch processing cancelled".to_string(), MessageType::Info);
                }
            }
        }

        // Enter: Start conversion (if file and format selected, and no batch processing)
        // Use key_pressed() instead of keys_down to prevent key repeat
        if ctx.input(|i| i.key_pressed(egui::Key::Enter))
            && !cmd_or_ctrl // Don't trigger if Ctrl+Enter/Cmd+Enter (batch processing)
            && self.source_file.is_some()
            && self.output_format.is_some()
            && !matches!(self.status, Status::Converting { .. })
        {
            if let Err(e) = self.start_conversion(ctx.clone()) {
                self.add_message(
                    format!("Could not start conversion: {}", e),
                    MessageType::Error,
                );
            }
        }

        // F1: Open help panel
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.show_help_panel = true;
        }
    }

    /// Render confirmation dialogs
    ///
    /// Shows confirmation dialogs based on the current confirmation_dialog state.
    fn render_confirmation_dialogs(&mut self, ctx: &egui::Context) {
        if let Some(dialog_type) = self.confirmation_dialog {
            let (title, message, action_text) = match dialog_type {
                ConfirmationDialog::ClearAll => (
                    "Clear All?",
                    "Are you sure you want to clear all selections?\nThis will reset the file selection, format, and options.",
                    "Clear",
                ),
                ConfirmationDialog::ClearQueue => (
                    "Clear Queue?",
                    "Are you sure you want to clear the entire batch queue?\nThis action cannot be undone.",
                    "Clear Queue",
                ),
                ConfirmationDialog::ClearHistory => (
                    "Clear History?",
                    "Are you sure you want to clear all conversion history?\nThis action cannot be undone.",
                    "Clear History",
                ),
                ConfirmationDialog::CancelBatchProcessing => (
                    "Cancel Batch Processing?",
                    "Are you sure you want to cancel batch processing?\nItems currently processing will finish, but pending items will be cancelled.",
                    "Cancel Processing",
                ),
            };

            let mut should_close = false;
            let mut should_confirm = false;

            egui::Window::new(title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label(message);
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                should_close = true;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button(action_text).clicked() {
                                        should_confirm = true;
                                    }
                                },
                            );
                        });
                    });
                });

            if should_close || should_confirm {
                self.confirmation_dialog = None;
                if should_confirm {
                    // Execute the action based on dialog type
                    match dialog_type {
                        ConfirmationDialog::ClearAll => {
                            self.reset();
                        }
                        ConfirmationDialog::ClearQueue => {
                            if let Some(ref mut queue) = self.batch_queue {
                                queue.clear();
                            }
                            self.add_message("Batch queue cleared".to_string(), MessageType::Info);
                        }
                        ConfirmationDialog::ClearHistory => {
                            if let Some(ref mut history) = self.history {
                                history.clear();
                            }
                            self.add_message(
                                "Conversion history cleared".to_string(),
                                MessageType::Info,
                            );
                        }
                        ConfirmationDialog::CancelBatchProcessing => {
                            if let Err(e) = self.cancel_batch_processing() {
                                self.add_message(
                                    format!("Cannot cancel batch processing: {}. Please start batch processing first.", e),
                                    MessageType::Error,
                                );
                            } else {
                                self.add_message(
                                    "Batch processing cancelled".to_string(),
                                    MessageType::Info,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// Save settings to configuration file
    ///
    /// Called on application exit or when user explicitly saves settings.
    pub fn save_settings(&self) -> Result<(), String> {
        if let Some(ref settings) = self.settings {
            settings
                .save()
                .map_err(|e| format!("Failed to save settings: {}", e))
        } else {
            Ok(()) // No settings to save
        }
    }
}
