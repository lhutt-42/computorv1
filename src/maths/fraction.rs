use super::Complex;
use std::fmt;

#[derive(Debug)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

const DEFAULT_PRECISION: f64 = 1e-3;

impl Fraction {
    pub fn from_float(num: f64, precision: Option<f64>) -> Self {
        let sign: i64 = if num.is_sign_positive() { 1 } else { -1 };
        let num: f64 = num.abs();

        let mut lower: (i64, i64) = (0, 1);
        let mut upper: (i64, i64) = (1, 0);

        let mut fraction: (i64, i64) = (1, 1);
        let precision = precision.unwrap_or(DEFAULT_PRECISION);

        while fraction.1 <= 1_000_000 {
            let mediant: (i64, i64) = (lower.0 + upper.0, lower.1 + upper.1);

            let mediant_value: f64 = mediant.0 as f64 / mediant.1 as f64;

            if (mediant_value - num).abs() <= precision {
                fraction = mediant;
                break;
            }

            if mediant_value < num {
                lower = mediant;
            } else {
                upper = mediant;
            }
        }

        let (numerator, denominator) = fraction;
        let gcd_value = Fraction::gcd(numerator.abs() as u64, denominator.abs() as u64) as i64;

        Fraction {
            numerator: numerator / gcd_value * sign,
            denominator: denominator / gcd_value,
        }
    }

    fn gcd(a: u64, b: u64) -> u64 {
        match b {
            0 => a,
            _ => Fraction::gcd(b, a % b),
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

#[derive(Debug)]
pub struct ComplexFraction {
    pub real: Fraction,
    pub imaginary: Fraction,
}

impl ComplexFraction {
    pub fn from_complex(num: &Complex, precision: Option<f64>) -> Self {
        ComplexFraction {
            real: Fraction::from_float(num.real, precision),
            imaginary: Fraction::from_float(num.imaginary, precision),
        }
    }
}

impl fmt::Display for ComplexFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + ({})i", self.real, self.imaginary)
    }
}
