use bigdecimal::BigDecimal;
use egui::{DragValue, ScrollArea, TextStyle, Ui};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crate::matrix::matrix::Matrix;
use crate::matrix::sistema_lineal::SistemaLineal;
use crate::ui::components::badges::fastest_badge;
use crate::ui::components::banner::{show_banner, BannerKind};
use crate::ui::format_utils::{format_decimal_short, format_duration, parse_tolerance_string};
use crate::ui::theme::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SolverSelection {
    GaussSeidel,
    Jacobi,
    Both,
}

#[derive(Clone, Debug, Default)]
pub struct LinearSystemResults {
    pub gauss_seidel: Option<(Result<Matrix, &'static str>, Duration)>,
    pub jacobi: Option<(Result<Matrix, &'static str>, Duration)>,
}

pub struct LinearSystemsTab {
    pub solver_selection: SolverSelection,
    pub tolerance_str: String,
    pub max_iterations: usize,

    // Estado asíncrono
    pub is_solving: bool,
    pub start_time: Option<Instant>,
    pub receiver: Option<Receiver<LinearSystemResults>>,
    pub last_result: Option<LinearSystemResults>,
    pub error_banner: Option<String>,
}

impl Default for LinearSystemsTab {
    fn default() -> Self {
        Self {
            solver_selection: SolverSelection::Both,
            tolerance_str: "1e-5".to_string(),
            max_iterations: 100,
            is_solving: false,
            start_time: None,
            receiver: None,
            last_result: None,
            error_banner: None,
        }
    }
}

impl LinearSystemsTab {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        current_matrix: &Option<Matrix>,
        current_b: &mut Option<Matrix>,
    ) {
        self.check_worker();

        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ Resolución de Sistemas Lineales (Ax = b)");
            });
            ui.colored_label(
                COLOR_TEXT_SECONDARY,
                "Resuelve el sistema mediante los métodos iterativos de Gauss-Seidel y Jacobi provistos por SistemaLineal, con medición de tiempo y comparador.",
            );

            ui.add_space(8.0);

            let matrix = match current_matrix {
                Some(m) => m,
                None => {
                    show_banner(
                        ui,
                        &BannerKind::Warning,
                        "No hay una matriz de coeficientes cargada. Dirígete a la pestaña de 'Generador' para crear una.",
                    );
                    return;
                }
            };

            let rows = matrix.rows;
            let cols = matrix.columns;

            // Verificación de matriz cuadrada
            if rows != cols || rows == 0 {
                show_banner(
                    ui,
                    &BannerKind::Error,
                    &format!(
                        "La matriz de coeficientes debe ser cuadrada para un sistema lineal determinado. Tamaño actual: {}×{}.",
                        rows, cols
                    ),
                );
                return;
            }

            // Verificación de vector independiente b
            let b_is_valid = match current_b {
                Some(b) => b.rows == rows && b.columns == 1,
                None => false,
            };

            if !b_is_valid {
                subcard_frame().show(ui, |ui| {
                    ui.colored_label(
                        COLOR_WARNING_TEXT,
                        format!("Se requiere un vector de términos independientes b de dimensión ({}×1).", rows),
                    );
                    if ui.button("⚡ Generar Vector b Automático").clicked() {
                        if let Ok(new_b) = Matrix::matrix_generate(rows, 1, -100, 100, 2) {
                            *current_b = Some(new_b);
                        }
                    }
                });
                ui.add_space(6.0);
            }

            // Comprobación de Dominancia Diagonal
            let is_dominant = matrix.is_diagonally_dominant();
            if !is_dominant {
                show_banner(
                    ui,
                    &BannerKind::Warning,
                    "⚠️ La matriz no cumple la condición de dominancia diagonal estricta (|a_ii| > ∑_{j≠i} |a_ij|). Los métodos de Gauss-Seidel y Jacobi requieren esta condición para garantizar la convergencia matemática.",
                );
                ui.add_space(4.0);
            }

