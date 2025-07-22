mod eval;
mod lexer;
mod parser;
mod type_checker;

use crate::{
    parser::{Parser, Term, Type},
    type_checker::{Context, type_check},
};
use eval::eval;
use lexer::lexer;

fn main() {
    let input = "(λx:Bool. x) true".to_string();

    match process_input(input) {
        Ok((term, type_option)) => {
            if let Some(ty) = type_option {
                println!("Tipo: {}", ty);
            } else {
                println!("Tipo: não verificado");
            }
            println!("Resultado: {}", term);
            println!("Debug: {:#?}", term);
        }
        Err(err) => eprintln!("Erro: {}", err),
    }
}

fn process_input(input: String) -> Result<(Term, Option<Type>), String> {
    let tokens = lexer(input.to_string());
    let mut parser = Parser::new(tokens);
    let term = parser.parse_term();

    let needs_type_check = input.contains(":") || input.contains("->");

    let ty = if needs_type_check {
        let ctx = Context::new();
        Some(type_check(&term, &ctx)?)
    } else {
        None
    };

    let evaluated = eval(term);
    Ok((evaluated, ty))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Var without type
    #[test]
    fn test_should_be_eval_var_without_type() {
        let input = "x".to_string();
        let (term, _) = process_input(input).expect("Erro ao processar input");

        assert_eq!(format!("{}", term), "x");
    }

    // Var with type
    #[test]
    fn test_should_be_eval_var_with_type() {
        let input = "x:Y".to_string();
        let (term, option_type) = process_input(input).expect("Erro ao processar input");
        let ty = option_type.expect("Erro no type check");

        assert_eq!(format!("{}", term), "x:Y");
        assert_eq!(format!("{}", ty), "Y");
    }

    // Abs without type
    #[test]
    fn test_should_be_eval_abstractions_without_type() {
        let input = "λx. x".to_string();
        let (term, _) = process_input(input).expect("Erro ao processar input");

        assert_eq!(format!("{}", term), "λx.(x)");
    }

    // Abs with type
    #[test]
    fn test_should_be_eval_abstractions_with_type() {
        let input = "λx:Y. x".to_string();
        let (term, option_type) = process_input(input).expect("Erro ao processar input");
        let ty = option_type.expect("Erro no type check");

        assert_eq!(format!("{}", term), "λx:Y.(x)");
        assert_eq!(format!("{}", ty), "(Y -> Y)");
    }

    // Abs with type arrow
    #[test]
    fn test_should_be_eval_abstractions_with_type_arrow() {
        let input = "λx:Y -> Y. x".to_string();
        let (term, option_type) = process_input(input).expect("Erro ao processar input");
        let ty = option_type.expect("Erro no type check");

        assert_eq!(format!("{}", term), "λx:(Y -> Y).(x)");
        assert_eq!(format!("{}", ty), "((Y -> Y) -> (Y -> Y))");
    }

    // App without type
    #[test]
    fn test_should_be_eval_appplication_without_type() {
        let input = "(λx. x) y".to_string();
        let (term, _) = process_input(input).expect("Erro ao processar input");

        assert_eq!(format!("{}", term), "y");
    }

    // App with type
    #[test]
    fn test_should_be_eval_appplication_with_type() {
        let input = "(λx:A. x) y:A".to_string();
        let (term, option_type) = process_input(input).expect("Erro ao processar input");
        let ty = option_type.expect("Erro no type check");

        assert_eq!(format!("{}", term), "y:A");
        assert_eq!(format!("{}", ty), "A");
    }

    // App with type arrow
    #[test]
    fn test_should_be_eval_appplication_with_type_arrow() {
        let input = "(λx:A -> A. x) y:A -> A".to_string();
        let (term, option_type) = process_input(input).expect("Erro ao processar input");
        let ty = option_type.expect("Erro no type check");

        assert_eq!(format!("{}", term), "y:(A -> A)");
        assert_eq!(format!("{}", ty), "(A -> A)");
    }
}
