use std::fmt;

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = match self.imaginary.is_sign_positive() {
            true => "+",
            false => "-",
        };

        let real_str = format!("{:.1$}", self.real, f.precision().unwrap_or(10));
        let imaginary_str = format!("{:.1$}", self.imaginary.abs(), f.precision().unwrap_or(10));

        write!(f, "{} {} {}i", real_str, sign, imaginary_str)
    }
}
