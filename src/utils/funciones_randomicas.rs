use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::BigInt;
use rand::RngExt;

// genera un numero decimal aleatorio entre min y max,
// con una cantidad aleatoria de decimales entre 0 y max_decimals.
pub fn random_decimal(min: i64, max: i64, max_decimals: u8) -> Result<BigDecimal, &'static str> {
    if min >= max {
        return Err("El valor minimo debe ser menor al valor maximo");
    }
    let mut rng = rand::rng();
    let integer = rng.random_range(min..max);
    let scale = rng.random_range(0..=max_decimals);
    let multiplier = BigInt::from(10).pow(scale as u32);
    let integer_part = BigInt::from(integer) * &multiplier;
    let mut decimal_part = BigInt::from(0);
    for _ in 0..scale {
        let digit = rng.random_range(0_u8..10_u8);
        decimal_part = decimal_part * 10 + BigInt::from(digit);
    }
    let coefficient = if integer < 0 {
        integer_part - decimal_part
    } else {
        integer_part + decimal_part
    };
    Ok(BigDecimal::new(coefficient, scale as i64))
}