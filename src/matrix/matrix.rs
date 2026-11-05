use bigdecimal::BigDecimal;
use crate::utils::funciones_randomicas::{self, random_decimal};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<BigDecimal>,
    pub rows: usize,
    pub columns: usize,
}

impl fmt::Display for Matrix {
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

}
