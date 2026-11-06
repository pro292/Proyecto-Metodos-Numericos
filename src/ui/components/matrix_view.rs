use egui::{Color32, ScrollArea, Ui, TextStyle};
use crate::matrix::matrix::Matrix;
use crate::ui::components::badges::{dimension_badge, dominance_badge};
use crate::ui::format_utils::format_decimal_short;
use crate::ui::theme::*;

pub struct MatrixViewOptions {
    pub show_vector_b: bool,
    pub max_display_dimension: usize,
}

impl Default for MatrixViewOptions {
    fn default() -> Self {
        Self {
            show_vector_b: true,
            max_display_dimension: 20,
        }
    }
}

pub fn render_matrix_view(
    ui: &mut Ui,
    matrix: &Matrix,
    vector_b: Option<&Matrix>,
    options: &MatrixViewOptions,
) {
    let rows = matrix.rows;
    let cols = matrix.columns;
    let is_square = rows == cols && rows > 0;
    let is_dominant = matrix.is_diagonally_dominant();

    // Encabezado de la vista de matriz con badges de estado
    ui.horizontal(|ui| {
        ui.heading("Matriz Activa");
        dimension_badge(ui, rows, cols);
        dominance_badge(ui, is_square, is_dominant);

        if let Some(b) = vector_b {
            if b.rows == rows && b.columns == 1 {
                crate::ui::components::badges::status_pill(
                    ui,
                    "Vector b cargado",
                    Color32::from_rgb(20, 45, 60),
                    COLOR_ACCENT_CYAN,
                    COLOR_DIAGONAL_TEXT,
                );
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("📋 Copiar al Portapapeles").clicked() {
                ui.ctx().copy_text(format!("{}", matrix));
            }
        });
    });

    ui.add_space(6.0);

    let display_rows = rows.min(options.max_display_dimension);
    let display_cols = cols.min(options.max_display_dimension);

    card_frame().show(ui, |ui| {
        ScrollArea::both()
            .max_height(280.0)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("matrix_grid_view")
                    .spacing([6.0, 4.0])
                    .striped(false)
                    .show(ui, |ui| {
                        // Fila de encabezado de columnas (j)
                        ui.label(""); // Espacio para el índice de fila
                        for j in 0..display_cols {
                            ui.colored_label(
                                COLOR_TEXT_MUTED,
                                egui::RichText::new(format!("c{}", j)).text_style(TextStyle::Monospace),
                            );
                        }
                        if options.show_vector_b && vector_b.is_some() {
                            ui.separator();
                            ui.colored_label(
                                COLOR_ACCENT_CYAN,
                                egui::RichText::new("b").strong().text_style(TextStyle::Monospace),
                            );
                        }
                        ui.end_row();

                        // Filas de datos
                        for i in 0..display_rows {
                            // Índice de fila (i)
                            ui.colored_label(
                                COLOR_TEXT_MUTED,
                                egui::RichText::new(format!("r{}", i)).text_style(TextStyle::Monospace),
                            );

                            for j in 0..display_cols {
                                let is_diagonal = is_square && (i == j);
                                let val = matrix.get(i, j);

                                matrix_cell_frame(is_diagonal).show(ui, |ui| {
                                    if let Some(num) = val {
                                        let short_str = format_decimal_short(num, 14);
                                        let text_color = if is_diagonal {
                                            COLOR_DIAGONAL_TEXT
                                        } else {
                                            COLOR_TEXT_PRIMARY
                                        };

                                        let label = ui.colored_label(
                                            text_color,
                                            egui::RichText::new(short_str).text_style(TextStyle::Monospace),
                                        );
                                        // Tooltip interactivo con el valor exacto completo
                                        label.on_hover_ui(|ui| {
                                            ui.label(format!("Posición ({}, {})", i, j));
                                            if is_diagonal {
                                                ui.colored_label(COLOR_ACCENT_CYAN, "Elemento Diagonal Principal");
                                            }
                                            ui.separator();
                                            ui.label(egui::RichText::new(num.to_string()).strong().text_style(TextStyle::Monospace));
                                        });
                                    } else {
                                        ui.colored_label(COLOR_DANGER_TEXT, "err");
                                    }
                                });
                            }

                            // Columna de vector b si existe
                            if options.show_vector_b {
                                if let Some(b) = vector_b {
                                    ui.separator();
                                    let b_val = b.get(i, 0);
                                    matrix_cell_frame(false).show(ui, |ui| {
                                        if let Some(num) = b_val {
                                            let short_str = format_decimal_short(num, 14);
                                            let label = ui.colored_label(
                                                COLOR_ACCENT_CYAN,
                                                egui::RichText::new(short_str).text_style(TextStyle::Monospace),
                                            );
                                            label.on_hover_ui(|ui| {
                                                ui.label(format!("Término independiente b[{}]", i));
                                                ui.separator();
                                                ui.label(egui::RichText::new(num.to_string()).strong().text_style(TextStyle::Monospace));
                                            });
                                        } else {
                                            ui.label("-");
                                        }
                                    });
                                }
                            }

                            ui.end_row();
                        }
                    });
            });

        if rows > display_rows || cols > display_cols {
            ui.add_space(4.0);
            ui.colored_label(
                COLOR_TEXT_MUTED,
                format!(
                    "ℹ️ Mostrando cuadrante {}×{} de la matriz total {}×{}.",
                    display_rows, display_cols, rows, cols
                ),
            );
        }
    });
}
