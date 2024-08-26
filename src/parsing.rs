mod tokenize;

pub fn parse() {
    let env: Vec<String> = std::env::args().collect();

    if env.len() != 2 {
        eprintln!("Usage: {} <expression>", env[0]);
        std::process::exit(2);
    }

    let expression: &str = &env[1];

    let tokens = tokenize::tokenize(expression);
    println!("{:?}", tokens);
}
