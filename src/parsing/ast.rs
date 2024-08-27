use super::token::Token;

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

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    tokens: Vec<Token>,
    pos: usize,
}

impl AbstractSyntaxTree {
    pub fn new(tokens: Vec<Token>) -> Self {
        AbstractSyntaxTree { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> AbstractSyntaxTreeNode {
        let left = self.parse_expression();

        if !self.match_token(&[Token::Equal]) {
            eprintln!("The expression must contain an equal sign");
            std::process::exit(2);
        }
        self.advance();

        let right = self.parse_expression();

        if self.pos != self.tokens.len() {
            eprintln!("Unexpected tokens at the end of the expression");
            std::process::exit(2);
        }

        AbstractSyntaxTreeNode::Equation {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn parse_expression(&mut self) -> AbstractSyntaxTreeNode {
        let mut node = self.parse_term();

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match self.tokens.get(self.pos) {
                Some(value) => value.clone(),
                None => {
                    eprintln!("Unexpected end of input");
                    std::process::exit(2);
                }
            };

            self.advance();

            let right = self.parse_term();
            node = AbstractSyntaxTreeNode::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        node
    }

    fn parse_term(&mut self) -> AbstractSyntaxTreeNode {
        let mut node = self.parse_factor();

        while self.match_token(&[Token::Multiply]) {
            self.advance();

            let right = self.parse_factor();

            node = AbstractSyntaxTreeNode::BinaryOp {
                left: Box::new(node),
                op: Token::Multiply,
                right: Box::new(right),
            };
        }

        node
    }

    fn parse_factor(&mut self) -> AbstractSyntaxTreeNode {
        let mut node = self.parse_primary();

        while self.match_token(&[Token::Exponent]) {
            self.advance();

            let right = self.parse_primary();

            node = AbstractSyntaxTreeNode::BinaryOp {
                left: Box::new(node),
                op: Token::Exponent,
                right: Box::new(right),
            };
        }

        node
    }
    fn parse_primary(&mut self) -> AbstractSyntaxTreeNode {
        let token = match self.tokens.get(self.pos) {
            Some(value) => value.clone(),
            None => {
                eprintln!("Unexpected end of input");
                std::process::exit(2);
            }
        };

        match token {
            Token::Number(value) => {
                self.advance();
                AbstractSyntaxTreeNode::Number(value)
            }
            Token::Variable(name) => {
                self.advance();
                AbstractSyntaxTreeNode::Variable(name)
            }
            _ => {
                eprintln!("Expected Number or Variable but found {:?}", token);
                std::process::exit(2);
            }
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn match_token(&self, types: &[Token]) -> bool {
        if let Some(token) = self.tokens.get(self.pos) {
            return types.iter().any(|t| t == token);
        }

        false
    }
}
