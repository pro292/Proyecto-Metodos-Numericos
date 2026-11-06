use bigdecimal::BigDecimal;
use std::time::Duration;

/// Formatea una duración de tiempo en nanosegundos, microsegundos, milisegundos o segundos
pub fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{} ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2} µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.3} ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.4} s", d.as_secs_f64())
    }
}

/// Trunca cadenas numéricas largas para mostrar en celdas de tabla
pub fn format_decimal_short(val: &BigDecimal, max_len: usize) -> String {
    let s = val.to_string();
    if s.len() > max_len {
        let keep = max_len.saturating_sub(3);
        format!("{}...", &s[..keep])
    } else {
        s
    }
}

/// Parsea un texto ingresado por el usuario como BigDecimal de forma segura
pub fn parse_tolerance_string(s: &str) -> Result<BigDecimal, &'static str> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err("La tolerancia no puede estar vacía");
    }

    // Soporte para notación científica como 1e-4 o 1E-6
    if let Some((base_str, exp_str)) = trimmed.split_once(['e', 'E']) {
        let base = base_str.parse::<f64>().map_err(|_| "Formato base inválido en notación científica")?;
        let exp = exp_str.parse::<i32>().map_err(|_| "Exponente inválido en notación científica")?;
        let computed = base * 10f64.powi(exp);
        if computed <= 0.0 {
            return Err("La tolerancia debe ser mayor a 0");
        }
        let formatted = format!("{:.15}", computed);
        let trimmed_fmt = formatted.trim_end_matches('0');
        return trimmed_fmt.parse::<BigDecimal>().map_err(|_| "Error al convertir tolerancia científica");
    }

    match trimmed.parse::<BigDecimal>() {
        Ok(val) => {
            if val <= BigDecimal::from(0) {
                Err("La tolerancia debe ser estrictamente mayor a 0")
            } else {
                Ok(val)
            }
        }
        Err(_) => Err("Formato numérico de tolerancia inválido (ejemplo: 0.0001 ó 1e-5)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_nanos(500)), "500 ns");
        assert_eq!(format_duration(Duration::from_micros(15)), "15.00 µs");
        assert_eq!(format_duration(Duration::from_millis(250)), "250.000 ms");
        assert!(format_duration(Duration::from_secs(2)).contains("s"));
    }

    #[test]
    fn test_parse_tolerance() {
        assert!(parse_tolerance_string("0.001").is_ok());
        assert!(parse_tolerance_string("1e-5").is_ok());
        assert!(parse_tolerance_string("1E-6").is_ok());
        assert!(parse_tolerance_string("-0.01").is_err());
        assert!(parse_tolerance_string("0").is_err());
        assert!(parse_tolerance_string("abc").is_err());
    }
}

