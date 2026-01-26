// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Modern alternate GUI app implementation.
//!
//! This crate intentionally keeps UI separate from the existing `converter-gui` binary,
//! while reusing the existing conversion + queue logic via the `converter_gui` library.

use crate::ui::mode_switch::ProcessingMode;
use common::limits::ResourceLimits;
use converter_gui::app::{FileType, InputFormat, MessageType, OutputFormat, QueueItemEditDraft, Status};
use converter_gui::batch_queue::{BatchItem, BatchItemOptions, BatchItemStatus, BatchQueue, MeshOptions};
use converter_gui::history::{ConversionEntry, ConversionHistory};
use converter_gui::ui::preview::PreviewCache;
use egui::{RichText, Ui};
use img_core::FormatRegistry as ImageFormatRegistry;
use mesh_core::FormatRegistry as MeshFormatRegistry;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsPane {
    None,
    Preferences,
    Help,
    About,
}

#[derive(Debug, Clone)]
struct BatchDefaults {
    output_directory: Option<PathBuf>,
    default_image_format: img_core::ImageFormat,
    default_mesh_format: mesh_core::MeshFormat,
    quality: u8,
    mesh_transform: Option<(mesh_core::CoordinateSystem, mesh_core::CoordinateSystem)>,
    mesh_recalculate_normals: bool,
    mesh_validate: bool,
}

impl Default for BatchDefaults {
    fn default() -> Self {
        Self {
            output_directory: None,
            // Match existing defaults (alphabetical first writable; in old GUI comments).
            default_image_format: img_core::ImageFormat::Bmp,
            default_mesh_format: mesh_core::MeshFormat::Dxf,
            quality: 90,
            mesh_transform: None,
            mesh_recalculate_normals: false,
            mesh_validate: false,
        }
    }
}

#[derive(Debug)]
struct BatchProcessingState {
    paused: AtomicBool,
    cancelled: AtomicBool,
}

impl BatchProcessingState {
    fn new() -> Self {
        Self {
            paused: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
        }
    }

    fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::Relaxed);
    }

    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

pub struct ModernApp {
    mode: ProcessingMode,
    settings: converter_gui::settings::AppSettings,
    settings_pane: SettingsPane,

    // Single-file flow
    source_file: Option<PathBuf>,
    detected_file_type: Option<FileType>,
    input_format: Option<InputFormat>,
    output_format: Option<OutputFormat>,
    output_filename: String,
    output_directory: PathBuf,
    quality: u8,
    mesh_transform: Option<(mesh_core::CoordinateSystem, mesh_core::CoordinateSystem)>,
    mesh_recalculate_normals: bool,
    mesh_validate: bool,

    // Preview + history
    show_preview: bool,
    preview_cache: Arc<Mutex<PreviewCache>>,
    history: ConversionHistory,
    recorded_batch_history_ids: HashSet<Uuid>,

    // Safety limits (kept consistent with original GUI defaults)
    max_file_size_mb: u64,
    max_dimension: u32,
    max_vertices: u64,
    max_faces: u64,

    // Batch flow
    batch_defaults: BatchDefaults,
    batch_queue: BatchQueue,
    batch_queue_shared: Option<Arc<Mutex<BatchQueue>>>,
    batch_processing_state: Option<Arc<BatchProcessingState>>,
    editing_queue_item: Option<Uuid>,
    editing_queue_item_draft: Option<QueueItemEditDraft>,

    // Messaging / status
    messages: Vec<Message>,
    status: Status,

    // Background single conversion result
    single_result_rx: Option<mpsc::Receiver<Result<PathBuf, String>>>,
}

impl ModernApp {
    pub fn new(settings: converter_gui::settings::AppSettings) -> Self {
        let output_directory = settings
            .default_output_directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let mut batch_defaults = BatchDefaults::default();
        batch_defaults.output_directory = settings.default_output_directory.clone();
        batch_defaults.quality = settings.default_quality;

        let mut history = ConversionHistory::default();
        history.max_entries = settings.max_history_entries;

        Self {
            mode: ProcessingMode::Single,
            settings,
            settings_pane: SettingsPane::None,

            source_file: None,
            detected_file_type: None,
            input_format: None,
            output_format: None,
            output_filename: String::new(),
            output_directory,
            quality: batch_defaults.quality,
            mesh_transform: None,
            mesh_recalculate_normals: false,
            mesh_validate: false,

            show_preview: true,
            preview_cache: Arc::new(Mutex::new(PreviewCache::new())),
            history,
            recorded_batch_history_ids: HashSet::new(),

            max_file_size_mb: 100,
            max_dimension: 65535,
            max_vertices: 10_000_000,
            max_faces: 10_000_000,

            batch_defaults,
            batch_queue: BatchQueue::new(),
            batch_queue_shared: None,
            batch_processing_state: None,
            editing_queue_item: None,
            editing_queue_item_draft: None,

            messages: Vec::new(),
            status: Status::Ready,

            single_result_rx: None,
        }
    }

    fn limits(&self) -> ResourceLimits {
        let max_vertices = (self.max_vertices.min(usize::MAX as u64)) as usize;
        let max_faces = (self.max_faces.min(usize::MAX as u64)) as usize;
        ResourceLimits::builder()
            .max_file_size((self.max_file_size_mb.saturating_mul(1024 * 1024)) as usize)
            .max_image_dimension(self.max_dimension)
            .max_vertices(max_vertices)
            .max_faces(max_faces)
            .build()
    }

    fn add_message(&mut self, text: impl Into<String>, message_type: MessageType) {
        self.messages.push(Message {
            text: text.into(),
            message_type,
        });
        if self.messages.len() > 50 {
            self.messages.remove(0);
        }
    }

    fn sanitize_path(path: &Path) -> String {
        converter_gui::utils::sanitize_path_for_display(path)
    }

    fn detect_file_type(path: &Path) -> Option<FileType> {
        if ImageFormatRegistry::detect_from_path(path).is_ok() {
            Some(FileType::Image)
        } else if MeshFormatRegistry::detect_from_path(path).is_ok() {
            Some(FileType::Mesh)
        } else {
            None
        }
    }