            // Configuración de Parámetros del Solver
            subcard_frame().show(ui, |ui| {
                ui.columns(2, |cols| {
                    cols[0].vertical(|ui| {
                        ui.label(egui::RichText::new("Método de Resolución").strong());
                        ui.radio_value(&mut self.solver_selection, SolverSelection::Both, "Ambos Métodos (Comparar Gauss-Seidel y Jacobi)");
                        ui.radio_value(&mut self.solver_selection, SolverSelection::GaussSeidel, "Solo Gauss-Seidel");
                        ui.radio_value(&mut self.solver_selection, SolverSelection::Jacobi, "Solo Jacobi");
                    });

                    cols[1].vertical(|ui| {
                        ui.label(egui::RichText::new("Criterio de Parada e Iteraciones").strong());
                        ui.horizontal(|ui| {
                            ui.label("Tolerancia (ε):");
                            ui.text_edit_singleline(&mut self.tolerance_str);
                        });

                        ui.horizontal(|ui| {
                            ui.colored_label(COLOR_TEXT_MUTED, "Presets:");
                            if ui.button("1e-4").clicked() { self.tolerance_str = "1e-4".to_string(); }
                            if ui.button("1e-5").clicked() { self.tolerance_str = "1e-5".to_string(); }
                            if ui.button("1e-8").clicked() { self.tolerance_str = "1e-8".to_string(); }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Máximo de Iteraciones:");
                            ui.add(DragValue::new(&mut self.max_iterations).range(1..=100_000).speed(10));
                        });
                    });
                });
            });

            ui.add_space(10.0);

            // Botón de Resolución
            ui.horizontal(|ui| {
                let btn_text = if self.is_solving {
                    "⏳ Resolviendo en segundo plano..."
                } else {
                    match self.solver_selection {
                        SolverSelection::Both => "⚡ Resolver con Ambos Métodos y Comparar",
                        SolverSelection::GaussSeidel => "⚡ Resolver con Gauss-Seidel",
                        SolverSelection::Jacobi => "⚡ Resolver con Jacobi",
                    }
                };

                let can_solve = current_b.is_some() && !self.is_solving;
                let btn = ui.add_enabled(
                    can_solve,
                    egui::Button::new(egui::RichText::new(btn_text).strong()),
                );

                if btn.clicked() {
                    if let Some(b) = current_b {
                        self.start_solving(matrix, b);
                    }
                }

                if self.is_solving {
                    ui.spinner();
                    if let Some(st) = self.start_time {
                        let elapsed = st.elapsed();
                        ui.colored_label(COLOR_ACCENT_CYAN, format!("Tiempo transcurrido: {:.2} s", elapsed.as_secs_f64()));
                        ui.ctx().request_repaint();
                    }
                }
            });

            if let Some(err) = &self.error_banner {
                ui.add_space(6.0);
                show_banner(ui, &BannerKind::Error, err);
            }

