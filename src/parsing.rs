mod ast;
mod tokenize;

use ast::AbstractSyntaxTree;
use tokenize::{tokenize, Token};

pub fn parse() {
    let env: Vec<String> = std::env::args().collect();

    if env.len() != 2 {
        eprintln!("Usage: {} <expression>", env[0]);
        std::process::exit(2);
    }

    let expression: &str = &env[1];

    // Filter out whitespace tokens
    let tokens: Vec<Token> = tokenize(expression)
        .into_iter()
        .filter(|token| *token != Token::Whitespace)
        .collect::<Vec<Token>>();

    let mut ast = AbstractSyntaxTree::new(tokens);
    let nodes = ast.parse();

    println!("AST: {:#?}", nodes);
}