    fn set_selected_file(&mut self, path: PathBuf) {
        // Security: validate file path early (defense-in-depth).
        if let Err(e) = common::validation::validate_file_path(&path) {
            self.add_message(format!("{e}"), MessageType::Error);
            return;
        }

        let limits = self.limits();
        let file_bytes = match common::io::read_file_bytes_checked(&path, &limits) {
            Ok(b) => b,
            Err(e) => {
                self.add_message(format!("{e}"), MessageType::Error);
                return;
            }
        };

        let Some(ft) = Self::detect_file_type(&path) else {
            self.add_message(
                "Unsupported file type. Please select a supported image or mesh file.",
                MessageType::Error,
            );
            return;
        };

        self.source_file = Some(path.clone());
        self.detected_file_type = Some(ft);

        if let Some(parent) = path.parent() {
            self.output_directory = parent.to_path_buf();
        }

        // Input format detection (security: image uses two-stage).
        match ft {
            FileType::Image => match img_core::FormatRegistry::detect_two_stage(&path, &file_bytes) {
                Ok(fmt) => {
                    self.input_format = Some(InputFormat::Image(fmt));
                    self.add_message(format!("Image file detected: {fmt:?}"), MessageType::Info);
                }
                Err(e) => {
                    self.input_format = None;
                    self.add_message(format!("{e}"), MessageType::Error);
                }
            },
            FileType::Mesh => match MeshFormatRegistry::detect_from_path(&path) {
                Ok(fmt) => {
                    self.input_format = Some(InputFormat::Mesh(fmt));
                    self.add_message(format!("Mesh file detected: {fmt:?}"), MessageType::Info);
                }
                Err(e) => {
                    self.input_format = None;
                    self.add_message(format!("{e}"), MessageType::Error);
                }
            },
        }

        // Default output format + filename.
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            let default_out = match ft {
                FileType::Image => converter_gui::format_helpers::get_writable_image_formats()
                    .first()
                    .copied()
                    .map(OutputFormat::Image),
                FileType::Mesh => converter_gui::format_helpers::get_writable_mesh_formats()
                    .first()
                    .copied()
                    .map(OutputFormat::Mesh),
            };
            if let Some(out) = default_out {
                self.output_format = Some(out);
                let ext = match out {
                    OutputFormat::Image(fmt) => converter_gui::format_helpers::get_format_extension(fmt),
                    OutputFormat::Mesh(fmt) => {
                        converter_gui::format_helpers::get_mesh_format_extension(fmt)
                    }
                };
                self.output_filename = format!("{stem}.{ext}");
            }
        }
    }

    fn output_path_for_single(&self) -> Option<PathBuf> {
        let name = self.output_filename.trim();
        if name.is_empty() {
            return None;
        }
        Some(self.output_directory.join(name))
    }

    fn start_single_conversion(&mut self) {
        let Some(source) = self.source_file.clone() else {
            return;
        };
        let Some(out_fmt) = self.output_format else {
            return;
        };
        let Some(output_path) = self.output_path_for_single() else {
            self.add_message("Output filename is empty.", MessageType::Error);
            return;
        };

        // Basic output safety checks (same utilities as original).
        if let Some(filename) = output_path.file_name().and_then(|n| n.to_str()) {
            if let Err(e) = converter_gui::utils::validate_output_filename(filename) {
                self.add_message(format!("Invalid output filename: {e}"), MessageType::Error);
                return;
            }
        }
        if let Err(e) = converter_gui::utils::validate_output_path_not_system(&output_path) {
            self.add_message(format!("Invalid output path: {e}"), MessageType::Error);
            return;
        }

        let limits = self.limits();
        let quality = self.quality;
        let mesh_options = mesh_core::ConversionOptions {
            transform: self.mesh_transform,
            recalculate_normals: self.mesh_recalculate_normals,
            validate: self.mesh_validate,
        };

        let (tx, rx) = mpsc::channel();
        self.single_result_rx = Some(rx);
        self.status = Status::Converting {
            start_time: Instant::now(),
        };

        thread::spawn(move || {
            let result = match out_fmt {
                OutputFormat::Image(img_fmt) => converter_gui::conversion::convert_image(
                    &source,
                    &output_path,
                    img_fmt,
                    quality,
                    &limits,
                )
                .map_err(|e| e.to_string()),
                OutputFormat::Mesh(mesh_fmt) => converter_gui::conversion::convert_mesh(
                    &source,
                    &output_path,
                    mesh_fmt,
                    mesh_options,
                    &limits,
                )
                .map_err(|e| e.to_string()),
            };
            let _ = tx.send(result);
        });
    }

    fn poll_single_conversion(&mut self) {
        let Some(rx) = self.single_result_rx.as_ref() else {
            return;
        };
        match rx.try_recv() {
            Ok(Ok(path)) => {
                self.single_result_rx = None;
                self.status = Status::Success {
                    output_path: path.clone(),
                };
                self.add_message(
                    format!("Conversion completed: {}", Self::sanitize_path(&path)),
                    MessageType::Success,
                );

                if self.settings.conversion_history_enabled {
                    self.history.max_entries = self.settings.max_history_entries;
                    let (input_format_str, output_format_str) =
                        formats_for_history(self.input_format, self.output_format);
                    if let Some(ref source) = self.source_file {
                        self.history.add_entry(ConversionEntry::new(
                            source.clone(),
                            path.clone(),
                            input_format_str,
                            output_format_str,
                            true,
                            None,
                        ));
                    }
                }
            }
            Ok(Err(err)) => {
                self.single_result_rx = None;
                self.status = Status::Error { message: err.clone() };
                self.add_message(err, MessageType::Error);

                if self.settings.conversion_history_enabled {
                    self.history.max_entries = self.settings.max_history_entries;
                    let (input_format_str, output_format_str) =
                        formats_for_history(self.input_format, self.output_format);
                    if let Some(ref source) = self.source_file {
                        self.history.add_entry(ConversionEntry::new(
                            source.clone(),
                            PathBuf::new(),
                            input_format_str,
                            output_format_str,
                            false,
                            Some("Conversion failed".to_string()),
                        ));
                    }
                }
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.single_result_rx = None;
                self.status = Status::Error {
                    message: "Conversion failed: internal error".to_string(),
                };
                self.add_message("Conversion failed: internal error", MessageType::Error);

                if self.settings.conversion_history_enabled {
                    self.history.max_entries = self.settings.max_history_entries;
                    let (input_format_str, output_format_str) =
                        formats_for_history(self.input_format, self.output_format);
                    if let Some(ref source) = self.source_file {
                        self.history.add_entry(ConversionEntry::new(
                            source.clone(),
                            PathBuf::new(),
                            input_format_str,
                            output_format_str,
                            false,
                            Some("Conversion failed: internal error".to_string()),
                        ));
                    }
                }
            }
        }
    }

    fn sync_batch_queue_from_thread(&mut self) {
        let Some(ref shared) = self.batch_queue_shared else {
            return;
        };
        let snapshot = {
            let guard = shared.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.clone()
        };
        self.batch_queue = snapshot.clone();

        // Record completed/failed items into history (once) on the UI thread.
        if self.settings.conversion_history_enabled {
            self.history.max_entries = self.settings.max_history_entries;
            for item in snapshot.items.iter() {
                if self.recorded_batch_history_ids.contains(&item.id) {
                    continue;
                }
                match &item.status {
                    BatchItemStatus::Completed { output_path } => {
                        let input_fmt = detect_input_format_string(item.file_type, &item.source_path);
                        let out_fmt = format!("{:?}", item.output_format);
                        self.history.add_entry(ConversionEntry::new(
                            item.source_path.clone(),
                            output_path.clone(),
                            input_fmt,
                            out_fmt,
                            true,
                            None,
                        ));
                        self.recorded_batch_history_ids.insert(item.id);
                    }
                    BatchItemStatus::Failed { error } => {
                        let input_fmt = detect_input_format_string(item.file_type, &item.source_path);
                        let out_fmt = format!("{:?}", item.output_format);
                        self.history.add_entry(ConversionEntry::new(
                            item.source_path.clone(),
                            item.output_path.clone(),
                            input_fmt,
                            out_fmt,
                            false,
                            Some(error.clone()),
                        ));
                        self.recorded_batch_history_ids.insert(item.id);
                    }
                    _ => {}
                }
            }
        }

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
                self.add_message("Batch processing finished (cancelled)", MessageType::Info);
            } else {
                self.add_message("Batch processing completed", MessageType::Success);
            }
        }
    }

    fn max_concurrent(&self) -> usize {
        self.settings
            .max_concurrent_conversions
            .unwrap_or_else(|| num_cpus::get().min(8).max(1))
            .clamp(1, 16)
    }

    fn start_batch_processing(&mut self, ctx: egui::Context) -> Result<(), String> {
        if !self.batch_queue.has_pending() {
            return Err("No pending items in queue".to_string());
        }
        if self.batch_queue.current_index.is_some() || !self.batch_queue.processing_ids.is_empty() {
            return Err("Batch processing already in progress".to_string());
        }

        let queue_arc = Arc::new(Mutex::new(self.batch_queue.clone()));
        let state = Arc::new(BatchProcessingState::new());
        let state_for_thread = Arc::clone(&state);
        let queue_for_thread = Arc::clone(&queue_arc);

        let max_concurrent = self.max_concurrent();
        let limits = self.limits();

        thread::spawn(move || {
            loop {
                if state_for_thread.is_cancelled() {
                    // Mark remaining pending as cancelled.
                    let mut guard = queue_for_thread.lock().unwrap_or_else(|p| p.into_inner());
                    for item in guard.items.iter_mut() {
                        if matches!(item.status, BatchItemStatus::Pending) {
                            item.status = BatchItemStatus::Cancelled;
                        }
                    }
                    guard.processing_ids.clear();
                    ctx.request_repaint();
                    break;
                }

                if state_for_thread.is_paused() {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                let batch_ids = {
                    let guard = queue_for_thread.lock().unwrap_or_else(|p| p.into_inner());
                    guard.get_pending_items(max_concurrent)
                };

                if batch_ids.is_empty() {
                    // Nothing left to do.
                    ctx.request_repaint();
                    break;
                }

                // Process batch IDs in parallel.
                rayon::scope(|s| {
                    for id in batch_ids {
                        let queue = Arc::clone(&queue_for_thread);
                        let state = Arc::clone(&state_for_thread);
                        let ctx = ctx.clone();
                        let limits = limits.clone();
                        s.spawn(move |_| {
                            if state.is_cancelled() {
                                return;
                            }
                            // Mark processing + snapshot item.
                            let item_snapshot = {
                                let mut guard =
                                    queue.lock().unwrap_or_else(|p| p.into_inner());
                                if !guard.mark_processing(id) {
                                    return;
                                }
                                guard.get_item(id).cloned()
                            };

                            let Some(item) = item_snapshot else {
                                return;
                            };

                            let result = process_batch_item(&item, &limits);

                            let mut guard = queue.lock().unwrap_or_else(|p| p.into_inner());
                            match result {
                                Ok(out_path) => {
                                    guard.update_item_status(
                                        id,
                                        BatchItemStatus::Completed {
                                            output_path: out_path,
                                        },
                                        1.0,
                                    );
                                }
                                Err(err) => {
                                    guard.update_item_status(
                                        id,
                                        BatchItemStatus::Failed { error: err },
                                        1.0,
                                    );
                                }
                            }
                            ctx.request_repaint();
                        });
                    }
                });
            }
        });

        self.batch_queue_shared = Some(queue_arc);
        self.batch_processing_state = Some(state);
        Ok(())
    }

    fn is_batch_paused(&self) -> bool {
        self.batch_processing_state
            .as_ref()
            .map(|s| s.is_paused())
            .unwrap_or(false)
    }

    fn pause_batch(&mut self) -> Result<(), String> {
        let Some(ref s) = self.batch_processing_state else {
            return Err("No batch processing active".to_string());
        };
        s.set_paused(true);
        Ok(())
    }

    fn resume_batch(&mut self) -> Result<(), String> {
        let Some(ref s) = self.batch_processing_state else {
            return Err("No batch processing active".to_string());
        };
        s.set_paused(false);
        Ok(())
    }

    fn cancel_batch(&mut self) -> Result<(), String> {
        let Some(ref s) = self.batch_processing_state else {
            return Err("No batch processing active".to_string());
        };
        s.cancel();
        Ok(())
    }

    fn add_files_to_batch(&mut self, files: Vec<PathBuf>) {
        for path in files {
            self.add_file_to_batch(path);
        }
    }

    fn add_file_to_batch(&mut self, file_path: PathBuf) {
        if let Err(e) = common::validation::validate_file_path(&file_path) {
            self.add_message(format!("{e}"), MessageType::Error);
            return;
        }

        let file_type = match Self::detect_file_type(&file_path) {
            Some(t) => t,
            None => {
                let filename = file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file");
                self.add_message(
                    format!("File type not supported: {filename}"),
                    MessageType::Error,
                );
                return;
            }
        };

        let output_format = match file_type {
            FileType::Image => OutputFormat::Image(self.batch_defaults.default_image_format),
            FileType::Mesh => OutputFormat::Mesh(self.batch_defaults.default_mesh_format),
        };

        let output_dir = self
            .batch_defaults
            .output_directory
            .clone()
            .or_else(|| file_path.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        let output_path = {
            let stem = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let ext = match output_format {
                OutputFormat::Image(fmt) => converter_gui::format_helpers::get_format_extension(fmt),
                OutputFormat::Mesh(fmt) => {
                    converter_gui::format_helpers::get_mesh_format_extension(fmt)
                }
            };
            output_dir.join(format!("{stem}.{ext}"))
        };

        let mesh_opts = if matches!(file_type, FileType::Mesh) {
            Some(MeshOptions {
                transform: self.batch_defaults.mesh_transform,
                recalculate_normals: self.batch_defaults.mesh_recalculate_normals,
                validate: self.batch_defaults.mesh_validate,
            })
        } else {
            None
        };

        let item = BatchItem::new(
            file_path,
            file_type,
            output_format,
            output_path,
            BatchItemOptions {
                quality: self.batch_defaults.quality,
                mesh_options: mesh_opts,
                priority: converter_gui::batch_queue::ProcessingPriority::Medium,
            },
        );

        match self.batch_queue.add_item(item) {
            Ok(()) => self.add_message("Added file to batch queue", MessageType::Info),
            Err(e) => self.add_message(e, MessageType::Error),
        }
    }

    fn render_messages(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Messages").strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").clicked() {
                    self.messages.clear();
                }
            });
        });
        ui.add_space(6.0);

        if self.messages.is_empty() {
            ui.label(
                RichText::new("No messages")
                    .italics()
                    .color(egui::Color32::GRAY)
                    .small(),
            );
            return;
        }

        egui::ScrollArea::vertical()
            .max_height(180.0)
            .show(ui, |ui| {
                for msg in self.messages.iter().rev() {
                    let color = match msg.message_type {
                        MessageType::Info => crate::ui::theme::Palette::default().secondary,
                        MessageType::Warning => crate::ui::theme::Palette::default().warn,
                        MessageType::Error => crate::ui::theme::Palette::default().error,
                        MessageType::Success => crate::ui::theme::Palette::default().success,
                    };
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("•").color(color).strong());
                        ui.label(RichText::new(&msg.text).color(color));
                    });
                    ui.add_space(2.0);
                }
            });
    }
}

