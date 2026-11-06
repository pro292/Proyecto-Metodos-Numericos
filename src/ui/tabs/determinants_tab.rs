use bigdecimal::BigDecimal;
use egui::{TextStyle, Ui};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crate::matrix::matrix::Matrix;
use crate::ui::components::badges::fastest_badge;
use crate::ui::components::banner::{show_banner, BannerKind};
use crate::ui::format_utils::format_duration;
use crate::ui::theme::*;

#[derive(Clone, Debug, Default)]
pub struct DeterminantRunResult {
    pub lu: Option<(Result<BigDecimal, &'static str>, Duration)>,
    pub leibniz: Option<(Result<BigDecimal, &'static str>, Duration)>,
    pub laplace: Option<(Result<BigDecimal, &'static str>, Duration)>,
}

pub struct DeterminantsTab {
    pub run_lu: bool,
    pub run_leibniz: bool,
    pub run_laplace: bool,

    // Estado asíncrono
    pub is_calculating: bool,
    pub start_time: Option<Instant>,
    pub receiver: Option<Receiver<DeterminantRunResult>>,
    pub last_result: Option<DeterminantRunResult>,
    pub error_banner: Option<String>,
}

impl Default for DeterminantsTab {
    fn default() -> Self {
        Self {
            run_lu: true,
            run_leibniz: true,
            run_laplace: true,
            is_calculating: false,
            start_time: None,
            receiver: None,
            last_result: None,
            error_banner: None,
        }
    }
}

impl DeterminantsTab {
    pub fn ui(&mut self, ui: &mut Ui, current_matrix: &Option<Matrix>) {
        // Polling del canal asíncrono si está calculando
        self.check_worker();

        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📐 Cálculo y Comparación de Determinantes");
            });
            ui.colored_label(
                COLOR_TEXT_SECONDARY,
                "Calcula el determinante usando los tres métodos provistos por el struct Matrix: Descomposición LU, Fórmula de Leibniz y Expansión de Laplace.",
            );

            ui.add_space(8.0);

            let matrix = match current_matrix {
                Some(m) => m,
                None => {
                    show_banner(
                        ui,
                        &BannerKind::Warning,
                        "No hay una matriz cargada. Dirígete a la pestaña de 'Generador' para crear una.",
                    );
                    return;
                }
            };

            let is_square = matrix.rows == matrix.columns && matrix.rows > 0;
            if !is_square {
                show_banner(
                    ui,
                    &BannerKind::Error,
                    &format!(
                        "La matriz actual no es cuadrada ({}×{}). Los determinantes solo están definidos para matrices cuadradas (R == C).",
                        matrix.rows, matrix.columns
                    ),
                );
                return;
            }

            // Advertencia de complejidad factorial para Laplace y Leibniz
            let n = matrix.rows;
            if n > 8 && (self.run_leibniz || self.run_laplace) {
                show_banner(
                    ui,
                    &BannerKind::Warning,
                    &format!(
                        "Matriz de {}×{}: Leibniz y Laplace poseen complejidad factorial O(n!). Para n = {}, se evaluarán hasta {} operaciones. Podría tardar bastante.",
                        n, n, n, factorial_approx_str(n)
                    ),
                );
                ui.add_space(4.0);
            }

            // Selección de métodos a ejecutar
            subcard_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Selecciona los métodos a ejecutar:").strong());
                    if ui.button("Seleccionar Todos").clicked() {
                        self.run_lu = true;
                        self.run_leibniz = true;
                        self.run_laplace = true;
                    }
                    if ui.button("Solo LU").clicked() {
                        self.run_lu = true;
                        self.run_leibniz = false;
                        self.run_laplace = false;
                    }
                    if ui.button("Deseleccionar Todos").clicked() {
                        self.run_lu = false;
                        self.run_leibniz = false;
                        self.run_laplace = false;
                    }
                });

                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.run_lu, "Factorización LU (O(n³))")
                        .on_hover_text("Eliminación gaussiana con pivoteo parcial. Eficiente para matrices de cualquier tamaño.");
                    ui.checkbox(&mut self.run_leibniz, "Fórmula de Leibniz (O(n!))")
                        .on_hover_text("Suma de todas las permutaciones signadas de la matriz.");
                    ui.checkbox(&mut self.run_laplace, "Expansión por Laplace (O(n!))")
                        .on_hover_text("Desarrollo recursivo por cofactores de la primera fila.");
                });
            });

            ui.add_space(10.0);

            // Botón de cálculo
            ui.horizontal(|ui| {
                let any_selected = self.run_lu || self.run_leibniz || self.run_laplace;
                let btn_text = if self.is_calculating {
                    "⏳ Calculando en segundo plano..."
                } else {
                    "🚀 Calcular Determinante(s)"
                };

                let btn = ui.add_enabled(
                    any_selected && !self.is_calculating,
                    egui::Button::new(egui::RichText::new(btn_text).strong()),
                );

                if btn.clicked() {
                    self.start_computation(matrix);
                }

                if self.is_calculating {
                    ui.spinner();
                    if let Some(st) = self.start_time {
                        let elapsed = st.elapsed();
                        ui.colored_label(COLOR_ACCENT_CYAN, format!("Tiempo transcurrido: {:.1} s", elapsed.as_secs_f64()));
                        ui.ctx().request_repaint(); // Redibuja suavemente el spinner
                    }
                }
            });

            if let Some(err) = &self.error_banner {
                ui.add_space(6.0);
                show_banner(ui, &BannerKind::Error, err);
            }

            // Resultados y comparaciones
            if let Some(results) = &self.last_result {
                ui.add_space(14.0);
                self.render_results_table(ui, results);
            }
        });
    }

    fn check_worker(&mut self) {
        if let Some(rx) = &self.receiver {
            if let Ok(res) = rx.try_recv() {
                self.last_result = Some(res);
                self.is_calculating = false;
                self.receiver = None;
                self.start_time = None;
            }
        }
    }

    fn start_computation(&mut self, matrix: &Matrix) {
        self.is_calculating = true;
        self.error_banner = None;
        self.start_time = Some(Instant::now());

        let mat = matrix.clone();
        let run_lu = self.run_lu;
        let run_leibniz = self.run_leibniz;
        let run_laplace = self.run_laplace;

        let (tx, rx) = channel();
        self.receiver = Some(rx);

        // Ejecución en hilo separado para evitar que la UI se congele
        thread::spawn(move || {
            let mut res = DeterminantRunResult::default();

            if run_lu {
                let t0 = Instant::now();
                let det = mat.lu_determinant();
                let dur = t0.elapsed();
                res.lu = Some((det, dur));
            }

            if run_leibniz {
                let t0 = Instant::now();
                let det = mat.leibniz_determinant();
                let dur = t0.elapsed();
                res.leibniz = Some((det, dur));
            }

            if run_laplace {
                let t0 = Instant::now();
                let det = mat.laplace_determinant();
                let dur = t0.elapsed();
                res.laplace = Some((det, dur));
            }

            let _ = tx.send(res);
        });
    }

    fn render_results_table(&self, ui: &mut Ui, results: &DeterminantRunResult) {
        ui.heading("📊 Resultados y Comparación");
        ui.add_space(6.0);

        // Encontrar el método más rápido entre los ejecutados con éxito
        let mut min_dur: Option<Duration> = None;
        let mut executed_count = 0;

        let methods_info = [
            ("Factorización LU", &results.lu),
            ("Fórmula de Leibniz", &results.leibniz),
            ("Expansión de Laplace", &results.laplace),
        ];

        for (_, res_opt) in &methods_info {
            if let Some((Ok(_), dur)) = res_opt {
                executed_count += 1;
                match min_dur {
                    None => min_dur = Some(*dur),
                    Some(cur) => {
                        if *dur < cur {
                            min_dur = Some(*dur);
                        }
                    }
                }
            }
        }

        // Tabla comparativa
        subcard_frame().show(ui, |ui| {
            egui::Grid::new("determinants_result_grid")
                .spacing([18.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Método").strong());
                    ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Estado").strong());
                    ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Determinante").strong());
                    ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Tiempo Transcurrido").strong());
                    ui.colored_label(COLOR_TEXT_MUTED, egui::RichText::new("Rendimiento").strong());
                    ui.end_row();

                    for (name, res_opt) in &methods_info {
                        if let Some((res, dur)) = res_opt {
                            ui.colored_label(COLOR_TEXT_PRIMARY, egui::RichText::new(*name).strong());

                            match res {
                                Ok(val) => {
                                    ui.colored_label(COLOR_SUCCESS_TEXT, "✓ Éxito");
                                    let det_str = val.to_string();
                                    let preview = if det_str.len() > 30 {
                                        format!("{}...", &det_str[..27])
                                    } else {
                                        det_str.clone()
                                    };

                                    let label = ui.colored_label(
                                        COLOR_ACCENT_CYAN,
                                        egui::RichText::new(preview).text_style(TextStyle::Monospace),
                                    );
                                    label.on_hover_ui(|ui| {
                                        ui.label("Valor exacto de determinante:");
                                        ui.separator();
                                        ui.label(egui::RichText::new(&det_str).text_style(TextStyle::Monospace));
                                    });

                                    ui.colored_label(
                                        COLOR_TEXT_PRIMARY,
                                        egui::RichText::new(format_duration(*dur)).text_style(TextStyle::Monospace),
                                    );

                                    // Indicador de velocidad
                                    if executed_count > 1 && Some(*dur) == min_dur {
                                        fastest_badge(ui);
                                    } else if let Some(best) = min_dur {
                                        if best.as_nanos() > 0 && dur.as_nanos() > 0 {
                                            let ratio = (dur.as_nanos() as f64) / (best.as_nanos() as f64);
                                            ui.colored_label(COLOR_TEXT_MUTED, format!("{:.1}x más lento", ratio));
                                        } else {
                                            ui.label("-");
                                        }
                                    } else {
                                        ui.label("-");
                                    }
                                }
                                Err(err_msg) => {
                                    ui.colored_label(COLOR_DANGER_TEXT, "❌ Error");
                                    ui.colored_label(COLOR_DANGER_TEXT, *err_msg);
                                    ui.colored_label(COLOR_TEXT_MUTED, format_duration(*dur));
                                    ui.label("-");
                                }
                            }
                            ui.end_row();
                        }
                    }
                });
        });

        // Verificación de consistencia entre los resultados
        let mut successful_dets: Vec<(&str, &BigDecimal)> = Vec::new();
        if let Some((Ok(v), _)) = &results.lu {
            successful_dets.push(("LU", v));
        }
        if let Some((Ok(v), _)) = &results.leibniz {
            successful_dets.push(("Leibniz", v));
        }
        if let Some((Ok(v), _)) = &results.laplace {
            successful_dets.push(("Laplace", v));
        }

        if successful_dets.len() >= 2 {
            ui.add_space(8.0);
            let first = successful_dets[0].1;
            let all_match = successful_dets.iter().all(|(_, val)| *val == first);

            if all_match {
                show_banner(
                    ui,
                    &BannerKind::Success,
                    "✅ Consistencia Verificada: Todos los métodos ejecutados obtuvieron exactamente el mismo determinante.",
                );
            } else {
                show_banner(
                    ui,
                    &BannerKind::Warning,
                    "⚠️ Variación detectada entre métodos (debido a redondeo de precisión en LU vs aritmética exacta entera/racional en Leibniz/Laplace).",
                );
            }
        }
    }
}

fn factorial_approx_str(n: usize) -> String {
    let mut f: u128 = 1;
    for i in 1..=n {
        f = f.saturating_mul(i as u128);
    }
    format!("{}", f)
}
