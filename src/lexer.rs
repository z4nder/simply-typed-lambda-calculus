#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lambda,
    Dot,
    Colon,
    Arrow,
    LParen,
    RParen,
    Var(String),
    Type(String),
}

pub fn lexer(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ch if ch.is_whitespace() => {
                chars.next();
            }
            'λ' | '\\' => {
                chars.next();
                tokens.push(Token::Lambda);
            }
            '.' => {
                chars.next();
                tokens.push(Token::Dot);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    panic!("Esperado '>' após '-'");
                }
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ch if ch.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "Bool" | "Nat" => tokens.push(Token::Type(ident)),
                    _ if ident.chars().next().unwrap().is_uppercase() => {
                        tokens.push(Token::Type(ident))
                    }
                    _ => tokens.push(Token::Var(ident)),
                }
            }
            _ => {
                panic!("Char não reconhecido: '{}'", ch);
            }
        }
    }

    tokens
}
