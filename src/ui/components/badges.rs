use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};
use crate::ui::theme::*;

pub fn status_pill(ui: &mut Ui, text: &str, bg: Color32, border: Color32, fg: Color32) {
    Frame::new()
        .fill(bg)
        .corner_radius(CornerRadius::same(12))
        .stroke(Stroke::new(1.0, border))
        .inner_margin(Margin::symmetric(8, 3))
        .show(ui, |ui| {
            ui.colored_label(fg, text);
        });
}

pub fn dominance_badge(ui: &mut Ui, is_square: bool, is_dominant: bool) {
    if !is_square {
        status_pill(
            ui,
            "📐 Matriz No Cuadrada",
            Color32::from_rgb(30, 41, 59),
            Color32::from_rgb(71, 85, 105),
            Color32::from_rgb(148, 163, 184),
        );
    } else if is_dominant {
        status_pill(
            ui,
            "✓ Diagonalmente Dominante",
            COLOR_SUCCESS_BG,
            COLOR_SUCCESS_BORDER,
            COLOR_SUCCESS_TEXT,
        );
    } else {
        status_pill(
            ui,
            "⚠ No Diagonalmente Dominante",
            COLOR_WARNING_BG,
            COLOR_WARNING_BORDER,
            COLOR_WARNING_TEXT,
        );
    }
}

pub fn dimension_badge(ui: &mut Ui, rows: usize, cols: usize) {
    let text = format!("{} × {}", rows, cols);
    status_pill(
        ui,
        &text,
        Color32::from_rgb(33, 41, 60),
        COLOR_CARD_BORDER,
        COLOR_ACCENT_CYAN,
    );
}

pub fn fastest_badge(ui: &mut Ui) {
    status_pill(
        ui,
        "⚡ Más Rápido",
        COLOR_SUCCESS_BG,
        COLOR_SUCCESS_BORDER,
        COLOR_SUCCESS_TEXT,
    );
}
