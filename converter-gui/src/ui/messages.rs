// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Messages display component

use crate::app::{ConverterApp, MessageType};
use egui::{Color32, ScrollArea, Ui};

/// Render messages area
///
/// Displays a scrollable list of messages with color coding:
/// - Info: Blue
/// - Warning: Yellow
/// - Error: Red
/// - Success: Green
pub fn render_messages(ui: &mut Ui, app: &ConverterApp) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Messages:").strong());

        if app.messages.is_empty() {
            ui.label(egui::RichText::new("No messages").color(Color32::GRAY));
        } else {
            ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for message in &app.messages {
                        let color = match message.message_type {
                            MessageType::Info => Color32::from_rgb(0, 100, 255),
                            MessageType::Warning => Color32::from_rgb(255, 200, 0),
                            MessageType::Error => Color32::from_rgb(255, 0, 0),
                            MessageType::Success => Color32::from_rgb(0, 200, 0),
                        };

                        let icon = match message.message_type {
                            MessageType::Info => "ℹ",
                            MessageType::Warning => "⚠",
                            MessageType::Error => "✗",
                            MessageType::Success => "✓",
                        };

                        ui.label(
                            egui::RichText::new(format!("{} {}", icon, message.text)).color(color),
                        );
                        ui.separator();
                    }
                });
            });
        }
    });
}
