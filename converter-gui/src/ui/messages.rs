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
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Messages:").strong());
            if !app.messages.is_empty() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{} messages", app.messages.len()))
                            .small()
                            .color(Color32::GRAY),
                    );
                });
            }
        });

        ui.add_space(5.0);

        if app.messages.is_empty() {
            ui.label(
                egui::RichText::new("No messages")
                    .italics()
                    .color(Color32::GRAY)
                    .small(),
            );
        } else {
            ScrollArea::vertical()
                .max_height(150.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Show messages in reverse order (newest first)
                        for message in app.messages.iter().rev() {
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

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon).size(16.0).color(color));
                                ui.label(egui::RichText::new(&message.text).color(color));
                            });
                            ui.add_space(3.0);
                        }
                    });
                });
        }
    });
}
