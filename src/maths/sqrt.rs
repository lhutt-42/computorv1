// Newton's method for finding the square root of a number
pub fn sqrt(value: f64) -> f64 {
    if value < 0.0 {
        eprintln!("Cannot compute the square root of a negative number.");
        std::process::exit(1);
    }

    let mut x = value;
    let mut y = 1.0;
    let e = 0.00001; // Convergence criterion

    while (x - y).abs() > e {
        x = (x + y) / 2.0;
        y = value / x;
    }

    x
}