fn process_batch_item(item: &BatchItem, limits: &ResourceLimits) -> Result<PathBuf, String> {
    match (item.file_type, item.output_format) {
        (FileType::Image, OutputFormat::Image(fmt)) => converter_gui::conversion::convert_image(
            &item.source_path,
            &item.output_path,
            fmt,
            item.options.quality,
            limits,
        )
        .map_err(|e| e.to_string()),
        (FileType::Mesh, OutputFormat::Mesh(fmt)) => {
            let mesh_opts = item
                .options
                .mesh_options
                .clone()
                .map(|m| mesh_core::ConversionOptions {
                    transform: m.transform,
                    recalculate_normals: m.recalculate_normals,
                    validate: m.validate,
                })
                .unwrap_or_default();

            converter_gui::conversion::convert_mesh(
                &item.source_path,
                &item.output_path,
                fmt,
                mesh_opts,
                limits,
            )
            .map_err(|e| e.to_string())
        }
        _ => Err("Mismatched file type and output format".to_string()),
    }
}

fn formats_for_history(
    input_format: Option<InputFormat>,
    output_format: Option<OutputFormat>,
) -> (String, String) {
    let input_format_str = match input_format {
        Some(InputFormat::Image(fmt)) => format!("{fmt:?}"),
        Some(InputFormat::Mesh(fmt)) => format!("{fmt:?}"),
        None => "Unknown".to_string(),
    };
    let output_format_str = match output_format {
        Some(OutputFormat::Image(fmt)) => format!("{fmt:?}"),
        Some(OutputFormat::Mesh(fmt)) => format!("{fmt:?}"),
        None => "Unknown".to_string(),
    };
    (input_format_str, output_format_str)
}

