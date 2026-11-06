use egui::{DragValue, Slider, Ui};
use crate::matrix::matrix::Matrix;
use crate::ui::components::banner::{show_banner, BannerKind};
use crate::ui::theme::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatrixGenerationMode {
    General,
    DiagonallyDominant,
}

pub struct GeneratorTab {
    pub mode: MatrixGenerationMode,
    pub rows: usize,
    pub cols: usize,
    pub min_val: i64,
    pub max_val: i64,
    pub max_decimals: u8,
    pub sync_square: bool,

    // Vector b correspondiente
    pub auto_generate_b: bool,
    pub b_min_val: i64,
    pub b_max_val: i64,
    pub b_decimals: u8,

    pub last_message: Option<(BannerKind, String)>,
}

impl Default for GeneratorTab {
    fn default() -> Self {
        Self {
            mode: MatrixGenerationMode::DiagonallyDominant,
            rows: 4,
            cols: 4,
            min_val: -50,
            max_val: 50,
            max_decimals: 2,
            sync_square: true,

            auto_generate_b: true,
            b_min_val: -100,
            b_max_val: 100,
            b_decimals: 2,

            last_message: None,
        }
    }
}

impl GeneratorTab {
    pub fn ui(&mut self, ui: &mut Ui, current_matrix: &mut Option<Matrix>, current_b: &mut Option<Matrix>) {
        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚙️ Generador de Matrices");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Restablecer Valores").clicked() {
                        *self = Self::default();
                    }
                });
            });
            ui.colored_label(
                COLOR_TEXT_SECONDARY,
                "Configura los parámetros para instanciar matrices mediante los métodos implementados en el struct Matrix.",
            );

            ui.add_space(10.0);

            // Selector del método de generación
            ui.horizontal(|ui| {
                ui.label("Tipo de Matriz:");
                ui.radio_value(
                    &mut self.mode,
                    MatrixGenerationMode::General,
                    "Matriz Aleatoria General (matrix_generate)",
                );
                ui.radio_value(
                    &mut self.mode,
                    MatrixGenerationMode::DiagonallyDominant,
                    "Diagonalmente Dominante (diagonally_dominant_matrix_generate)",
                );
            });

            ui.add_space(10.0);

            // Advertencias y descripciones de cada método
            match self.mode {
                MatrixGenerationMode::General => {
                    subcard_frame().show(ui, |ui| {
                        ui.colored_label(COLOR_TEXT_PRIMARY, "ℹ️ Método Matrix::matrix_generate");
                        ui.colored_label(
                            COLOR_TEXT_MUTED,
                            "Genera una matriz de dimensiones R × C con valores decimales aleatorios en el intervalo [min, max).",
                        );
                        ui.colored_label(
                            COLOR_WARNING_TEXT,
                            "Condición interna del método: 'min' debe ser estrictamente menor que 'max'.",
                        );
                    });
                }
                MatrixGenerationMode::DiagonallyDominant => {
                    subcard_frame().show(ui, |ui| {
                        ui.colored_label(COLOR_ACCENT_CYAN, "ℹ️ Método Matrix::diagonally_dominant_matrix_generate");
                        ui.colored_label(
                            COLOR_TEXT_MUTED,
                            "Genera una matriz cuadrada estrictamente diagonalmente dominante (|a_ii| > ∑|a_ij|), requerida para Gauss-Seidel y Jacobi.",
                        );
                        ui.colored_label(
                            COLOR_WARNING_TEXT,
                            "Condición interna del método: Debe ser cuadrada (R == C), dimensiones mayores a 0 y 'min' < 'max'.",
                        );
                    });
                }
            }

            ui.add_space(12.0);

            // Controles de Parámetros
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.label(egui::RichText::new("Dimensiones de la Matriz").strong());
                    ui.horizontal(|ui| {
                        ui.label("Filas (R):");
                        let r_res = ui.add(DragValue::new(&mut self.rows).range(1..=usize::MAX).speed(1.0));
                        if r_res.changed() && self.sync_square && self.mode == MatrixGenerationMode::DiagonallyDominant {
                            self.cols = self.rows;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Columnas (C):");
                        let c_res = ui.add(DragValue::new(&mut self.cols).range(1..=usize::MAX).speed(1.0));
                        if c_res.changed() && self.sync_square && self.mode == MatrixGenerationMode::DiagonallyDominant {
                            self.rows = self.cols;
                        }
                    });

                    if self.mode == MatrixGenerationMode::DiagonallyDominant {
                        ui.checkbox(&mut self.sync_square, "Mantener cuadrada (R = C)");
                    }
                });

                columns[1].vertical(|ui| {
                    ui.label(egui::RichText::new("Rango Numérico y Precisión").strong());
                    ui.horizontal(|ui| {
                        ui.label("Mínimo (min):");
                        ui.add(DragValue::new(&mut self.min_val).range(-10_000..=10_000).speed(1));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Máximo (max):");
                        ui.add(DragValue::new(&mut self.max_val).range(-10_000..=10_000).speed(1));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Máximo de Decimales:");
                        ui.add(Slider::new(&mut self.max_decimals, 0..=10));
                    });
                });
            });

            ui.add_space(12.0);

            // Sección del Vector b de Términos Independientes
            subcard_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(
                        &mut self.auto_generate_b,
                        "Generar simultáneamente Vector b de términos independientes (R × 1)",
                    );
                });

                if self.auto_generate_b {
                    ui.horizontal(|ui| {
                        ui.label("Rango de b: [");
                        ui.add(DragValue::new(&mut self.b_min_val).speed(1));
                        ui.label(",");
                        ui.add(DragValue::new(&mut self.b_max_val).speed(1));
                        ui.label("]");
                        ui.label("Decimales:");
                        ui.add(Slider::new(&mut self.b_decimals, 0..=8));
                    });
                }
            });

            ui.add_space(10.0);

            // Validaciones previas visuales
            if self.min_val >= self.max_val {
                show_banner(
                    ui,
                    &BannerKind::Warning,
                    "Aviso de parámetro: 'min' debe ser menor que 'max'. Si continúas, el método retornará su error controlado.",
                );
                ui.add_space(4.0);
            }

            if self.mode == MatrixGenerationMode::DiagonallyDominant && self.rows != self.cols {
                show_banner(
                    ui,
                    &BannerKind::Warning,
                    "Aviso de parámetro: Para matriz diagonalmente dominante, filas y columnas deben coincidir.",
                );
                ui.add_space(4.0);
            }

            // Botón Principal de Generación
            ui.horizontal(|ui| {
                let btn_text = match self.mode {
                    MatrixGenerationMode::General => "🎲 Generar Matriz Aleatoria (matrix_generate)",
                    MatrixGenerationMode::DiagonallyDominant => "✨ Generar Matriz Diagonalmente Dominante",
                };

                let generate_btn = ui.add_sized([280.0, 36.0], egui::Button::new(egui::RichText::new(btn_text).strong()));

                if generate_btn.clicked() {
                    self.execute_generation(current_matrix, current_b);
                }

                if current_matrix.is_some() {
                    if ui.button("🔄 Regenerar solo Vector b").clicked() {
                        self.regenerate_vector_b(current_matrix, current_b);
                    }
                }
            });

            ui.add_space(10.0);

            // Botones de prueba rápida (Presets para verificar robustez y manejo de errores)
            ui.horizontal(|ui| {
                ui.colored_label(COLOR_TEXT_MUTED, "Casos rápidos de prueba:");
                if ui.button("3×3 Dominante").clicked() {
                    self.mode = MatrixGenerationMode::DiagonallyDominant;
                    self.rows = 3;
                    self.cols = 3;
                    self.min_val = -50;
                    self.max_val = 50;
                    self.max_decimals = 2;
                    self.execute_generation(current_matrix, current_b);
                }
                if ui.button("5×5 General").clicked() {
                    self.mode = MatrixGenerationMode::General;
                    self.rows = 5;
                    self.cols = 5;
                    self.min_val = -100;
                    self.max_val = 100;
                    self.max_decimals = 4;
                    self.execute_generation(current_matrix, current_b);
                }
                if ui.button("⚠️ Probar Error: min >= max").clicked() {
                    self.min_val = 100;
                    self.max_val = 50;
                    self.execute_generation(current_matrix, current_b);
                }
                if ui.button("⚠️ Probar Error: R != C en Dominante").clicked() {
                    self.mode = MatrixGenerationMode::DiagonallyDominant;
                    self.sync_square = false;
                    self.rows = 4;
                    self.cols = 3;
                    self.execute_generation(current_matrix, current_b);
                }
            });

            // Mensajes de resultado o error
            if let Some((kind, msg)) = &self.last_message {
                ui.add_space(8.0);
                show_banner(ui, kind, msg);
            }
        });
    }

    fn execute_generation(&mut self, current_matrix: &mut Option<Matrix>, current_b: &mut Option<Matrix>) {
        let result = match self.mode {
            MatrixGenerationMode::General => {
                Matrix::matrix_generate(self.rows, self.cols, self.min_val, self.max_val, self.max_decimals)
            }
            MatrixGenerationMode::DiagonallyDominant => {
                Matrix::diagonally_dominant_matrix_generate(self.rows, self.cols, self.min_val, self.max_val, self.max_decimals)
            }
        };

        match result {
            Ok(mat) => {
                let r = mat.rows;
                let c = mat.columns;
                *current_matrix = Some(mat);

                if self.auto_generate_b && r > 0 {
                    match Matrix::matrix_generate(r, 1, self.b_min_val, self.b_max_val, self.b_decimals) {
                        Ok(b) => {
                            *current_b = Some(b);
                            self.last_message = Some((
                                BannerKind::Success,
                                format!("Matriz de {}×{} y vector b ({}×1) generados correctamente.", r, c, r),
                            ));
                        }
                        Err(b_err) => {
                            *current_b = None;
                            self.last_message = Some((
                                BannerKind::Warning,
                                format!("Matriz {}×{} generada, pero falló la generación de b: {}", r, c, b_err),
                            ));
                        }
                    }
                } else {
                    *current_b = None;
                    self.last_message = Some((
                        BannerKind::Success,
                        format!("Matriz de {}×{} generada correctamente.", r, c),
                    ));
                }
            }
            Err(err_msg) => {
                // Gestión de errores: Mostramos el mensaje exacto retornado por el struct sin cerrar ni crashear el programa
                self.last_message = Some((
                    BannerKind::Error,
                    format!("Error devuelto por el método del struct: \"{}\"", err_msg),
                ));
            }
        }
    }

    fn regenerate_vector_b(&mut self, current_matrix: &Option<Matrix>, current_b: &mut Option<Matrix>) {
        if let Some(mat) = current_matrix {
            let r = mat.rows;
            match Matrix::matrix_generate(r, 1, self.b_min_val, self.b_max_val, self.b_decimals) {
                Ok(b) => {
                    *current_b = Some(b);
                    self.last_message = Some((
                        BannerKind::Success,
                        format!("Vector b ({}×1) regenerado exitosamente.", r),
                    ));
                }
                Err(err) => {
                    self.last_message = Some((
                        BannerKind::Error,
                        format!("No se pudo generar el vector b: {}", err),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_matrix_generation() {
        let mut tab = GeneratorTab::default();
        tab.rows = 150;
        tab.cols = 150;
        tab.mode = MatrixGenerationMode::DiagonallyDominant;

        let mut mat = None;
        let mut b = None;
        tab.execute_generation(&mut mat, &mut b);

        assert!(mat.is_some());
        let m = mat.unwrap();
        assert_eq!(m.rows, 150);
        assert_eq!(m.columns, 150);
        assert!(m.is_diagonally_dominant());
        assert!(b.is_some());
        assert_eq!(b.unwrap().rows, 150);
    }
}

