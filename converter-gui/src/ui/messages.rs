// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Messages display component

use crate::app::{ConverterApp, MessageType};
use crate::ui::style;
use egui::{ScrollArea, Ui};

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
                            .color(style::colors::ui::SECONDARY_TEXT),
                    );
                });
            }
        });

        ui.add_space(style::spacing::MEDIUM);

        if app.messages.is_empty() {
            ui.label(
                egui::RichText::new("No messages")
                    .italics()
                    .color(style::colors::ui::PLACEHOLDER_TEXT)
                    .small(),
            );
        } else {
            ScrollArea::vertical()
                .max_height(style::scroll::MESSAGES_MAX_HEIGHT)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Show messages in reverse order (newest first)
                        for message in app.messages.iter().rev() {
                            let color = match message.message_type {
                                MessageType::Info => style::colors::message::INFO,
                                MessageType::Warning => style::colors::message::WARNING,
                                MessageType::Error => style::colors::message::ERROR,
                                MessageType::Success => style::colors::message::SUCCESS,
                            };

                            let icon = match message.message_type {
                                MessageType::Info => style::icons::INFO,
                                MessageType::Warning => style::icons::WARNING,
                                MessageType::Error => style::icons::ERROR,
                                MessageType::Success => style::icons::SUCCESS,
                            };

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon).size(16.0).color(color));
                                ui.label(egui::RichText::new(&message.text).color(color));
                            });
                            ui.add_space(style::spacing::SMALL);
                        }
                    });
                });
        }
    });
}
