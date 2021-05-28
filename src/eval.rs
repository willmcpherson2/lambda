use crate::construct::Term;

const RECURSION_LIMIT: usize = 1000;

pub fn eval(term: &mut Term) -> Result<(), String> {
    loop {
        let mut modified = false;
        let mut recursion = 0;
        do_eval(term, &mut modified, &mut recursion)?;
        if !modified {
            return Ok(());
        }
    }
}

fn do_eval(term: &mut Term, modified: &mut bool, recursion: &mut usize) -> Result<(), String> {
    bump_recursion_count(recursion)?;
    match term {
        Term::App(func, ref mut arg) => {
            if let Term::Lambda(_, id, ref mut body) = **func {
                substitute(id.unwrap(), body, arg, modified, recursion)?;
                *term = *body.clone();
                *modified = true;
            } else if let Term::App { .. } = **func {
                do_eval(func, modified, recursion)?
            } else if let Term::App { .. } = **arg {
                do_eval(arg, modified, recursion)?
            } else if let Term::Lambda { .. } = **arg {
                do_eval(arg, modified, recursion)?
            }
            Ok(())
        }
        Term::Lambda(_, _, body) => do_eval(body, modified, recursion),
        Term::Var { .. } => Ok(()),
    }
}

fn substitute(
    id: usize,
    body: &mut Term,
    arg: &mut Term,
    modified: &mut bool,
    recursion: &mut usize,
) -> Result<(), String> {
    bump_recursion_count(recursion)?;
    match body {
        Term::App(func, app_arg) => {
            substitute(id, func, arg, modified, recursion)?;
            substitute(id, app_arg, arg, modified, recursion)
        }
        Term::Lambda(_, _, ref mut body) => substitute(id, body, arg, modified, recursion),
        Term::Var(_, child_id) => {
            if child_id.unwrap() == id {
                *body = arg.clone();
                *modified = true;
            }
            Ok(())
        }
    }
}

fn bump_recursion_count(recursion: &mut usize) -> Result<(), String> {
    *recursion += 1;
    if *recursion >= RECURSION_LIMIT {
        Err(String::from("Hit recursion limit"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::construct::app;
    use crate::construct::lambda_id;
    use crate::construct::var_id;
    use crate::pipeline;
    use crate::pipeline::Pipeline;
    use crate::pipeline::PipelineResult;

    macro_rules! run {
        ($text:literal, $expected:expr) => {
            if let Ok(PipelineResult::Term(term)) = pipeline::pipeline(Pipeline::Eval, $text) {
                assert_eq!(term, $expected);
            } else {
                panic!();
            }
        };
    }

    macro_rules! recursive {
        ($text:literal, $expected:expr) => {
            if let Ok(PipelineResult::Term(mut term)) = pipeline::pipeline(Pipeline::Define, $text)
            {
                do_eval(&mut term, &mut false, &mut 0).unwrap();
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
        run!("(f x)", app(var_id('f', 0), var_id('x', 1)));
        run!(
            "(f (g x))",
            app(var_id('f', 0), app(var_id('g', 1), var_id('x', 2)))
        );
        run!("((x -> x) y)", var_id('y', 1));
        run!("((x -> y) z)", var_id('y', 1));
        run!("((x -> (y -> x)) z)", lambda_id('y', 1, var_id('z', 2)));
        run!("((a -> ((b -> c) d)) e)", var_id('c', 2));
        run!("((x -> x) (y -> z))", lambda_id('y', 1, var_id('z', 2)));
        run!("((a -> a) ((b -> c) d))", var_id('c', 2));
        run!("(f ((x -> x) y))", app(var_id('f', 0), var_id('y', 2)));
        run!(
            "(f (g ((x -> x) x)))",
            app(var_id('f', 0), app(var_id('g', 1), var_id('x', 3)))
        );
        run!("((f -> (f x)) (y -> y))", var_id('x', 1));
        run!(
            "((x -> (y -> x)) (a -> a))",
            lambda_id('y', 1, lambda_id('a', 2, var_id('a', 2)))
        );
        run!(
            "((y -> (a -> a)) ((x -> (x x)) (x -> (x x))))",
            lambda_id('a', 1, var_id('a', 1))
        );
        run!(
            "(((x -> (y -> x)) (a -> a)) ((x -> (x x)) (x -> (x x))))",
            lambda_id('a', 2, var_id('a', 2))
        );
        recursive!(
            "((x -> (x x)) (x -> (x x)))",
            app(
                lambda_id('x', 1, app(var_id('x', 1), var_id('x', 1))),
                lambda_id('x', 1, app(var_id('x', 1), var_id('x', 1)))
            )
        );
    }
}
