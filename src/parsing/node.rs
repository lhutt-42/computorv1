use super::token::Token;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTreeNode {
    Number(f64),
    Variable(char),
    BinaryOp {
        left: Box<AbstractSyntaxTreeNode>,
        op: Token,
        right: Box<AbstractSyntaxTreeNode>,
    },
    Equation {
        left: Box<AbstractSyntaxTreeNode>,
        right: Box<AbstractSyntaxTreeNode>,
    },
}

impl AbstractSyntaxTreeNode {
    pub fn extract_coefficients(&self) -> BTreeMap<u32, f64> {
        let mut coefficients: BTreeMap<u32, f64> = BTreeMap::new();

        self.collect_coefficients(1.0, 0, &mut coefficients);
        coefficients
    }

    fn collect_coefficients(
        &self,
        coefficient: f64,
        exponent: u32,
        coefficients: &mut BTreeMap<u32, f64>,
    ) {
        match self {
            AbstractSyntaxTreeNode::Number(value) => {
                *coefficients.entry(exponent).or_insert(0.0) += coefficient * value;
            }

            AbstractSyntaxTreeNode::Variable(_) => {
                *coefficients.entry(exponent).or_insert(0.0) += coefficient;
            }

            AbstractSyntaxTreeNode::BinaryOp { left, op, right } => {
                match op {
                    Token::Plus => {
                        left.collect_coefficients(coefficient, exponent, coefficients);
                        right.collect_coefficients(coefficient, exponent, coefficients);
                    }

                    Token::Minus => {
                        left.collect_coefficients(coefficient, exponent, coefficients);
                        right.collect_coefficients(-coefficient, exponent, coefficients);
                    }

                    Token::Multiply => {
                        match (&**left, &**right) {
                            // Number * X
                            (AbstractSyntaxTreeNode::Number(value), _) => {
                                right.collect_coefficients(
                                    coefficient * value,
                                    exponent,
                                    coefficients,
                                );
                            }
                            // X * Number
                            (_, AbstractSyntaxTreeNode::Number(value)) => {
                                left.collect_coefficients(
                                    coefficient * value,
                                    exponent,
                                    coefficients,
                                );
                            }
                            // X * X
                            _ => {
                                eprintln!("Unsupported multiplication between non-numeric values");
                                std::process::exit(2);
                            }
                        }
                    }

                    Token::Exponent => {
                        if let AbstractSyntaxTreeNode::Number(_) = **left {
                            eprintln!("Unsupported exponentiation with non-variable base");
                            std::process::exit(2);
                        }

                        if let AbstractSyntaxTreeNode::Number(value) = **right {
                            match value.fract() {
                                0.0 => {
                                    left.collect_coefficients(
                                        coefficient,
                                        exponent + value as u32,
                                        coefficients,
                                    );
                                }
                                _ => {
                                    eprintln!("Exponent must be an integer");
                                    std::process::exit(2);
                                }
                            }
                        } else {
                            eprintln!("Unsupported exponentiation with non-numeric exponent");
                            std::process::exit(2);
                        }
                    }

                    _ => {
                        eprintln!("Unsupported binary operation: {:?}", op);
                        std::process::exit(2);
                    }
                }
            }
            _ => {
                eprintln!("Unsupported AST node: {:?}", self);
                std::process::exit(2);
            }
        }
    }
}
