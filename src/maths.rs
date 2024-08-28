mod complex;
pub mod format;
mod fraction;
mod sqrt;

use complex::Complex;
use fraction::{ComplexFraction, Fraction};
use sqrt::sqrt;
use std::collections::BTreeMap;

pub fn solve_polynomial(degree: u32, coefficients: BTreeMap<u32, f64>) {
    match degree {
        0 => solve_degree_0(*coefficients.get(&0).unwrap_or(&0.0)),
        1 => solve_degree_1(
            *coefficients.get(&0).unwrap_or(&0.0),
            *coefficients.get(&1).unwrap_or(&0.0),
        ),
        2 => solve_degree_2(
            *coefficients.get(&0).unwrap_or(&0.0),
            *coefficients.get(&1).unwrap_or(&0.0),
            *coefficients.get(&2).unwrap_or(&0.0),
        ),
        _ => {
            eprintln!("The polynomial degree is strictly greater than 2, I can't solve.");
            std::process::exit(1);
        }
    }
}

fn solve_degree_0(a: f64) {
    if a == 0.0 {
        println!("The equation is true for all x (infinite solutions).");
        return;
    }

    println!("No solution exists.");
}

fn solve_degree_1(a: f64, b: f64) {
    if b == 0.0 {
        solve_degree_0(a);
        return;
    }

    let x = -a / b;

    println!("The solution is:");
    println!("{:.6} \u{2248} {}", x, Fraction::from_float(x, None));
}

fn solve_degree_2(a: f64, b: f64, c: f64) {
    if c == 0.0 {
        solve_degree_1(a, b);
        return;
    }

    let discriminant = b * b - 4.0 * a * c;
    println!("The discriminant is: {:.1}", discriminant);

    match discriminant {
        discriminant if discriminant > 0.0 => {
            let sqrt_discriminant = sqrt(discriminant);

            let x1 = (-b - sqrt_discriminant) / (2.0 * c);
            let x2 = (-b + sqrt_discriminant) / (2.0 * c);

            println!("Discriminant is strictly positive, the two solutions are:");
            println!("{:.6} \u{2248} {}", x1, Fraction::from_float(x1, None));
            println!("{:.6} \u{2248} {}", x2, Fraction::from_float(x2, None));
        }

        0.0 => {
            let x = -b / (2.0 * c);

            println!("Discriminant is zero, the solution is:");
            println!("{:.6} \u{2248} {}", x, Fraction::from_float(x, None));
        }

        _ => {
            let sqrt_discriminant = sqrt(-discriminant);
            let real_part = -b / (2.0 * c);
            let imaginary_part = sqrt_discriminant / (2.0 * c);

            let x1 = Complex::new(real_part, imaginary_part);
            let x2 = Complex::new(real_part, -imaginary_part);

            println!("Discriminant is strictly negative, the two solutions are:");
            println!(
                "{:.6} \u{2248} {}",
                x1,
                ComplexFraction::from_complex(&x1, None)
            );
            println!(
                "{:.6} \u{2248} {}",
                x2,
                ComplexFraction::from_complex(&x2, None)
            );
        }
    }
}