fn detect_input_format_string(file_type: FileType, path: &Path) -> String {
    match file_type {
        FileType::Image => img_core::FormatRegistry::detect_from_path(path)
            .map(|f| format!("{f:?}"))
            .unwrap_or_else(|_| "Unknown".to_string()),
        FileType::Mesh => mesh_core::FormatRegistry::detect_from_path(path)
            .map(|f| format!("{f:?}"))
            .unwrap_or_else(|_| "Unknown".to_string()),
    }
}

impl eframe::App for ModernApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Best-effort: persist settings if possible.
        let _ = self.settings.save();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_single_conversion();
        self.sync_batch_queue_from_thread();

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Simple Image Converter").strong());
                ui.add_space(10.0);
                crate::ui::mode_switch::render_mode_switch(ui, &mut self.mode);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("About").clicked() {
                        self.settings_pane = SettingsPane::About;
                    }
                    if ui.button("Help").clicked() {
                        self.settings_pane = SettingsPane::Help;
                    }
                    if ui
                        .add(
                            egui::Button::new(RichText::new("⚙").size(20.0))
                                .min_size(egui::vec2(36.0, 28.0)),
                        )
                        .on_hover_text("Preferences")
                        .clicked()
                    {
                        self.settings_pane = SettingsPane::Preferences;
                    }
                });
            });
            ui.add_space(6.0);
        });

        egui::TopBottomPanel::bottom("messages_panel")
            .resizable(true)
            .min_height(90.0)
            .max_height(240.0)
            .default_height(120.0)
            .show(ctx, |ui| self.render_messages(ui));

        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.set_height(28.0);
                ui.horizontal(|ui| {
                    ui.separator();
                    match &self.status {
                        Status::Ready => {
                            ui.label("Ready");
                        }
                        Status::Converting { start_time } => {
                            let secs = start_time.elapsed().as_secs();
                            ui.label(if secs > 30 {
                                format!("Converting... ({secs}s)")
                            } else {
                                "Converting...".to_string()
                            });
                            if secs > 30 {
                                ui.spinner();
                            }
                        }
                        Status::Success { output_path } => {
                            ui.label(format!(
                                "Conversion complete: {}",
                                Self::sanitize_path(output_path)
                            ));
                        }
                        Status::Error { message } => {
                            ui.label(
                                RichText::new(message)
                                    .color(crate::ui::theme::Palette::default().error),
                            );
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);

                match self.mode {
                    ProcessingMode::Single => {
                        render_single_mode(ui, self, ctx);
                    }
                    ProcessingMode::Batch => {
                        render_batch_mode(ui, self, ctx);
                    }
                }
            });
        });

        // Popovers / dialogs
        let popup_frame = egui::Frame::popup(ctx.style().as_ref())
            .fill(crate::ui::theme::Palette::default().panel)
            .stroke(egui::Stroke::new(
                1.5,
                crate::ui::theme::Palette::default().primary,
            ));

        match self.settings_pane {
            SettingsPane::None => {}
            SettingsPane::Preferences => {
                egui::Window::new("Preferences")
                    .collapsible(false)
                    .resizable(true)
                    .default_width(600.0)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(popup_frame)
                    .show(ctx, |ui| render_preferences(ui, self));
            }
            SettingsPane::Help => {
                egui::Window::new("Help")
                    .collapsible(false)
                    .resizable(true)
                    .default_width(720.0)
                    .default_height(560.0)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(popup_frame)
                    .show(ctx, |ui| {
                        render_help(ui);
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.button("Close").clicked() {
                                    self.settings_pane = SettingsPane::None;
                                }
                            },
                        );
                    });
            }
            SettingsPane::About => {
                egui::Window::new("About")
                    .collapsible(false)
                    .resizable(false)
                    .default_width(540.0)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(popup_frame)
                    .show(ctx, |ui| {
                        render_about(ui);
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.button("Close").clicked() {
                                    self.settings_pane = SettingsPane::None;
                                }
                            },
                        );
                    });
            }
        }

        if let Some(editing_id) = self.editing_queue_item {
            egui::Window::new("Edit Queue Item")
                .collapsible(false)
                .resizable(true)
                .default_width(560.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(popup_frame)
                .show(ctx, |ui| render_edit_queue_item(ui, self, editing_id));
        }
    }
}

