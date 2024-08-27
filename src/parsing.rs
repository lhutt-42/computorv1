mod ast;
mod node;
mod token;

use std::collections::BTreeMap;

use ast::AbstractSyntaxTree;
use node::AbstractSyntaxTreeNode;
use token::{tokenize, Token};

pub fn parse() -> BTreeMap<u32, f64> {
    let env: Vec<String> = std::env::args().collect();

    if env.len() != 2 {
        eprintln!("Usage: {} <expression>", env[0]);
        std::process::exit(2);
    }

    let expression: &str = &env[1];

    let tokens: Vec<Token> = tokenize(expression);

    let mut ast = AbstractSyntaxTree::new(tokens);
    let nodes = ast.parse();

    let coefficients: BTreeMap<u32, f64> = match nodes {
        AbstractSyntaxTreeNode::Equation { left, right } => {
            let left_coefficients = left.extract_coefficients();
            let right_coefficients = right.extract_coefficients();

            let mut coefficients = left_coefficients.clone();
            for (exponent, coefficient) in right_coefficients {
                *coefficients.entry(exponent).or_insert(0.0) -= coefficient;
            }

            coefficients.retain(|_, &mut coefficient| coefficient != 0.0);
            coefficients
        }
        _ => {
            eprintln!("Unexpected node");
            std::process::exit(2);
        }
    };

    coefficients
}
