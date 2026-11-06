use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Visuals};

pub const COLOR_BG_DARK: Color32 = Color32::from_rgb(18, 21, 28);
pub const COLOR_PANEL_BG: Color32 = Color32::from_rgb(24, 28, 38);
pub const COLOR_CARD_BG: Color32 = Color32::from_rgb(31, 37, 51);
pub const COLOR_CARD_HOVER: Color32 = Color32::from_rgb(38, 45, 62);
pub const COLOR_CARD_BORDER: Color32 = Color32::from_rgb(49, 58, 80);

pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(99, 102, 241); // Indigo
pub const COLOR_PRIMARY_HOVER: Color32 = Color32::from_rgb(129, 140, 248);
pub const COLOR_PRIMARY_ACTIVE: Color32 = Color32::from_rgb(79, 70, 229);

pub const COLOR_ACCENT_CYAN: Color32 = Color32::from_rgb(56, 189, 248); // Sky blue
pub const COLOR_DIAGONAL_BG: Color32 = Color32::from_rgb(30, 58, 95);
pub const COLOR_DIAGONAL_BORDER: Color32 = Color32::from_rgb(56, 189, 248);
pub const COLOR_DIAGONAL_TEXT: Color32 = Color32::from_rgb(186, 230, 253);

pub const COLOR_SUCCESS_BG: Color32 = Color32::from_rgb(19, 53, 37);
pub const COLOR_SUCCESS_BORDER: Color32 = Color32::from_rgb(34, 197, 94);
pub const COLOR_SUCCESS_TEXT: Color32 = Color32::from_rgb(74, 222, 128);

pub const COLOR_WARNING_BG: Color32 = Color32::from_rgb(66, 42, 15);
pub const COLOR_WARNING_BORDER: Color32 = Color32::from_rgb(245, 158, 11);
pub const COLOR_WARNING_TEXT: Color32 = Color32::from_rgb(251, 191, 36);

pub const COLOR_DANGER_BG: Color32 = Color32::from_rgb(69, 26, 26);
pub const COLOR_DANGER_BORDER: Color32 = Color32::from_rgb(239, 68, 68);
pub const COLOR_DANGER_TEXT: Color32 = Color32::from_rgb(248, 113, 113);

pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(241, 245, 249);
pub const COLOR_TEXT_SECONDARY: Color32 = Color32::from_rgb(148, 163, 184);
pub const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(100, 116, 139);

pub fn setup_custom_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(COLOR_TEXT_PRIMARY);
    visuals.panel_fill = COLOR_PANEL_BG;
    visuals.window_fill = COLOR_CARD_BG;
    visuals.window_stroke = Stroke::new(1.0, COLOR_CARD_BORDER);
    visuals.window_corner_radius = CornerRadius::same(10);
    
    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = COLOR_CARD_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, COLOR_CARD_BORDER);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, COLOR_TEXT_PRIMARY);

    // Inactive widgets
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(33, 39, 54);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, COLOR_CARD_BORDER);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, COLOR_TEXT_PRIMARY);

    // Hovered widgets
    visuals.widgets.hovered.bg_fill = COLOR_CARD_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, COLOR_PRIMARY);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);

    // Active widgets
    visuals.widgets.active.bg_fill = COLOR_PRIMARY_ACTIVE;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, COLOR_PRIMARY_HOVER);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);

    // Selection
    visuals.selection.bg_fill = COLOR_PRIMARY;
    visuals.selection.stroke = Stroke::new(1.0, COLOR_PRIMARY_HOVER);

    ctx.set_visuals(visuals);
}

pub fn card_frame() -> Frame {
    Frame::new()
        .fill(COLOR_CARD_BG)
        .corner_radius(CornerRadius::same(8))
        .stroke(Stroke::new(1.0, COLOR_CARD_BORDER))
        .inner_margin(Margin::same(12))
}

pub fn subcard_frame() -> Frame {
    Frame::new()
        .fill(Color32::from_rgb(24, 28, 40))
        .corner_radius(CornerRadius::same(6))
        .stroke(Stroke::new(1.0, Color32::from_rgb(42, 50, 70)))
        .inner_margin(Margin::same(10))
}

pub fn matrix_cell_frame(is_diagonal: bool) -> Frame {
    if is_diagonal {
        Frame::new()
            .fill(COLOR_DIAGONAL_BG)
            .corner_radius(CornerRadius::same(4))
            .stroke(Stroke::new(1.5, COLOR_DIAGONAL_BORDER))
            .inner_margin(Margin::symmetric(8, 6))
    } else {
        Frame::new()
            .fill(Color32::from_rgb(26, 32, 45))
            .corner_radius(CornerRadius::same(4))
            .stroke(Stroke::new(1.0, Color32::from_rgb(40, 48, 68)))
            .inner_margin(Margin::symmetric(8, 6))
    }
}