fn render_single_mode(ui: &mut Ui, app: &mut ModernApp, _ctx: &egui::Context) {
    ui.heading("Single File");
    ui.add_space(10.0);

    // Drop zone (bold)
    ui.group(|ui| {
        ui.label(RichText::new("Drop a file here, or click to browse").strong());
        ui.add_space(8.0);

        let w = ui.available_width();
        let h = if app.source_file.is_some() { 72.0 } else { 200.0 };
        let resp = ui.allocate_response(egui::vec2(w, h), egui::Sense::click());

        let rect = resp.rect;
        let palette = crate::ui::theme::Palette::default();
        let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
        let dropped_files = ui
            .ctx()
            .input(|i| i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect::<Vec<_>>());

        let is_drag_over = !hovered_files.is_empty()
            && ui
                .ctx()
                .input(|i| rect.contains(i.pointer.interact_pos().unwrap_or_default()));

        let bg = if app.source_file.is_some() {
            palette.success.linear_multiply(0.12)
        } else if is_drag_over {
            palette.secondary.linear_multiply(0.12)
        } else {
            ui.visuals().faint_bg_color
        };

        ui.painter()
            .rect_filled(rect, 12.0, bg);
        ui.painter().rect_stroke(
            rect,
            12.0,
            egui::Stroke::new(
                2.0,
                if app.source_file.is_some() {
                    palette.success
                } else if is_drag_over {
                    palette.secondary
                } else {
                    ui.visuals().widgets.inactive.bg_stroke.color
                },
            ),
        );

        if !dropped_files.is_empty() {
            app.set_selected_file(dropped_files[0].clone());
        }

        if resp.clicked() {
            if let Some(file_path) = rfd::FileDialog::new()
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
                app.set_selected_file(file_path);
            }
        }

        ui.allocate_ui_at_rect(rect.shrink(10.0), |ui| {
            if let Some(ref f) = app.source_file {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Selected").strong().color(palette.success));
                    ui.label(RichText::new(ModernApp::sanitize_path(f)).strong());
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Drag & Drop").size(26.0).strong());
                    ui.label(RichText::new("or click to browse").color(palette.secondary));
                });
            }
        });
    });

    ui.add_space(12.0);

    // Preview (kept as a first-class UI element)
    let preview_header = egui::CollapsingHeader::new("Preview")
        .default_open(app.show_preview)
        .show(ui, |ui| {
            ui.add_space(8.0);
            let Some(ref source_file) = app.source_file else {
                ui.label(RichText::new("No file selected").italics());
                return;
            };

            let limits = app.limits();
            match app.detected_file_type {
                Some(FileType::Image) => {
                    match converter_gui::ui::preview::get_or_generate_preview(
                        source_file,
                        520,
                        360,
                        &limits,
                        &app.preview_cache,
                    ) {
                        Ok(preview) => {
                            let texture_id = format!("preview:{}", source_file.display());
                            let texture = ui.ctx().load_texture(
                                texture_id,
                                preview.image.clone(),
                                Default::default(),
                            );
                            ui.image((texture.id(), texture.size_vec2()));
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(format!(
                                    "Original: {}×{}",
                                    preview.original_width, preview.original_height
                                ))
                                .small()
                                .color(egui::Color32::GRAY),
                            );
                        }
                        Err(_) => {
                            ui.label("Preview not available");
                        }
                    }
                }
                Some(FileType::Mesh) => {
                    match converter_gui::ui::preview::get_mesh_metadata(source_file, &limits) {
                        Ok(meta) => {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Format").strong());
                                ui.label(format!("{:?}", meta.format));
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Vertices").strong());
                                ui.label(format!("{}", meta.vertex_count));
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Faces").strong());
                                ui.label(format!("{}", meta.face_count));
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Normals").strong());
                                ui.label(if meta.has_normals { "Yes" } else { "No" });
                            });
                        }
                        Err(_) => {
                            ui.label("Mesh metadata not available");
                        }
                    }
                }
                None => {
                    ui.label("No file selected for preview");
                }
            }
        });
    if preview_header.header_response.clicked() {
        app.show_preview = !app.show_preview;
    }

    ui.add_space(12.0);

    // Format + options
    ui.horizontal(|ui| {
        let left_w = (ui.available_width() * 0.42).clamp(260.0, 360.0);
        let right_w = ui.available_width() - left_w - 16.0;

        ui.vertical(|ui| {
            ui.set_width(left_w);
            ui.group(|ui| {
                ui.heading("Output Format");
                ui.add_space(8.0);

                let Some(ft) = app.detected_file_type else {
                    ui.label(RichText::new("Select a file first").italics());
                    return;
                };

                match ft {
                    FileType::Image => {
                        for fmt in converter_gui::format_helpers::get_writable_image_formats() {
                            let label = converter_gui::format_helpers::get_image_format_name(fmt);
                            let resp = ui.radio_value(
                                &mut app.output_format,
                                Some(OutputFormat::Image(fmt)),
                                label,
                            );
                            if resp.changed() {
                                // update output filename extension
                                if let Some(ref src) = app.source_file {
                                    if let Some(stem) = src.file_stem().and_then(|s| s.to_str()) {
                                        let ext =
                                            converter_gui::format_helpers::get_format_extension(fmt);
                                        app.output_filename = format!("{stem}.{ext}");
                                    }
                                }
                            }
                            ui.add_space(2.0);
                        }
                    }
                    FileType::Mesh => {
                        for fmt in converter_gui::format_helpers::get_writable_mesh_formats() {
                            let label = converter_gui::format_helpers::get_mesh_format_name(fmt);
                            let resp = ui.radio_value(
                                &mut app.output_format,
                                Some(OutputFormat::Mesh(fmt)),
                                label,
                            );
                            if resp.changed() {
                                if let Some(ref src) = app.source_file {
                                    if let Some(stem) = src.file_stem().and_then(|s| s.to_str()) {
                                        let ext = converter_gui::format_helpers::get_mesh_format_extension(fmt);
                                        app.output_filename = format!("{stem}.{ext}");
                                    }
                                }
                            }
                            ui.add_space(2.0);
                        }
                    }
                }
            });
        });

        ui.add_space(16.0);

        ui.vertical(|ui| {
            ui.set_width(right_w.max(260.0));
            ui.group(|ui| {
                ui.heading("Options");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Output filename");
                    ui.text_edit_singleline(&mut app.output_filename);
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Output folder");
                    ui.label(RichText::new(ModernApp::sanitize_path(&app.output_directory)).small());
                    if ui.button("Browse…").clicked() {
                        if let Some(selected_dir) = rfd::FileDialog::new().pick_folder() {
                            if common::validation::validate_directory_path(&selected_dir).is_ok() {
                                app.output_directory = selected_dir;
                            } else {
                                app.add_message("Invalid output folder.", MessageType::Error);
                            }
                        }
                    }
                });

                ui.add_space(8.0);

                if let Some(OutputFormat::Image(fmt)) = app.output_format {
                    if converter_gui::format_helpers::format_supports_quality(fmt) {
                        ui.label(RichText::new(format!("Quality: {}", app.quality)).strong());
                        ui.add(egui::Slider::new(&mut app.quality, 1..=100));
                    }
                }

                if matches!(app.detected_file_type, Some(FileType::Mesh)) {
                    ui.separator();
                    ui.label(RichText::new("Mesh Options").strong());
                    ui.add_space(6.0);

                    ui.label("Coordinate transform");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut app.mesh_transform, None, "None");
                        ui.radio_value(
                            &mut app.mesh_transform,
                            Some((mesh_core::CoordinateSystem::ZUp, mesh_core::CoordinateSystem::YUp)),
                            "Z-up → Y-up",
                        );
                        ui.radio_value(
                            &mut app.mesh_transform,
                            Some((mesh_core::CoordinateSystem::YUp, mesh_core::CoordinateSystem::ZUp)),
                            "Y-up → Z-up",
                        );
                    });
                    ui.add_space(6.0);
                    ui.checkbox(&mut app.mesh_recalculate_normals, "Recalculate normals");
                    ui.checkbox(&mut app.mesh_validate, "Validate mesh");
                }

                ui.separator();
                ui.collapsing("Advanced safety limits", |ui| {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("Max file size (MB)");
                        ui.add(egui::Slider::new(&mut app.max_file_size_mb, 1..=1024));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Max image dimension (px)");
                        ui.add(egui::Slider::new(&mut app.max_dimension, 1000..=65535));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Max vertices");
                        ui.add(egui::Slider::new(&mut app.max_vertices, 1000..=10_000_000));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Max faces");
                        ui.add(egui::Slider::new(&mut app.max_faces, 1000..=10_000_000));
                    });
                });
            });
        });
    });

    ui.add_space(16.0);

    // Actions
    ui.horizontal(|ui| {
        let can_convert = app.source_file.is_some()
            && app.output_format.is_some()
            && !matches!(app.status, Status::Converting { .. });

        if ui.button("Clear").clicked() {
            app.source_file = None;
            app.detected_file_type = None;
            app.input_format = None;
            app.output_format = None;
            app.output_filename.clear();
            app.status = Status::Ready;
            app.single_result_rx = None;
        }

        ui.add_enabled_ui(can_convert, |ui| {
            if ui.button(RichText::new("Convert").strong()).clicked() {
                app.start_single_conversion();
            }
        });
    });

    ui.add_space(14.0);
    render_history_panel(ui, app);
}

