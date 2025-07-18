use crate::parser::Term;

pub fn eval(term: Term) -> Term {
    match term {
        Term::App(left, right) => {
            let left = eval(*left);
            let right = eval(*right);

            match left {
                Term::Abs(param, _ty, body) => eval(substitute(&param, &right, *body)),
                _ => Term::App(Box::new(left), Box::new(right)),
            }
        }

        Term::Abs(param, ty, body) => Term::Abs(param, ty, Box::new(eval(*body))),

        Term::Var(_) => term,
    }
}

pub fn substitute(var: &str, value: &Term, term: Term) -> Term {
    match term {
        Term::Var(name) if name == var => value.clone(),
        Term::Var(_) => term,

        Term::Abs(param, ty, body) if param == var => {
            // Variável está sombreando — não substitui ????
            Term::Abs(param, ty, body)
        }

        Term::Abs(param, ty, body) => {
            let new_body = substitute(var, value, *body);
            Term::Abs(param, ty, Box::new(new_body))
        }

        Term::App(left, right) => {
            let new_left = substitute(var, value, *left);
            let new_right = substitute(var, value, *right);
            Term::App(Box::new(new_left), Box::new(new_right))
        }
    }
}
