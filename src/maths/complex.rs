use std::fmt;

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format = match (self.real, self.imaginary) {
            (0.0, 0.0) => format!("0"),
            (0.0, _) => format!("{}i", self.imaginary),
            (_, 0.0) => format!("{}", self.real),
            (_, _) => {
                if self.imaginary < 0.0 {
                    format!("{} - {}i", self.real, -self.imaginary.abs())
                } else {
                    format!("{} + {}i", self.real, self.imaginary.abs())
                }
            }
        };
        write!(f, "{}", format)
    }
}
