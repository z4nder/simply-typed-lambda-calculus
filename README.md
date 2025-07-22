# Simply Typed Lambda Calculus Interpreter

This project is an evolution of the previous [untyped lambda calculus project](https://github.com/z4nder/lambda-calculus), now with support for the Simply Typed Lambda Calculus (STLC) type system, implemented in Rust.

<h1 align="center">
  <img src="./assets/logo.png" alt="logo of project" width="500px" />
</h1>

## Features

*   **Lexer**: Tokenizes the input string into a stream of tokens.
*   **Parser**: Builds an Abstract Syntax Tree (AST) from the tokens.
*   **Type Checker**: Performs static type checking on the AST to ensure type safety.
*   **Evaluator**: Evaluates the AST to produce a result.


### Typing rules

1.If a variable x has type σ in the context Γ, then:
```
x:σ ∈ Γ
──────────────
Γ ⊢ x : σ
```

2. Constants Rules
If `c` is a constant of type `T`:
```
c is a constant of type T
──────────────────────────
Γ ⊢ c : T
```

3. Abstraction rules
If under context Γ extended with x:σ the term e has type τ, then:
```
Γ, x:σ ⊢ e : τ
────────────────────────────
Γ ⊢ (λx:σ. e) : (σ → τ)
```

4. Application Rule
If e₁ is a function from σ to τ, and e₂ is an argument of type σ:
```
Γ ⊢ e₁ : σ → τ      Γ ⊢ e₂ : σ
──────────────────────────────
         Γ ⊢ e₁ e₂ : τ
```

### Usage
Set input
```rust
fn main() {
    let input = "(λx:A -> A. x) y:A -> A".to_string();

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
```
Output example
```
Tipo: (A -> A)
Resultado: y:(A -> A)
Debug: Var(
    "y",
    Some(
        Arrow(
            Named(
                "A",
            ),
            Named(
                "A",
            ),
        ),
    ),
)
```