fn render_batch_mode(ui: &mut Ui, app: &mut ModernApp, ctx: &egui::Context) {
    ui.heading("Batch Processing");
    ui.add_space(10.0);

    // Batch defaults (visible; applies to newly-added items)
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Defaults (applies to newly added files)").strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Reset defaults").clicked() {
                    app.batch_defaults = BatchDefaults::default();
                    app.batch_defaults.output_directory = app.settings.default_output_directory.clone();
                }
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Output folder");
            let label = app
                .batch_defaults
                .output_directory
                .as_ref()
                .map(|p| ModernApp::sanitize_path(p))
                .unwrap_or_else(|| "(use each file's folder)".to_string());
            ui.label(RichText::new(label).small());
            if ui.button("Browse…").clicked() {
                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                    if common::validation::validate_directory_path(&dir).is_ok() {
                        app.batch_defaults.output_directory = Some(dir);
                    } else {
                        app.add_message("Invalid output folder.", MessageType::Error);
                    }
                }
            }
            if ui.button("Clear").clicked() {
                app.batch_defaults.output_directory = None;
            }
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Default image format");
            egui::ComboBox::from_id_source("default_img_fmt")
                .selected_text(format!("{:?}", app.batch_defaults.default_image_format))
                .show_ui(ui, |ui| {
                    for fmt in converter_gui::format_helpers::get_writable_image_formats() {
                        ui.selectable_value(
                            &mut app.batch_defaults.default_image_format,
                            fmt,
                            converter_gui::format_helpers::get_image_format_name(fmt),
                        );
                    }
                });

            ui.add_space(10.0);

            ui.label("Default mesh format");
            egui::ComboBox::from_id_source("default_mesh_fmt")
                .selected_text(format!("{:?}", app.batch_defaults.default_mesh_format))
                .show_ui(ui, |ui| {
                    for fmt in converter_gui::format_helpers::get_writable_mesh_formats() {
                        ui.selectable_value(
                            &mut app.batch_defaults.default_mesh_format,
                            fmt,
                            converter_gui::format_helpers::get_mesh_format_name(fmt),
                        );
                    }
                });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Default quality: {}", app.batch_defaults.quality)).strong());
            ui.add(egui::Slider::new(&mut app.batch_defaults.quality, 1..=100));
        });

        ui.add_space(8.0);

        ui.collapsing("Mesh defaults", |ui| {
            ui.add_space(6.0);
            ui.label("Coordinate transform");
            ui.horizontal(|ui| {
                ui.radio_value(&mut app.batch_defaults.mesh_transform, None, "None");
                ui.radio_value(
                    &mut app.batch_defaults.mesh_transform,
                    Some((mesh_core::CoordinateSystem::ZUp, mesh_core::CoordinateSystem::YUp)),
                    "Z-up → Y-up",
                );
                ui.radio_value(
                    &mut app.batch_defaults.mesh_transform,
                    Some((mesh_core::CoordinateSystem::YUp, mesh_core::CoordinateSystem::ZUp)),
                    "Y-up → Z-up",
                );
            });
            ui.add_space(6.0);
            ui.checkbox(
                &mut app.batch_defaults.mesh_recalculate_normals,
                "Recalculate normals",
            );
            ui.checkbox(&mut app.batch_defaults.mesh_validate, "Validate mesh");
        });
    });

    ui.add_space(14.0);

    // Queue controls + queue list
    ui.group(|ui| {
        // Drop area for batch adds
        ui.label(RichText::new("Drop files here to add to queue").strong());
        let w = ui.available_width();
        let resp = ui.allocate_response(egui::vec2(w, 90.0), egui::Sense::click());
        let rect = resp.rect;
        let palette = crate::ui::theme::Palette::default();

        let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
        let dropped_files = ui
            .ctx()
            .input(|i| i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect::<Vec<_>>());
        let is_drag_over = !hovered_files.is_empty()
            && ui
                .ctx()
                .input(|i| rect.contains(i.pointer.interact_pos().unwrap_or_default()));

        ui.painter().rect_filled(
            rect,
            12.0,
            if is_drag_over {
                palette.secondary.linear_multiply(0.12)
            } else {
                ui.visuals().faint_bg_color
            },
        );
        ui.painter().rect_stroke(
            rect,
            12.0,
            egui::Stroke::new(
                2.0,
                if is_drag_over {
                    palette.secondary
                } else {
                    ui.visuals().widgets.inactive.bg_stroke.color
                },
            ),
        );

        if !dropped_files.is_empty() {
            app.add_files_to_batch(dropped_files);
        }

        if resp.clicked() {
            let mut dialog = rfd::FileDialog::new()
                .add_filter(
                    "Image Files",
                    &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "tif", "webp", "svg"],
                )
                .add_filter(
                    "Mesh Files",
                    &["stl", "obj", "ply", "off", "gltf", "glb", "dxf", "step", "stp"],
                )
                .add_filter("All Files", &["*"]);

            if let Some(dir) = app.batch_defaults.output_directory.as_ref() {
                if let Ok(canon) = dir.canonicalize() {
                    dialog = dialog.set_directory(canon);
                }
            }

            if let Some(files) = dialog.pick_files() {
                app.add_files_to_batch(files);
            }
        }

        ui.allocate_ui_at_rect(rect.shrink(10.0), |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Add Files").size(22.0).strong());
                ui.label(RichText::new("Click or drop a bunch of files here").color(palette.secondary));
            });
        });

        ui.add_space(12.0);

        // Buttons
        ui.horizontal(|ui| {
            if ui.button("Add Files…").clicked() {
                if let Some(files) = rfd::FileDialog::new().pick_files() {
                    app.add_files_to_batch(files);
                }
            }

            if ui.button("Clear Queue").clicked() {
                app.batch_queue.clear();
                app.add_message("Batch queue cleared", MessageType::Info);
            }

            let has_pending = app.batch_queue.has_pending();
            let is_processing = app.batch_processing_state.is_some();

            ui.add_enabled_ui(has_pending && !is_processing, |ui| {
                if ui.button(RichText::new("Process Queue").strong()).clicked() {
                    match app.start_batch_processing(ctx.clone()) {
                        Ok(()) => app.add_message("Batch processing started", MessageType::Info),
                        Err(e) => app.add_message(e, MessageType::Error),
                    }
                }
            });

            ui.add_enabled_ui(is_processing, |ui| {
                if app.is_batch_paused() {
                    if ui.button("▶ Resume").clicked() {
                        if let Err(e) = app.resume_batch() {
                            app.add_message(e, MessageType::Error);
                        }
                    }
                } else if ui.button("⏸ Pause").clicked() {
                    if let Err(e) = app.pause_batch() {
                        app.add_message(e, MessageType::Error);
                    }
                }

                if ui.button("⏹ Cancel").clicked() {
                    if let Err(e) = app.cancel_batch() {
                        app.add_message(e, MessageType::Error);
                    } else {
                        app.add_message("Batch cancellation requested", MessageType::Info);
                    }
                }
            });
        });

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(10.0);

        let stats = app.batch_queue.statistics();
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Total: {}", stats.total)).strong());
            ui.separator();
            ui.label(RichText::new(format!("Pending: {}", stats.pending)).strong());
            ui.separator();
            ui.label(RichText::new(format!("Processing: {}", stats.processing)).strong());
            ui.separator();
            ui.label(RichText::new(format!("Done: {}", stats.completed)).strong());
            ui.separator();
            ui.label(RichText::new(format!("Failed: {}", stats.failed)).strong());
        });

        ui.add_space(10.0);

        if app.batch_queue.is_empty() {
            ui.label(RichText::new("No files in queue").italics());
            return;
        }

        egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| {
            let mut to_remove: Vec<Uuid> = Vec::new();
            for item in app.batch_queue.items.iter() {
                let (icon, color) = match &item.status {
                    BatchItemStatus::Pending => ("⏳", egui::Color32::GRAY),
                    BatchItemStatus::Processing => ("⚙", crate::ui::theme::Palette::default().secondary),
                    BatchItemStatus::Completed { .. } => ("✓", crate::ui::theme::Palette::default().success),
                    BatchItemStatus::Failed { .. } => ("✗", crate::ui::theme::Palette::default().error),
                    BatchItemStatus::Cancelled => ("⊘", egui::Color32::GRAY),
                };

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(icon).color(color).size(18.0).strong());

                        let filename = item
                            .source_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown");
                        ui.label(RichText::new(filename).strong());

                        ui.label("→");
                        ui.label(format!("{:?}", item.output_format));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let can_edit = matches!(item.status, BatchItemStatus::Pending);
                            let can_remove = !matches!(item.status, BatchItemStatus::Processing);

                            ui.add_enabled_ui(can_remove, |ui| {
                                if ui.small_button("Remove").clicked() {
                                    to_remove.push(item.id);
                                }
                            });

                            ui.add_enabled_ui(can_edit, |ui| {
                                if ui.small_button("Edit").clicked() {
                                    app.editing_queue_item = Some(item.id);
                                }
                            });
                        });
                    });

                    match &item.status {
                        BatchItemStatus::Failed { error } => {
                            ui.label(RichText::new(error).small().color(
                                crate::ui::theme::Palette::default().error,
                            ));
                        }
                        BatchItemStatus::Completed { output_path } => {
                            ui.label(
                                RichText::new(format!("Output: {}", ModernApp::sanitize_path(output_path)))
                                    .small()
                                    .color(egui::Color32::GRAY),
                            );
                        }
                        BatchItemStatus::Processing => {
                            ui.add(egui::ProgressBar::new(item.progress).show_percentage());
                        }
                        _ => {}
                    }
                });
                ui.add_space(8.0);
            }

            for id in to_remove {
                app.batch_queue.remove_item(id);
            }
        });
    });

    ui.add_space(14.0);
    render_history_panel(ui, app);
}

