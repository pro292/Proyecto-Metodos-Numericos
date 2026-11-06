use bigdecimal::{BigDecimal, Zero, RoundingMode};
use crate::utils::funciones_randomicas::{self, random_decimal};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<BigDecimal>,
    pub rows: usize,
    pub columns: usize,
}

// implementa el treat Display al tipo Matrix
// para poder hacer pruebas
impl fmt::Display for Matrix {

    // define el comportamimento de acuerdo al trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let index = i * self.columns + j;
                if j > 0 {
                    write!(f, "\t")?;
                }
                write!(f, "{}", self.data[index])?;
            }
            if i + 1 < self.rows {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Matrix {

    // constructor
    // contruye una matriz vacia(ceros) de RxC
    pub fn new(rows: usize, columns: usize) -> Result<Matrix, &'static str> {
        let total = rows.checked_mul(columns)
            .ok_or("Las dimensiones de la matriz exceden el limite")?;
        Ok(Self {data: vec![BigDecimal::from(0); total], rows: rows, columns: columns})
    }

    // Genera una matriz de elementos aleatorios de RxC
    pub fn matrix_generate(r: usize, c: usize, min: i64, max: i64, max_decimals: u8) -> Result<Self, &'static str> {
        if min >= max {
            return Err("El valor minimo debe ser menor al valor maximo");
        }
        let total = r.checked_mul(c)
            .ok_or("Las dimensiones de la matriz exceden el limite")?;
        let mut data = Vec::with_capacity(total);
        for _ in 0..total {
            data.push(random_decimal(min, max, max_decimals)?);
        }
        Ok(Self {data, rows: r, columns: c})
    }

    // Genera una matriz cuadrada estrictamente diagonalmente dominante
    pub fn diagonally_dominant_matrix_generate(r: usize, c: usize, min: i64, max: i64, max_decimals: u8) -> Result<Self, &'static str> {    
        if r != c || r == 0 {
            return Err(
                "La matriz debe ser cuadrada y tener dimensiones mayores a cero"
            );
        }
        if min >= max {
            return Err("El valor minimo debe ser menor al valor maximo");
        }
        let total = r.checked_mul(c)
            .ok_or("Las dimensiones de la matriz exceden el limite")?;
        let mut data = vec![BigDecimal::from(0); total];
        let mut dominant_position = 0;
        for _ in 0..r {
            let mut sum = BigDecimal::from(0);
            for j in 0..c {
                if j != dominant_position {
                    let value = random_decimal(min, max, max_decimals)?;
                    sum += value.clone().abs();
                    let index = dominant_position * c + j;
                    data[index] = value;
                }
            }
            let dominant = sum
                + BigDecimal::from(1).with_scale(max_decimals as i64);
            let index = dominant_position * c + dominant_position;
            data[index] = dominant;
            dominant_position += 1;
        }
        Ok(Self {data, rows: r, columns: c,})
    }

     // Obtiene una referencia al elemento (r, c)
    pub fn get(&self, r: usize, c: usize) -> Option<&BigDecimal> {
        if r >= self.rows || c >= self.columns {
            return None;
        }
        let index = r * self.columns + c;
        self.data.get(index)
    }

    // Modifica el elemento (r, c)
    pub fn set(&mut self, r: usize, c: usize, value: BigDecimal) -> Result<(), &'static str> {
        if r >= self.rows || c >= self.columns {
            return Err("Indices fuera de rango");
        }
        let index = r * self.columns + c;
        self.data[index] = value;
        Ok(())  
    }

    // verifica si la matriz es diagonalmente dominante
    // devolvera true si lo es, caso contrario devolvera false
    pub fn is_diagonally_dominant(&self) -> bool {
        if self.rows == 0 || self.rows != self.columns {
            return false;
        }
        for i in 0..self.rows {
            let diagonal = match self.get(i, i) {
                Some(value) => value.clone().abs(),
                None => return false,
            };
            let mut sum = BigDecimal::from(0);
            for j in 0..self.columns {
                if i != j {
                    let value = match self.get(i, j) {
                        Some(value) => value.clone().abs(),
                        None => return false,
                    };
                    sum += value;
                }
            }
            if diagonal <= sum {
                return false;
            }
        }
        true
    }

    // metodo para permutar filas
    pub fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), &'static str> {
        if row1 >= self.rows || row2 >= self.rows {
            return Err("Indices de fila fuera de rango");
        }
        for j in 0..self.columns {
            let index1 = row1 * self.columns + j;
            let index2 = row2 * self.columns + j;
            self.data.swap(index1, index2);
        }
        Ok(())
    }

    // multiplica toda una fila por un escalar
    pub fn multiply_row(&mut self, row: usize, scalar: BigDecimal) -> Result<(), &'static str> {
        if row >= self.rows {
            return Err("Indice de fila fuera de rango");
        }
        for j in 0..self.columns {
            let index = row * self.columns + j;
            self.data[index] *= scalar.clone();
        }
        Ok(())
    }

    pub fn subtract_row_multiple(&mut self, target_row: usize, source_row: usize, scalar: BigDecimal) -> Result<(), &'static str> {
        if target_row >= self.rows || source_row >= self.rows {
            return Err("Indices de fila fuera de rango");
        }
        for j in 0..self.columns {
            let target_index = target_row * self.columns + j;
            let source_index = source_row * self.columns + j;
            let source_value = self.data[source_index].clone();
            self.data[target_index] -= scalar.clone() * source_value;
        }
        Ok(())
    }

    pub fn lu_determinant(&self) -> Result<BigDecimal, &'static str>{
        if self.rows == 0 || self.rows != self.columns {
            return Err("La matriz debe ser cuadrada y tener dimensiones mayores a cero");
        }
        let precision = self.calculate_precision();
        let mut upper = self.clone();
        let n = self.rows;
        let mut np = 0;
        for k in 0..n {
            let mut pivot_row = k;
            for i in (k + 1)..n {
                let current = upper.data[i * n + k].clone().abs();
                let pivot = upper.data[pivot_row * n + k].clone().abs();
                if current > pivot {
                    pivot_row = i;
                }
            }
            if upper.data[pivot_row * n + k].is_zero() {
                return Ok(BigDecimal::zero());
            }
            if pivot_row != k {
                upper.swap_rows(k, pivot_row)?;
                np += 1;
            }
            let pivot = upper.data[k * n + k].clone();
            for i in (k + 1)..n {
                let factor = (
                    upper.data[i * n + k].clone() / pivot.clone()
                ).with_scale_round(precision, RoundingMode::HalfEven);
                upper.subtract_row_multiple(i, k, factor)?;
                for j in (k + 1)..n {
                    let index = i * n + j;
                    upper.data[index] = upper.data[index]
                        .clone()
                        .with_scale_round(precision, RoundingMode::HalfEven);
                }
                upper.data[i * n + k] = BigDecimal::zero();
            }
        }
        let mut determinant = BigDecimal::from(1);
        for i in 0..n {
            determinant *= upper.data[i * n + i].clone();
        }
        if np % 2 != 0 {
            determinant = -determinant;
        }
        determinant = determinant.with_scale_round(precision, RoundingMode::HalfEven);
        Ok(determinant)
    }

    // establece una precision minima de 8, pero si 
    // la matriz tiene una precision mayor tomara la mayor precision
    // presente en la matriz
    fn calculate_precision(&self) -> i64 {
        let max_scale = self.data.iter()
            .map(|value| value.as_bigint_and_exponent().1)
            .max()
            .unwrap_or(0);
        if max_scale <= 8 {
            8
        } else {
            max_scale + max_scale / 2
        }
    }

    //calcula el determinante usando el metodo de Leibnizs
    pub fn leibniz_determinant(&self) -> Result<BigDecimal, &'static str> {
        if self.rows == 0 || self.rows != self.columns {
            return Err("La matriz debe ser cuadrada y tener dimensiones mayores a cero");
        }
        let n = self.rows;
        let mut permutation: Vec<usize> = (0..n).collect();
        let product = |p: &[usize]| -> BigDecimal {
            (0..n).fold(BigDecimal::from(1), |acc, i| {
                acc * self.data[i * n + p[i]].clone()
            })
        };
        let mut total = product(&permutation);
        let mut positive = true;
        let mut counters = vec![0; n];
        let mut i = 1;
        while i < n {
            if counters[i] < i {
                let j = if i % 2 == 0 { 0 } else { counters[i] };
                permutation.swap(j, i);
                positive = !positive;
                let term = product(&permutation);
                if positive {
                    total += term;
                } else {
                    total -= term;
                }
                counters[i] += 1;
                i = 1;
            } else {
                counters[i] = 0;
                i += 1;
            }
        }
        Ok(total)
    }

    // calcula el determinante usando el metodo de Laplace
    pub fn laplace_determinant(&self) -> Result<BigDecimal, &'static str> {
        if self.rows == 0 || self.rows != self.columns {
            return Err("La matriz debe ser cuadrada y tener dimensiones mayores a cero");
        }
        if self.rows == 1 {
            return Ok(self.data[0].clone());
        }
        let mut total = BigDecimal::zero();
        for j in 0..self.columns {
            let minor = self.minor(0, j)?;
            let term = self.data[j].clone() * minor.laplace_determinant()?;
            if j % 2 == 0 {
                total += term;
            } else {
                total -= term;
            }
        }
        Ok(total)
    }

    // contruye una matriz eliminando la fila y columna especificadas
    fn minor(&self, excluded_row: usize, excluded_column: usize) -> Result<Self, &'static str> {
        if excluded_row >= self.rows || excluded_column >= self.columns {
            return Err("Indices fuera de rango");
        }
        let mut minor = Self::new(self.rows - 1, self.columns - 1)?;
        let mut minor_row = 0;
        for i in 0..self.rows {
            if i == excluded_row {
                continue;
            }
            let mut minor_column = 0;
            for j in 0..self.columns {
                if j == excluded_column {
                    continue;
                }
                let original_index = i * self.columns + j;
                let minor_index = minor_row * minor.columns + minor_column;
                minor.data[minor_index] = self.data[original_index].clone();
                minor_column += 1;
            }
            minor_row += 1;
        }
        Ok(minor)
    }

}
