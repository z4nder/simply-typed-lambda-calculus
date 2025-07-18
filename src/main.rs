mod eval;
mod lexer;
mod parser;

use crate::parser::Parser;
use eval::eval;
use lexer::lexer;

fn main() {
    let input = "λx:A. x".to_string();
    let tokens = lexer(input);

    let mut parser = Parser::new(tokens);
    let term = parser.parse_term();

    println!("Debug: {:#?}", term);
    println!("Formatted: {}", term);

    let result = eval(term);

    println!("Resultado: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Var
    #[test]
    fn test_should_be_eval_var() {
        let input = "x".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "x");
    }

    // Abs without type
    #[test]
    fn test_should_be_eval_abstractions_without_type() {
        let input = "λx. x".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "λx.(x)");
    }

    // Abs with type
    #[test]
    fn test_should_be_eval_abstractions_with_type() {
        let input = "λx:Y. x".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "λx:Y.(x)");
    }

    // Abs with type arrow
    #[test]
    fn test_should_be_eval_abstractions_with_type_arrow() {
        let input = "λx:Y -> Y. x".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result: parser::Term = eval(term);

        assert_eq!(format!("{}", result), "λx:(Y -> Y).(x)");
    }

    // App without type
    #[test]
    fn test_should_be_eval_appplication_without_type() {
        let input = "(λx. x) y".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "y");
    }

    // App with type
    #[test]
    fn test_should_be_eval_appplication_with_type() {
        let input = "(λx:A. x) y".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "y");
    }

    // App with type arrow
    #[test]
    fn test_should_be_eval_appplication_with_type_arrow() {
        let input = "(λx:A -> A. x) y".to_string();
        let tokens = lexer(input);

        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();

        let result = eval(term);

        assert_eq!(format!("{}", result), "y");
    }
}
