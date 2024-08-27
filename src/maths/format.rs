use std::collections::BTreeMap;
use std::fmt::Write;

fn format_sign(coefficient: f64, is_first: bool) -> String {
    match (coefficient.is_sign_positive(), is_first) {
        (true, true) => String::new(),
        (true, false) => "+ ".to_string(),
        (false, _) => "- ".to_string(),
    }
}

fn format_term(coefficient: f64, exponent: u32, is_first: bool) -> String {
    let sign = format_sign(coefficient, is_first);
    match exponent {
        0 => format!("{}{}", sign, coefficient.abs()),
        1 => format!("{}{} * X", sign, coefficient.abs()),
        _ => format!("{}{} * X^{}", sign, coefficient.abs(), exponent),
    }
}

pub fn get_reduced_form(coefficients: &BTreeMap<u32, f64>) -> String {
    let mut result = String::new();
    let mut is_first = true;

    for (exponent, coefficient) in coefficients {
        if is_first == false {
            write!(result, " ").unwrap();
        }

        write!(result, "{}", format_term(*coefficient, *exponent, is_first)).unwrap();

        if is_first == true {
            is_first = false;
        }
    }
    write!(result, " = 0").unwrap();
    result
}
