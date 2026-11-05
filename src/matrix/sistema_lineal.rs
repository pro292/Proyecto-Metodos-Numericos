use crate::matrix::matrix::Matrix;
use bigdecimal::{BigDecimal, Zero, Signed};

#[derive(Debug, Clone)]
pub struct SistemaLineal {
    coefficient: Matrix,
    independent: Matrix,
}


impl SistemaLineal {

    // constructor
    pub fn new(coef: Matrix, indep: Matrix) -> Self {
        Self {coefficient: coef, independent: indep}
    }

    // resuleve el sistema mediante el metodo de gauss-seidel
    pub fn Gauss_Seidel(&self, tolerance: BigDecimal, max_iterations: usize) -> Result<Matrix, &'static str> {
        if !self.coefficient.is_diagonally_dominant() {
            return Err("La matriz no cumple la condicion de dominancia diagonal estricta");
        }
        if self.independent.rows != self.coefficient.rows || self.independent.columns != 1 {
            return Err("El vector independiente tiene dimensiones incompatibles");
        }
        if tolerance <= BigDecimal::zero() {
            return Err("La tolerancia debe ser mayor que cero");
        }
        if max_iterations == 0 {
            return Err("El numero maximo de iteraciones debe ser mayor que cero");
        }
        let mut x = Matrix::new(self.coefficient.rows, 1)?;
        for _ in 0..max_iterations {
            // Error maximo de la iteracion actual
            let mut max_error = BigDecimal::zero();
            // Recorrer las filas
            for i in 0..self.coefficient.rows {
                // Obtener termino independiente
                let b = self.independent.get(i, 0)
                    .ok_or("No se pudo acceder al termino independiente")?;
                // Obtener valor anterior de la incognita
                let previous = x.get(i, 0)
                    .ok_or("No se pudo acceder a la incognita")?
                    .clone();
                // Inicializar sumatoria
                let mut sum = BigDecimal::zero();
                // Recorrer las columnas
                for j in 0..self.coefficient.columns {
                    if i != j {
                        let a = self.coefficient.get(i, j)
                            .ok_or("No se pudo acceder al coeficiente")?;
                        let xj = x.get(j, 0)
                            .ok_or("No se pudo acceder a la incognita")?;
                        sum += a * xj;
                    }
                }
                // Obtener elemento de la diagonal
                let diagonal = self.coefficient.get(i, i)
                    .ok_or("No se pudo acceder al elemento diagonal")?;
                // Calcular nuevo valor de la incognita
                let new_value = (b - &sum) / diagonal;
                // Calcular diferencia absoluta
                let error = (&new_value - &previous).abs();
                // Actualizar error maximo
                if error > max_error {
                    max_error = error;
                }
                // Actualizar inmediatamente la incognita
                x.set(i, 0, new_value)?;
            }
            // Comprobar convergencia numerica
            if max_error <= tolerance {
                return Ok(x);
            }
        }
        // No se alcanzo la tolerancia
        Err("Se alcanzo el limite de iteraciones sin converger")
        }

}
