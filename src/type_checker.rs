use std::collections::HashMap;

use crate::parser::{Term, Type};
pub type Context = HashMap<String, Type>; // Context -> Γ HIIHIIH 

/// * # TODO
///  * Talvez eu não deveria aceitar x:Y poiss isso sai do lambda
///  * Justamente deria ter que declara o Context 'manualmente' montar meu input e manter as Vars lambda possibilidade de Type
///
pub fn type_check(term: &Term, ctx: &Context) -> Result<Type, String> {
    match term {
        /*
         1.Var rules: If a variable x has type σ in the context Γ
            x:σ ∈ Γ
            ──────────────
            Γ ⊢ x : σ
        */
        Term::Var(name, maybe_type) => {
            if let Some(ty) = ctx.get(name) {
                Ok(ty.clone())
            } else if let Some(ty) = maybe_type {
                Ok(ty.clone())
            } else {
                Err(format!("Variável não tipada: {}", name))
            }
        }

        /* TODO Implementar
         2.Constants Rules: If `c` is a constant of type `T`:
            c is a constant of type T
            ──────────────────────────
            Γ ⊢ c : T
            Ex: se c é true:Bool deve ser Bool...
        */


        /*
         3.Abstraction rules: If under context Γ extended with x:σ the term e has type τ, then:
            Γ, x:σ ⊢ e : τ
            ──────────────────────────
            Γ ⊢ (λx:σ. e) : (σ → τ)
            Ex: Se x tem tipo Bool no contexto, então λx:Bool. x tem tipo Bool → Bool
        */
        Term::Abs(param, Some(param_type), body) => {
            let mut new_ctx = ctx.clone();
            new_ctx.insert(param.clone(), param_type.clone());

            let body_type = type_check(body, &new_ctx)?;
            Ok(Type::Arrow(
                Box::new(param_type.clone()),
                Box::new(body_type),
            ))
        }

        Term::Abs(_, None, _) => {
            Err("Não é possível inferir o tipo de uma abstração sem anotação.".to_string())
        }

        /*
            4. Application Rule If e₁ is a function from σ to τ, and e₂ is an argument of type σ:
                Γ ⊢ e₁ : σ → τ      Γ ⊢ e₂ : σ
                ──────────────────────────
                Γ ⊢ e₁ e₂ : τ
        */
        Term::App(t1, t2) => {
            let t1_type = type_check(t1, ctx)?;
            let t2_type = type_check(t2, ctx)?;

            match t1_type {
                Type::Arrow(param_type, return_type) => {
                    if *param_type == t2_type {
                        Ok(*return_type)
                    } else {
                        Err(format!(
                            "Tipo do argumento não corresponde ao tipo esperado. Esperado: {:?}, encontrado: {:?}",
                            param_type, t2_type
                        ))
                    }
                }
                other => Err(format!(
                    "Tentando aplicar um termo que não é função: {:?}",
                    other
                )),
            }
        }
    }
}
