mod matrix;
mod utils;

use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::matrix::matrix::Matrix;
use crate::matrix::sistema_lineal::SistemaLineal;
use crate::utils::funciones_randomicas::random_decimal;

fn main() {
    let m1 = Matrix::diagonally_dominant_matrix_generate(5, 5, -100, 100, 10).unwrap();
    let m2 = Matrix::matrix_generate(5, 1, -100, 100, 4).unwrap();
    let sl1 = SistemaLineal::new(m1.clone(), m2.clone());
    let tolerance = BigDecimal::new(BigInt::from_u16(1).unwrap(), 5);
    let solve = sl1.jacobi(tolerance, 100);
    println!("matriz: {}", m1);
    println!("determinante LU: {}", m1.lu_determinant().unwrap());
    println!("determinante Leibniz: {}", m1.leibniz_determinant().unwrap());
    println!("determinante Laplace: {}", m1.laplace_determinant().unwrap());
    //println!("solve {:?}", solve);
}
