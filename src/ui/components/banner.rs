use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};
use crate::ui::theme::*;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum BannerKind {
    Success,
    Warning,
    Error,
    Info,
}

pub fn show_banner(ui: &mut Ui, kind: &BannerKind, message: &str) {
    let (bg, border, fg, icon) = match kind {
        BannerKind::Success => (
            COLOR_SUCCESS_BG,
            COLOR_SUCCESS_BORDER,
            COLOR_SUCCESS_TEXT,
            "✅",
        ),
        BannerKind::Warning => (
            COLOR_WARNING_BG,
            COLOR_WARNING_BORDER,
            COLOR_WARNING_TEXT,
            "⚠️",
        ),
        BannerKind::Error => (
            COLOR_DANGER_BG,
            COLOR_DANGER_BORDER,
            COLOR_DANGER_TEXT,
            "❌",
        ),
        BannerKind::Info => (
            Color32::from_rgb(20, 35, 60),
            COLOR_ACCENT_CYAN,
            COLOR_DIAGONAL_TEXT,
            "ℹ️",
        ),
    };

    Frame::new()
        .fill(bg)
        .corner_radius(CornerRadius::same(6))
        .stroke(Stroke::new(1.0, border))
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.colored_label(fg, message);
            });
        });
}
