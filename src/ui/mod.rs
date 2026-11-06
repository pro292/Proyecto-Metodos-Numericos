pub mod app;
pub mod components;
pub mod format_utils;
pub mod tabs;
pub mod theme;

use app::NumericalMethodsApp;

pub fn run_gui() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Métodos Numéricos — Matrices & Sistemas Lineales")
            .with_inner_size([1120.0, 780.0])
            .with_min_inner_size([860.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Métodos Numéricos — Matrices & Sistemas Lineales",
        native_options,
        Box::new(|cc| Ok(Box::new(NumericalMethodsApp::new(cc)))),
    )
}

