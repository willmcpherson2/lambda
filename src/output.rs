use crate::construct::Term;
use std::collections::HashMap;
use std::fmt;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        do_fmt(self, f, &mut HashMap::new())
    }
}

fn do_fmt(
    term: &Term,
    f: &mut fmt::Formatter<'_>,
    scope: &mut HashMap<char, usize>,
) -> fmt::Result {
    match term {
        Term::Var(name, id) => {
            if let Some(parent_id) = scope.get(name) {
                if id.unwrap() != *parent_id {
                    return write!(f, "{}.{}", name, id.unwrap());
                }
            }
            write!(f, "{}", name)
        }
        Term::Lambda(name, id, term) => {
            if scope.get(name).is_some() {
                write!(f, "({}.{} -> ", name, id.unwrap())?;
                do_fmt(term, f, scope)?;
            } else {
                scope.insert(*name, id.unwrap());
                write!(f, "({} -> ", name)?;
                do_fmt(term, f, scope)?;
                scope.remove(name);
            }
            write!(f, ")")
        }
        Term::App(term1, term2) => {
            write!(f, "(")?;
            do_fmt(term1, f, scope)?;
            write!(f, " ")?;
            do_fmt(term2, f, scope)?;
            write!(f, ")")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pipeline;
    use crate::pipeline::Pipeline;
    use crate::pipeline::PipelineResult;

    macro_rules! run {
        ($text:literal, $expected:literal) => {
            if let Ok(PipelineResult::Term(term)) = pipeline::pipeline(Pipeline::Eval, $text) {
                assert_eq!(format!("{}", term), $expected);
            } else {
                panic!();
            }
        };
    }

    #[test]
    fn test() {
        // 3 = 3
        run!("(f -> (x -> (f (f (f x)))))", "(f -> (x -> (f (f (f x)))))");

        // increment = increment
        run!(
            "(n -> (f -> (x -> (f ((n f) x)))))",
            "(n -> (f -> (x -> (f ((n f) x)))))"
        );

        // (increment 3) = 4
        run!(
            "((n -> (f -> (x -> (f ((n f) x))))) (f -> (x -> (f (f (f x))))))",
            "(f -> (x -> (f (f (f (f x))))))"
        );

        // (plus 1 1) = 2
        run!(
            "(((m -> (n -> (f -> (x -> ((m f) ((n f) x)))))) (f -> (x -> (f x)))) (f -> (x -> (f x))))",
            "(f -> (x -> (f (f x))))"
        );

        // true = true
        run!("(x -> (y -> x))", "(x -> (y -> x))");

        // false = false
        run!("(x -> (y -> y))", "(x -> (y -> y))");

        // and = and
        run!("(p -> (q -> ((p q) p)))", "(p -> (q -> ((p q) p)))");

        // (and true true) = true
        run!(
            "(((p -> (q -> ((p q) p))) (x -> (y -> x))) (x -> (y -> x)))",
            "(x -> (y -> x))"
        );

        // (and true false) = false
        run!(
            "(((p -> (q -> ((p q) p))) (x -> (y -> x))) (x -> (y -> y)))",
            "(x -> (y -> y))"
        );

        // or = or
        run!("(p -> (q -> ((p p) q)))", "(p -> (q -> ((p p) q)))");

        // (or false true) = true
        run!(
            "(((p -> (q -> ((p p) q))) (x -> (y -> y))) (x -> (y -> x)))",
            "(x -> (y -> x))"
        );

        // not = not
        run!(
            "(p -> ((p (x -> (y -> y))) (x -> (y -> x))))",
            "(p -> ((p (x -> (y -> y))) (x -> (y -> x))))"
        );

        // (not true) = false
        run!(
            "((p -> ((p (x -> (y -> y))) (x -> (y -> x)))) (x -> (y -> x)))",
            "(x -> (y -> y))"
        );

        // (not false) = true
        run!(
            "((p -> ((p (x -> (y -> y))) (x -> (y -> x)))) (x -> (y -> y)))",
            "(x -> (y -> x))"
        );

        // ifThenElse = ifThenElse
        run!(
            "(p -> (a -> (b -> ((p a) b))))",
            "(p -> (a -> (b -> ((p a) b))))"
        );

        // (ifThenElse true 3 0) = 3
        run!(
            "((((p -> (a -> (b -> ((p a) b)))) (x -> (y -> x))) (f -> (x -> (f (f (f x)))))) (f -> (x -> x)))",
            "(f -> (x -> (f (f (f x)))))"
        );

        // (ifThenElse false 3 0) = 3
        run!(
            "((((p -> (a -> (b -> ((p a) b)))) (x -> (y -> y))) (f -> (x -> (f (f (f x)))))) (f -> (x -> x)))",
            "(f -> (x -> x))"
        );

        //isZero = isZero
        run!(
            "(n -> ((n (p -> (x -> (y -> y)))) (x -> (y -> x))))",
            "(n -> ((n (p -> (x -> (y -> y)))) (x -> (y -> x))))"
        );

        // (isZero 1) = false
        run!(
            "((n -> ((n (p -> (x -> (y -> y)))) (x -> (y -> x)))) (f -> (x -> (f x))))",
            "(x -> (y -> y))"
        );

        // (isZero 0) = true
        run!(
            "((n -> ((n (p -> (x -> (y -> y)))) (x -> (y -> x)))) (f -> (x -> x)))",
            "(x -> (y -> x))"
        );

        // (pair a b) = (pair a b)
        run!("(p -> ((p a) b))", "(p -> ((p a) b))");

        // makePair = makePair
        run!(
            "(a -> (b -> (p -> ((p a) b))))",
            "(a -> (b -> (p -> ((p a) b))))"
        );

        // (makePair 0 1) = (pair 0 1)
        run!(
            "(((a -> (b -> (p -> ((p a) b)))) (f -> (x -> x))) (f -> (x -> (f x))))",
            "(p -> ((p (f -> (x -> x))) (f -> (x -> (f x)))))"
        );

        // (makePair false true) = (pair false true)
        run!(
            "(((a -> (b -> (p -> ((p a) b)))) (x -> (y -> y))) (x -> (y -> x)))",
            "(p -> ((p (x -> (y -> y))) (x -> (y -> x))))"
        );

        // fst = fst
        run!("(p -> (p (x -> (y -> x))))", "(p -> (p (x -> (y -> x))))");

        // snd = snd
        run!("(p -> (p (x -> (y -> y))))", "(p -> (p (x -> (y -> y))))");

        // (fst (pair a b)) = a
        run!("((p -> (p (x -> (y -> x)))) (p -> ((p a) b)))", "a");

        // (snd (pair a b)) = b
        run!("((p -> (p (x -> (y -> y)))) (p -> ((p a) b)))", "b");
    }
}
