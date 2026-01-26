// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Modern theme for the alternate GUI.

use egui::{Color32, FontFamily, FontId, TextStyle, Visuals};

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub primary: Color32,
    pub secondary: Color32,
    pub success: Color32,
    pub warn: Color32,
    pub error: Color32,
    pub bg: Color32,
    pub panel: Color32,
}

impl Default for Palette {
    fn default() -> Self {
        // Bold, modern, high-contrast (dark-first).
        Self {
            primary: Color32::from_rgb(120, 90, 255),
            secondary: Color32::from_rgb(40, 200, 255),
            success: Color32::from_rgb(0, 220, 140),
            warn: Color32::from_rgb(255, 195, 0),
            error: Color32::from_rgb(255, 70, 90),
            bg: Color32::from_rgb(14, 14, 18),
            panel: Color32::from_rgb(24, 24, 32),
        }
    }
}

pub fn apply_modern_theme(ctx: &egui::Context) {
    let palette = Palette::default();

    // Fonts: keep default families, but scale sizes up.
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(28.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("H2".into()),
            FontId::new(22.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(18.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(16.0, FontFamily::Monospace),
        ),
    ]
    .into();

    // Spacing + rounding for bigger, touch-friendlier UI.
    style.spacing.button_padding = egui::vec2(12.0, 10.0);
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.visuals.window_rounding = egui::Rounding::same(10.0);
    style.visuals.menu_rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(10.0);

    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(Color32::from_rgb(235, 235, 245));
    visuals.panel_fill = palette.panel;
    visuals.window_fill = palette.panel;
    visuals.extreme_bg_color = palette.bg;
    visuals.faint_bg_color = Color32::from_rgb(30, 30, 40);

    // Accent widgets.
    visuals.selection.bg_fill = palette.primary.linear_multiply(0.65);
    visuals.selection.stroke = egui::Stroke::new(1.5, palette.primary);
    visuals.widgets.hovered.bg_fill = palette.primary.linear_multiply(0.25);
    visuals.widgets.active.bg_fill = palette.primary.linear_multiply(0.40);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, Color32::WHITE);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.2, Color32::WHITE);

    style.visuals = visuals;

    ctx.set_style(style);
}
