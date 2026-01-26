// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use egui::{Color32, RichText, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingMode {
    Single,
    Batch,
}

impl ProcessingMode {
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            ProcessingMode::Single => "Single File",
            ProcessingMode::Batch => "Batch",
        }
    }
}

/// Segmented mode switch (Single vs Batch).
pub fn render_mode_switch(ui: &mut Ui, mode: &mut ProcessingMode) {
    let desired_height = 34.0;
    ui.set_height(desired_height);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        let selected_bg = ui.visuals().selection.bg_fill;
        let hovered_bg = ui.visuals().widgets.hovered.bg_fill;
        let inactive_bg = ui.visuals().widgets.inactive.bg_fill;
        let stroke = ui.visuals().widgets.inactive.bg_stroke;

        let button = |ui: &mut Ui, label: &str, selected: bool| -> bool {
            let mut b = egui::Button::new(RichText::new(label).strong().color(if selected {
                Color32::WHITE
            } else {
                ui.visuals().text_color()
            }))
            .min_size(egui::vec2(150.0, desired_height));

            if selected {
                b = b
                    .fill(selected_bg)
                    .stroke(egui::Stroke::new(1.5, stroke.color));
            } else {
                b = b.fill(inactive_bg).stroke(stroke);
            }

            let resp = ui.add(b);
            if resp.hovered() && !selected {
                ui.painter().rect_filled(resp.rect, 10.0, hovered_bg);
            }
            resp.clicked()
        };

        let single_selected = *mode == ProcessingMode::Single;
        let batch_selected = *mode == ProcessingMode::Batch;

        if button(ui, "Single", single_selected) {
            *mode = ProcessingMode::Single;
        }
        if button(ui, "Batch", batch_selected) {
            *mode = ProcessingMode::Batch;
        }
    });
}