fn render_history_panel(ui: &mut Ui, app: &mut ModernApp) {
    let enabled = app.settings.conversion_history_enabled;
    let header_text = if enabled {
        "Conversion History"
    } else {
        "Conversion History (disabled in Preferences)"
    };

    egui::CollapsingHeader::new(header_text)
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(8.0);
            if !enabled {
                ui.label(
                    RichText::new("Enable Conversion History in Preferences to record results.")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
                return;
            }

            if app.history.entries.is_empty() {
                ui.label(RichText::new("No conversion history yet").italics());
                return;
            }

            let mut open_path: Option<PathBuf> = None;
            let mut remove_indices: Vec<usize> = Vec::new();

            ui.horizontal(|ui| {
                if ui.button("Clear History").clicked() {
                    app.history.clear();
                }
            });

            ui.add_space(10.0);

            egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                for (idx, entry) in app.history.entries.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let color = if entry.success {
                                crate::ui::theme::Palette::default().success
                            } else {
                                crate::ui::theme::Palette::default().error
                            };
                            ui.label(
                                RichText::new(if entry.success { "✓" } else { "✗" })
                                    .color(color)
                                    .strong(),
                            );
                            ui.label(RichText::new(entry.source_filename()).strong());
                            ui.label("→");
                            ui.label(RichText::new(entry.output_format.clone()).small());

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("Remove").clicked() {
                                        remove_indices.push(idx);
                                    }
                                    if entry.success && ui.small_button("Open Output").clicked() {
                                        open_path = Some(entry.output_path.clone());
                                    }
                                },
                            );
                        });

                        ui.label(
                            RichText::new(entry.formatted_timestamp())
                                .small()
                                .color(egui::Color32::GRAY),
                        );

                        if !entry.success {
                            if let Some(ref err) = entry.error {
                                ui.label(
                                    RichText::new(err)
                                        .small()
                                        .color(crate::ui::theme::Palette::default().error),
                                );
                            }
                        }
                    });
                    ui.add_space(6.0);
                }
            });

            // Apply removals after render (reverse order).
            for idx in remove_indices.into_iter().rev() {
                app.history.remove_entry(idx);
            }

            // Open output file outside of list iteration.
            if let Some(path) = open_path {
                if let Err(e) = common::validation::validate_file_path(&path) {
                    app.add_message(format!("Cannot open file: {e}"), MessageType::Error);
                } else if path.exists() {
                    if let Err(e) = open::that(&path) {
                        app.add_message(format!("Failed to open file: {e}"), MessageType::Error);
                    }
                } else {
                    app.add_message("Output file not found.", MessageType::Error);
                }
            }
        });
}

fn update_path_extension_for_format(path_str: &str, fmt: OutputFormat) -> String {
    let mut path = PathBuf::from(path_str);
    let ext = match fmt {
        OutputFormat::Image(img_fmt) => converter_gui::format_helpers::get_format_extension(img_fmt),
        OutputFormat::Mesh(mesh_fmt) => converter_gui::format_helpers::get_mesh_format_extension(mesh_fmt),
    };
    path.set_extension(ext);
    path.to_string_lossy().to_string()
}

