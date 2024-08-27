mod maths;
mod parsing;

fn main() {
    let coefficients = parsing::parse();
    let degree = coefficients.keys().max().unwrap_or(&0);

    println!(
        "Reduced form: {}",
        maths::format::get_reduced_form(&coefficients)
    );
    println!("Polynomial degree: {}", degree);

    if *degree > 2 {
        eprintln!("The polynomial degree is strictly greater than 2, I can't solve.");
        std::process::exit(1);
    }
}