            // Despliegue de Resultados y Comparaciones
            if let Some(results) = &self.last_result {
                ui.add_space(14.0);
                self.render_solution_results(ui, results);
            }
        });
    }

    fn check_worker(&mut self) {
        if let Some(rx) = &self.receiver {
            if let Ok(res) = rx.try_recv() {
                self.last_result = Some(res);
                self.is_solving = false;
                self.receiver = None;
                self.start_time = None;
            }
        }
    }

    fn start_solving(&mut self, matrix: &Matrix, vector_b: &Matrix) {
        let tolerance = match parse_tolerance_string(&self.tolerance_str) {
            Ok(t) => t,
            Err(e) => {
                self.error_banner = Some(e.to_string());
                return;
            }
        };

        self.error_banner = None;
        self.is_solving = true;
        self.start_time = Some(Instant::now());

        let mat_coef = matrix.clone();
        let mat_indep = vector_b.clone();
        let selection = self.solver_selection;
        let max_iter = self.max_iterations;

        let (tx, rx) = channel();
        self.receiver = Some(rx);

        thread::spawn(move || {
            let mut res = LinearSystemResults::default();
            let sistema = SistemaLineal::new(mat_coef, mat_indep);

            if selection == SolverSelection::GaussSeidel || selection == SolverSelection::Both {
                let t0 = Instant::now();
                let gs_res = sistema.gauss_seidel(tolerance.clone(), max_iter);
                let dur = t0.elapsed();
                res.gauss_seidel = Some((gs_res, dur));
            }

            if selection == SolverSelection::Jacobi || selection == SolverSelection::Both {
                let t0 = Instant::now();
                let jacobi_res = sistema.jacobi(tolerance, max_iter);
                let dur = t0.elapsed();
                res.jacobi = Some((jacobi_res, dur));
            }

            let _ = tx.send(res);
        });
    }

    fn render_solution_results(&self, ui: &mut Ui, results: &LinearSystemResults) {
        ui.heading("📊 Resultados de la Resolución");
        ui.add_space(8.0);

        // Tarjetas resumen de cada método
        ui.columns(2, |cols| {
            if let Some((res, dur)) = &results.gauss_seidel {
                cols[0].vertical(|ui| {
                    subcard_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(COLOR_TEXT_PRIMARY, egui::RichText::new("Gauss-Seidel").strong());
                            match res {
                                Ok(_) => ui.colored_label(COLOR_SUCCESS_TEXT, "✓ Convergió"),
                                Err(_) => ui.colored_label(COLOR_DANGER_TEXT, "❌ No Convergió"),
                            };
                        });
                        ui.horizontal(|ui| {
                            ui.label("Tiempo:");
                            ui.colored_label(COLOR_ACCENT_CYAN, egui::RichText::new(format_duration(*dur)).text_style(TextStyle::Monospace));
                        });
                        if let Err(err_msg) = res {
                            ui.colored_label(COLOR_DANGER_TEXT, format!("Motivo: {}", err_msg));
                        }
                    });
                });
            }

            if let Some((res, dur)) = &results.jacobi {
                cols[1].vertical(|ui| {
                    subcard_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(COLOR_TEXT_PRIMARY, egui::RichText::new("Jacobi").strong());
                            match res {
                                Ok(_) => ui.colored_label(COLOR_SUCCESS_TEXT, "✓ Convergió"),
                                Err(_) => ui.colored_label(COLOR_DANGER_TEXT, "❌ No Convergió"),
                            };
                        });
                        ui.horizontal(|ui| {
                            ui.label("Tiempo:");
                            ui.colored_label(COLOR_ACCENT_CYAN, egui::RichText::new(format_duration(*dur)).text_style(TextStyle::Monospace));
                        });
                        if let Err(err_msg) = res {
                            ui.colored_label(COLOR_DANGER_TEXT, format!("Motivo: {}", err_msg));
                        }
                    });
                });
            }
        });

        // Comparativa de tiempos si ambos corrieron con éxito
        if let (Some((Ok(_), dur_gs)), Some((Ok(_), dur_j))) = (&results.gauss_seidel, &results.jacobi) {
            ui.add_space(6.0);
            subcard_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Comparación de Rendimiento:");
                    if *dur_gs < *dur_j {
                        fastest_badge(ui);
                        let ratio = (dur_j.as_nanos() as f64) / (dur_gs.as_nanos().max(1) as f64);
                        ui.colored_label(COLOR_SUCCESS_TEXT, format!("Gauss-Seidel fue {:.1}x más rápido que Jacobi.", ratio));
                    } else if *dur_j < *dur_gs {
                        fastest_badge(ui);
                        let ratio = (dur_gs.as_nanos() as f64) / (dur_j.as_nanos().max(1) as f64);
                        ui.colored_label(COLOR_SUCCESS_TEXT, format!("Jacobi fue {:.1}x más rápido que Gauss-Seidel.", ratio));
                    } else {
                        ui.label("Ambos métodos tomaron tiempos prácticamente idénticos.");
                    }
                });
            });
        }

        ui.add_space(10.0);

        // Tabla de soluciones del vector X
        let gs_matrix = results.gauss_seidel.as_ref().and_then(|(r, _)| r.as_ref().ok());
        let j_matrix = results.jacobi.as_ref().and_then(|(r, _)| r.as_ref().ok());

        if gs_matrix.is_some() || j_matrix.is_some() {
            let n_vars = gs_matrix.map(|m| m.rows).or_else(|| j_matrix.map(|m| m.rows)).unwrap_or(0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Vector Solución X").strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📋 Copiar Soluciones").clicked() {
                        let mut text = String::new();
                        for i in 0..n_vars {
                            let gs_val = gs_matrix.and_then(|m| m.get(i, 0)).map(|v| v.to_string()).unwrap_or_else(|| "-".to_string());
                            let j_val = j_matrix.and_then(|m| m.get(i, 0)).map(|v| v.to_string()).unwrap_or_else(|| "-".to_string());
                            text.push_str(&format!("x[{}] => GS: {}, Jacobi: {}\n", i, gs_val, j_val));
                        }
                        ui.ctx().copy_text(text);
                    }
                });
            });

            ui.add_space(4.0);

            card_frame().show(ui, |ui| {
                ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    egui::Grid::new("solutions_grid")
                        .spacing([18.0, 6.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Incógnita").strong());
                            if gs_matrix.is_some() {
                                ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("x (Gauss-Seidel)").strong());
                            }
                            if j_matrix.is_some() {
                                ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("x (Jacobi)").strong());
                            }
                            if gs_matrix.is_some() && j_matrix.is_some() {
                                ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("|x_GS - x_J| (Diferencia)").strong());
                            }
                            ui.end_row();

                            let mut max_diff = BigDecimal::from(0);

                            for i in 0..n_vars {
                                ui.colored_label(COLOR_TEXT_MUTED, format!("x_{}", i));

                                let gs_val = gs_matrix.and_then(|m| m.get(i, 0));
                                let j_val = j_matrix.and_then(|m| m.get(i, 0));

                                if gs_matrix.is_some() {
                                    if let Some(v) = gs_val {
                                        let label = ui.colored_label(
                                            COLOR_TEXT_PRIMARY,
                                            egui::RichText::new(format_decimal_short(v, 18)).text_style(TextStyle::Monospace),
                                        );
                                        label.on_hover_ui(|ui| {
                                            ui.label(format!("x_{} Gauss-Seidel:", i));
                                            ui.separator();
                                            ui.label(egui::RichText::new(v.to_string()).text_style(TextStyle::Monospace));
                                        });
                                    } else {
                                        ui.label("-");
                                    }
                                }

                                if j_matrix.is_some() {
                                    if let Some(v) = j_val {
                                        let label = ui.colored_label(
                                            COLOR_ACCENT_CYAN,
                                            egui::RichText::new(format_decimal_short(v, 18)).text_style(TextStyle::Monospace),
                                        );
                                        label.on_hover_ui(|ui| {
                                            ui.label(format!("x_{} Jacobi:", i));
                                            ui.separator();
                                            ui.label(egui::RichText::new(v.to_string()).text_style(TextStyle::Monospace));
                                        });
                                    } else {
                                        ui.label("-");
                                    }
                                }

                                if let (Some(v1), Some(v2)) = (gs_val, j_val) {
                                    let diff = (v1.clone() - v2).abs();
                                    if diff > max_diff {
                                        max_diff = diff.clone();
                                    }
                                    let label = ui.colored_label(
                                        COLOR_DIAGONAL_TEXT,
                                        egui::RichText::new(format_decimal_short(&diff, 16)).text_style(TextStyle::Monospace),
                                    );
                                    label.on_hover_ui(|ui| {
                                        ui.label("Diferencia exacta:");
                                        ui.separator();
                                        ui.label(egui::RichText::new(diff.to_string()).text_style(TextStyle::Monospace));
                                    });
                                }

                                ui.end_row();
                            }
                        });
                });
            });
        }
    }
}
