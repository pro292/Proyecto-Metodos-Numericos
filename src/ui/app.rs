use egui::{CentralPanel, Color32, Frame, Margin, Panel, ScrollArea, Ui};
use crate::matrix::matrix::Matrix;
use crate::ui::components::badges::{dimension_badge, dominance_badge};
use crate::ui::components::matrix_view::{render_matrix_view, MatrixViewOptions};
use crate::ui::tabs::determinants_tab::DeterminantsTab;
use crate::ui::tabs::generator_tab::GeneratorTab;
use crate::ui::tabs::linear_systems_tab::LinearSystemsTab;
use crate::ui::theme::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveTab {
    Generator,
    Determinants,
    LinearSystems,
}

pub struct NumericalMethodsApp {
    pub current_matrix: Option<Matrix>,
    pub current_b: Option<Matrix>,
    pub active_tab: ActiveTab,
    pub show_matrix_preview: bool,

    // Submódulos de cada pestaña
    pub generator_tab: GeneratorTab,
    pub determinants_tab: DeterminantsTab,
    pub linear_systems_tab: LinearSystemsTab,
    pub matrix_options: MatrixViewOptions,
}

impl NumericalMethodsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_theme(&cc.egui_ctx);

        // Inicializamos con una matriz diagonalmente dominante 4×4 por defecto para mayor comodidad del usuario
        let initial_matrix = Matrix::diagonally_dominant_matrix_generate(4, 4, -50, 50, 2).ok();
        let initial_b = Matrix::matrix_generate(4, 1, -100, 100, 2).ok();

        Self {
            current_matrix: initial_matrix,
            current_b: initial_b,
            active_tab: ActiveTab::Generator,
            show_matrix_preview: true,

            generator_tab: GeneratorTab::default(),
            determinants_tab: DeterminantsTab::default(),
            linear_systems_tab: LinearSystemsTab::default(),
            matrix_options: MatrixViewOptions::default(),
        }
    }
}

impl eframe::App for NumericalMethodsApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Barra Superior (Header & Navegación)
        Panel::top("top_header_panel")
            .frame(Frame::new().fill(COLOR_BG_DARK).inner_margin(Margin::symmetric(16, 12)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔢 Métodos Numéricos").size(18.0).strong().color(Color32::WHITE));
                    ui.colored_label(COLOR_TEXT_MUTED, "|");
                    ui.colored_label(COLOR_ACCENT_CYAN, "Matrices & Sistemas Lineales");

                    if let Some(mat) = &self.current_matrix {
                        ui.separator();
                        dimension_badge(ui, mat.rows, mat.columns);
                        let is_sq = mat.rows == mat.columns && mat.rows > 0;
                        dominance_badge(ui, is_sq, mat.is_diagonally_dominant());
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.checkbox(&mut self.show_matrix_preview, "Mostrar Vista Previa");
                    });
                });

                ui.add_space(8.0);

                // Selector de Pestañas con botones tipo tab moderno
                ui.horizontal(|ui| {
                    let tabs = [
                        (ActiveTab::Generator, "🎲 1. Generador de Matrices"),
                        (ActiveTab::Determinants, "📐 2. Determinantes (LU / Leibniz / Laplace)"),
                        (ActiveTab::LinearSystems, "⚡ 3. Sistemas Lineales (Gauss-Seidel / Jacobi)"),
                    ];

                    for (tab, label) in tabs {
                        let is_active = self.active_tab == tab;
                        let text = egui::RichText::new(label).strong();
                        let btn = if is_active {
                            egui::Button::new(text.color(Color32::WHITE)).fill(COLOR_PRIMARY)
                        } else {
                            egui::Button::new(text.color(COLOR_TEXT_SECONDARY)).fill(COLOR_CARD_BG)
                        };

                        if ui.add_sized([230.0, 32.0], btn).clicked() {
                            self.active_tab = tab;
                        }
                    }
                });
            });

        // Barra Inferior (Status Bar)
        Panel::bottom("bottom_status_panel")
            .frame(Frame::new().fill(COLOR_BG_DARK).inner_margin(Margin::symmetric(16, 6)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Some(mat) = &self.current_matrix {
                        ui.colored_label(COLOR_TEXT_MUTED, format!("Matriz activa: {} filas × {} columnas", mat.rows, mat.columns));
                        if self.current_b.is_some() {
                            ui.colored_label(COLOR_ACCENT_CYAN, "• Vector b cargado");
                        }
                    } else {
                        ui.colored_label(COLOR_TEXT_MUTED, "Sin matriz cargada");
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(COLOR_TEXT_MUTED, "Proyecto Métodos Numéricos — egui");
                    });
                });
            });

        // Panel Central
        CentralPanel::default()
            .frame(Frame::new().fill(COLOR_PANEL_BG).inner_margin(Margin::same(14)))
            .show(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    // Vista previa de la matriz activa (si está habilitada y existe matriz)
                    if self.show_matrix_preview {
                        if let Some(mat) = &self.current_matrix {
                            render_matrix_view(
                                ui,
                                mat,
                                self.current_b.as_ref(),
                                &self.matrix_options,
                            );
                            ui.add_space(12.0);
                        }
                    }

                    // Renderizado del contenido de la pestaña activa
                    match self.active_tab {
                        ActiveTab::Generator => {
                            self.generator_tab.ui(
                                ui,
                                &mut self.current_matrix,
                                &mut self.current_b,
                            );
                        }
                        ActiveTab::Determinants => {
                            self.determinants_tab.ui(ui, &self.current_matrix);
                        }
                        ActiveTab::LinearSystems => {
                            self.linear_systems_tab.ui(
                                ui,
                                &self.current_matrix,
                                &mut self.current_b,
                            );
                        }
                    }

                    ui.add_space(20.0);
                });
            });
    }
}
