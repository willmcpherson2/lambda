use crate::construct::Term;

pub fn define(term: &mut Term) {
    let mut id_counter = 0;
    define_lambdas(term, &mut id_counter);
}

fn define_lambdas(term: &mut Term, id_counter: &mut usize) {
    match term {
        Term::Lambda(name, id, term) => {
            *id = Some(*id_counter);
            *id_counter += 1;
            define_body(term, *name, id.unwrap());
            define_lambdas(term, id_counter);
        }
        Term::Def(name, id, term) => {
            *id = Some(*id_counter);
            *id_counter += 1;
            define_body(term, *name, id.unwrap());
            define_lambdas(term, id_counter);
        }
        Term::App(term1, term2) => {
            define_lambdas(term1, id_counter);
            define_lambdas(term2, id_counter);
        }
        Term::Var(_, id) => {
            if id.is_none() {
                *id = Some(*id_counter);
                *id_counter += 1;
            }
        }
    }
}

fn define_body(term: &mut Term, parent_name: char, parent_id: usize) {
    match term {
        Term::Lambda(name, _, term) => {
            if *name != parent_name {
                define_body(term, parent_name, parent_id)
            }
        }
        Term::Def(_, _, _) => {
            panic!();
        }
        Term::App(term1, term2) => {
            define_body(term1, parent_name, parent_id);
            define_body(term2, parent_name, parent_id);
        }
        Term::Var(name, id) => {
            if id.is_none() && *name == parent_name {
                *id = Some(parent_id);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::construct::app;
    use crate::construct::lambda_id;
    use crate::construct::var_id;
    use crate::pipeline;
    use crate::pipeline::Pipeline;
    use crate::pipeline::PipelineResult;

    macro_rules! run {
        ($text:literal, $expected:expr) => {
            if let Ok(PipelineResult::Term(term)) = pipeline::pipeline(Pipeline::Define, $text) {
                assert_eq!(term, $expected);
            } else {
                panic!();
            }
        };
    }

    #[test]
    fn test() {
        run!("x", var_id('x', 0));
        run!("(x -> x)", lambda_id('x', 0, var_id('x', 0)));
        run!(
            "((x -> x) (x -> x))",
            app(
                lambda_id('x', 0, var_id('x', 0)),
                lambda_id('x', 1, var_id('x', 1))
            )
        );
        run!(
            "((x -> x) (y -> y))",
            app(
                lambda_id('x', 0, var_id('x', 0)),
                lambda_id('y', 1, var_id('y', 1))
            )
        );
        run!("(f x)", app(var_id('f', 0), var_id('x', 1)));
        run!(
            "(f (g x))",
            app(var_id('f', 0), app(var_id('g', 1), var_id('x', 2)))
        );
        run!(
            "((x -> (y -> x)) (y -> y))",
            app(
                lambda_id('x', 0, lambda_id('y', 1, var_id('x', 0))),
                lambda_id('y', 2, var_id('y', 2))
            )
        );
        run!(
            "((f -> (x -> (f x))) (y -> (x -> y)))",
            app(
                lambda_id(
                    'f',
                    0,
                    lambda_id('x', 1, app(var_id('f', 0), var_id('x', 1)))
                ),
                lambda_id('y', 2, lambda_id('x', 3, var_id('y', 2)))
            )
        );
        run!(
            "((x -> (x x)) (x -> (x x)))",
            app(
                lambda_id('x', 0, app(var_id('x', 0), var_id('x', 0))),
                lambda_id('x', 1, app(var_id('x', 1), var_id('x', 1)))
            )
        );
    }
}
