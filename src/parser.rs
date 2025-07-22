use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Var(String, Option<Type>),
    Abs(String, Option<Type>, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(String),
    Bool,
    Nat,
    Arrow(Box<Type>, Box<Type>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(name, Some(ty)) => write!(f, "{}:{}", name, ty),
            Term::Var(name, None) => write!(f, "{}", name),
            Term::Abs(param, Some(ty), body) => write!(f, "λ{}:{}.({})", param, ty, body),
            Term::Abs(param, None, body) => write!(f, "λ{}.({})", param, body),
            Term::App(left, right) => write!(f, "({}) ({})", left, right),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "Bool"),
            Type::Nat => write!(f, "Nat"),
            Type::Named(name) => write!(f, "{}", name),
            Type::Arrow(left, right) => write!(f, "({} -> {})", left, right),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        if tok.is_some() {
            self.position += 1;
        }
        tok
    }

    fn expect(&mut self, expected: Token) {
        let next = self.advance();
        if next != Some(&expected) {
            panic!("Esperado {:?}, mas veio {:?}", expected, next);
        }
    }

    pub fn parse_term(&mut self) -> Term {
        self.parse_application()
    }

    fn parse_abstraction(&mut self) -> Term {
        self.expect(Token::Lambda);

        let name = match self.advance() {
            Some(Token::Var(name)) => name.clone(),
            other => panic!("Esperado nome de variável após λ, mas veio: {:?}", other),
        };

        let param_type = if self.peek() == Some(&Token::Colon) {
            self.advance();
            Some(self.parse_type())
        } else {
            None
        };

        self.expect(Token::Dot);
        let body = self.parse_term();

        Term::Abs(name, param_type, Box::new(body))
    }

    fn parse_type(&mut self) -> Type {
        let mut ty = self.parse_simple_type();

        while self.peek() == Some(&Token::Arrow) {
            self.advance();
            let right = self.parse_type();
            ty = Type::Arrow(Box::new(ty), Box::new(right));
        }

        ty
    }

    fn parse_application(&mut self) -> Term {
        let mut term = self.parse_atom();

        while let Some(token) = self.peek() {
            match token {
                Token::Var(_) | Token::LParen | Token::Lambda => {
                    let arg = self.parse_atom();
                    term = Term::App(Box::new(term), Box::new(arg));
                }
                _ => break,
            }
        }

        term
    }

    fn parse_atom(&mut self) -> Term {
        match self.peek() {
            Some(Token::Var(_)) => {
                let name = if let Token::Var(name) = self.advance().unwrap() {
                    name.clone()
                } else {
                    unreachable!()
                };

                let ty = if self.peek() == Some(&Token::Colon) {
                    self.advance();
                    Some(self.parse_type())
                } else {
                    None
                };

                Term::Var(name.clone(), ty)
            }

            Some(Token::Lambda) => self.parse_abstraction(),

            Some(Token::LParen) => {
                self.advance();
                let term = self.parse_term();
                self.expect(Token::RParen);
                term
            }

            _ => panic!("Token não esperado: {:?}", self.peek()),
        }
    }

    fn parse_simple_type(&mut self) -> Type {
        match self.advance() {
            Some(Token::Type(t)) if t == "Bool" => Type::Bool,
            Some(Token::Type(t)) if t == "Nat" => Type::Nat,
            Some(Token::Type(t)) => Type::Named(t.clone()),
            Some(Token::LParen) => {
                let ty = self.parse_type();
                self.expect(Token::RParen);
                ty
            }
            other => panic!("Tipo não esperado: {:?}", other),
        }
    }
}