fn render_edit_queue_item(ui: &mut Ui, app: &mut ModernApp, id: Uuid) {
    let Some(item) = app.batch_queue.get_item(id).cloned() else {
        app.editing_queue_item = None;
        app.editing_queue_item_draft = None;
        return;
    };

    // Only allow editing pending items (clear, obvious behavior).
    if !matches!(item.status, BatchItemStatus::Pending) {
        app.editing_queue_item = None;
        app.editing_queue_item_draft = None;
        return;
    }

    // Keep a persistent draft so edits (radio selection, typed path, sliders) survive across frames.
    let needs_new_draft = app
        .editing_queue_item_draft
        .as_ref()
        .map(|d| d.id != id)
        .unwrap_or(true);
    if needs_new_draft {
        app.editing_queue_item_draft = Some(QueueItemEditDraft {
            id,
            output_format: item.output_format,
            output_path_str: item.output_path.to_string_lossy().to_string(),
            quality: item.options.quality,
            mesh_options: item.options.mesh_options.clone(),
            priority: item.options.priority,
        });
    }

    let palette = crate::ui::theme::Palette::default();
    ui.label(RichText::new("Edit item").strong().color(palette.secondary));
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Source");
        ui.label(RichText::new(
            item.source_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown"),
        )
        .strong());
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // Format selection
    ui.label(RichText::new("Output format").strong());
    let mut draft = match app.editing_queue_item_draft.clone() {
        Some(d) => d,
        None => {
            app.editing_queue_item = None;
            app.editing_queue_item_draft = None;
            return;
        }
    };
    match item.file_type {
        FileType::Image => {
            for fmt in converter_gui::format_helpers::get_writable_image_formats() {
                let resp = ui.radio_value(
                    &mut draft.output_format,
                    OutputFormat::Image(fmt),
                    converter_gui::format_helpers::get_image_format_name(fmt),
                );
                if resp.changed() {
                    draft.output_path_str = update_path_extension_for_format(
                        &draft.output_path_str,
                        draft.output_format,
                    );
                }
            }
        }
        FileType::Mesh => {
            for fmt in converter_gui::format_helpers::get_writable_mesh_formats() {
                let resp = ui.radio_value(
                    &mut draft.output_format,
                    OutputFormat::Mesh(fmt),
                    converter_gui::format_helpers::get_mesh_format_name(fmt),
                );
                if resp.changed() {
                    draft.output_path_str = update_path_extension_for_format(
                        &draft.output_path_str,
                        draft.output_format,
                    );
                }
            }
        }
    }

    ui.add_space(8.0);

    // Output path
    ui.label(RichText::new("Output path").strong());
    ui.text_edit_singleline(&mut draft.output_path_str);
    if ui.button("Browse…").clicked() {
        let mut dialog = rfd::FileDialog::new();
        if let Some(parent) = item.output_path.parent() {
            if let Ok(canon) = parent.canonicalize() {
                dialog = dialog.set_directory(canon);
            }
        }
        if let Some(path) = dialog.save_file() {
            draft.output_path_str = path.to_string_lossy().to_string();
        }
    }

    ui.add_space(8.0);

    // Quality (images only, if supported)
    if matches!(item.file_type, FileType::Image) {
        if let OutputFormat::Image(fmt) = draft.output_format {
            if converter_gui::format_helpers::format_supports_quality(fmt) {
                ui.label(RichText::new(format!("Quality: {}", draft.quality)).strong());
                ui.add(egui::Slider::new(&mut draft.quality, 1..=100));
            }
        }
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    let mut cancel_clicked = false;
    let mut save_clicked = false;
    ui.horizontal(|ui| {
        if ui.button("Cancel").clicked() {
            cancel_clicked = true;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(RichText::new("Save").strong()).clicked() {
                save_clicked = true;
            }
        });
    });

    if cancel_clicked {
        app.editing_queue_item = None;
        app.editing_queue_item_draft = None;
        return;
    }

    if save_clicked {
        let output_path = PathBuf::from(&draft.output_path_str);
        let output_dir_valid = output_path
            .parent()
            .map(|p| common::validation::validate_directory_path(p).is_ok())
            .unwrap_or(false);
        let not_system_dir =
            converter_gui::utils::validate_output_path_not_system(&output_path).is_ok();

        if !output_dir_valid || !not_system_dir {
            app.add_message(
                "Invalid output path. Please choose a valid non-system directory.",
                MessageType::Error,
            );
            app.editing_queue_item_draft = Some(draft);
            return;
        }

        // Commit changes (explicit, visible action)
        if draft.output_format != item.output_format {
            app.batch_queue.update_item_format(id, draft.output_format);
        }
        if output_path != item.output_path {
            app.batch_queue
                .update_item_output_path(id, output_path);
        }

        if draft.quality != item.options.quality
            || draft.mesh_options != item.options.mesh_options
            || draft.priority != item.options.priority
        {
            let new_opts = BatchItemOptions {
                quality: draft.quality,
                mesh_options: draft.mesh_options.clone(),
                priority: draft.priority,
            };
            app.batch_queue.update_item_options(id, new_opts);
        }

        app.add_message("Queue item updated", MessageType::Success);
        app.editing_queue_item = None;
        app.editing_queue_item_draft = None;
        return;
    }

    // Persist edits across frames while the dialog is open.
    app.editing_queue_item_draft = Some(draft);
}

fn render_preferences(ui: &mut Ui, app: &mut ModernApp) {
    ui.label(RichText::new("These settings are shared with the classic GUI.").small().color(
        egui::Color32::GRAY,
    ));
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.heading("General");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Default output folder");
            let label = app
                .settings
                .default_output_directory
                .as_ref()
                .map(|p| ModernApp::sanitize_path(p))
                .unwrap_or_else(|| "(use source file folder)".to_string());
            ui.label(RichText::new(label).small());
            if ui.button("Browse…").clicked() {
                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                    if common::validation::validate_directory_path(&dir).is_ok() {
                        app.settings.default_output_directory = Some(dir.clone());
                        app.output_directory = dir;
                        app.batch_defaults.output_directory = app.settings.default_output_directory.clone();
                        let _ = app.settings.save();
                        app.add_message("Settings saved", MessageType::Success);
                    } else {
                        app.add_message("Invalid output folder.", MessageType::Error);
                    }
                }
            }
            if ui.button("Clear").clicked() {
                app.settings.default_output_directory = None;
                let _ = app.settings.save();
                app.add_message("Settings saved", MessageType::Success);
            }
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Default quality");
            let mut q = app.settings.default_quality;
            let resp = ui.add(egui::Slider::new(&mut q, 1..=100));
            if resp.changed() {
                app.settings.default_quality = q;
                app.quality = q;
                app.batch_defaults.quality = q;
                let _ = app.settings.save();
            }
            ui.label(format!("{q}"));
        });
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.heading("Conversion");
        ui.add_space(8.0);

        // History
        ui.horizontal(|ui| {
            let mut enabled = app.settings.conversion_history_enabled;
            let resp = ui.checkbox(&mut enabled, "Enable Conversion History");
            if resp.changed() {
                app.settings.conversion_history_enabled = enabled;
                let _ = app.settings.save();
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Max history entries");
            let mut max_entries = app.settings.max_history_entries;
            let resp = ui.add(egui::Slider::new(&mut max_entries, 10..=1000));
            if resp.changed() {
                app.settings.max_history_entries = max_entries;
                app.history.max_entries = max_entries;
                app.history.entries.truncate(max_entries);
                let _ = app.settings.save();
            }
            ui.label(format!("{max_entries}"));
        });

        ui.add_space(10.0);

        ui.separator();
        ui.add_space(10.0);

        ui.heading("Batch");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Max concurrent conversions");
            let mut val = app.settings.max_concurrent_conversions.unwrap_or(app.max_concurrent());
            let resp = ui.add(egui::Slider::new(&mut val, 1..=16));
            if resp.changed() {
                app.settings.max_concurrent_conversions = Some(val);
                let _ = app.settings.save();
                app.add_message("Settings saved", MessageType::Success);
            }
            ui.label(format!("{val}"));
        });
    });

    ui.add_space(10.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if ui.button("Close").clicked() {
            app.settings_pane = SettingsPane::None;
        }
    });
}

fn render_help(ui: &mut Ui) {
    ui.heading("Help");
    ui.add_space(10.0);
    ui.label("Single mode: pick a file, choose output format, convert.");
    ui.label("Batch mode: set defaults, add files, then Process Queue.");
    ui.add_space(10.0);
    ui.label(RichText::new("Tip: In Batch mode, defaults apply only to newly-added files.").strong());
}

fn render_about(ui: &mut Ui) {
    ui.heading("Simple Image Converter (Modern)");
    ui.add_space(10.0);
    ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).strong());
    ui.add_space(10.0);
    ui.label("Alternate UI/UX fork built with egui/eframe.");
    ui.label("Conversion engines are shared with the classic GUI.");
    ui.add_space(10.0);
    ui.hyperlink_to(
        "GitHub Repository",
        "https://github.com/BelongaGezza/SimpleImageConverter",
    );
}

