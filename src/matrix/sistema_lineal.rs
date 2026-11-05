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
    pub fn gauss_seidel(&self, tolerance: BigDecimal, max_iterations: usize) -> Result<Matrix, &'static str> {
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
            let mut max_error = BigDecimal::zero();
            for i in 0..self.coefficient.rows {
                let b = self.independent.get(i, 0)
                    .ok_or("No se pudo acceder al termino independiente")?;
                let previous = x.get(i, 0)
                    .ok_or("No se pudo acceder a la incognita")?
                    .clone();
                let mut sum = BigDecimal::zero();
                for j in 0..self.coefficient.columns {
                    if i != j {
                        let a = self.coefficient.get(i, j)
                            .ok_or("No se pudo acceder al coeficiente")?;
                        let xj = x.get(j, 0)
                            .ok_or("No se pudo acceder a la incognita")?;
                        sum += a * xj;
                    }
                }
                let diagonal = self.coefficient.get(i, i)
                    .ok_or("No se pudo acceder al elemento diagonal")?;
                let new_value = (b - &sum) / diagonal;
                let error = (&new_value - &previous).abs();
                if error > max_error {
                    max_error = error;
                }
                x.set(i, 0, new_value)?;
            }
            if max_error <= tolerance {
                return Ok(x);
            }
        }
        Err("Se alcanzo el limite de iteraciones sin converger")
        }

    pub fn jacobi(&self, tolerance: BigDecimal, max_iterations: usize) -> Result<Matrix, &'static str> {
        if !self.coefficient.is_diagonally_dominant() {
            return Err("La matriz no cumple la condicion de dominancia diagonal estricta");
        }
        if self.independent.rows != self.coefficient.rows
            || self.independent.columns != 1 {
            return Err("El vector independiente tiene dimensiones incompatibles");
        }
        if tolerance <= BigDecimal::zero() {
            return Err("La tolerancia debe ser mayor que cero");
        }
        if max_iterations == 0 {
            return Err("El numero maximo de iteraciones debe ser mayor que cero");
        }
        let mut x = Matrix::new(self.coefficient.rows, 1)?;
        let mut next_x = Matrix::new(self.coefficient.rows, 1)?;
        for _ in 0..max_iterations {
            let mut max_error = BigDecimal::zero();
            for i in 0..self.coefficient.rows {
                let b = self.independent.get(i, 0)
                    .ok_or("No se pudo acceder al termino independiente")?;
                let mut sum = BigDecimal::zero();
                for j in 0..self.coefficient.columns {
                    if i != j {
                        let a = self.coefficient.get(i, j)
                            .ok_or("No se pudo acceder al coeficiente")?;
                        let xj = x.get(j, 0)
                            .ok_or("No se pudo acceder a la incognita")?;
                        sum += a * xj;
                    }
                }
                let diagonal = self.coefficient.get(i, i)
                    .ok_or("No se pudo acceder al elemento diagonal")?;
                let new_value = (b - &sum) / diagonal;
                let previous = x.get(i, 0)
                    .ok_or("No se pudo acceder a la incognita")?;
                let error = (&new_value - previous).abs();
                if error > max_error {
                    max_error = error;
                }
                next_x.set(i, 0, new_value)?;
            }
            if max_error <= tolerance {
                return Ok(next_x);
            }
            std::mem::swap(&mut x, &mut next_x);
        }
        Err("Se alcanzo el limite de iteraciones sin converger")
    }

}
