mod ast;
mod node;
mod token;

use ast::AbstractSyntaxTree;
use token::{tokenize, Token};

pub fn parse() {
    let env: Vec<String> = std::env::args().collect();

    if env.len() != 2 {
        eprintln!("Usage: {} <expression>", env[0]);
        std::process::exit(2);
    }

    let expression: &str = &env[1];

    let tokens: Vec<Token> = tokenize(expression);

    let mut ast = AbstractSyntaxTree::new(tokens);
    let nodes = ast.parse();

    println!("{:?}", nodes);
}
