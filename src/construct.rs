use crate::parse::Tree;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Var(char, Option<usize>),
    Lambda(char, Option<usize>, Box<Term>),
    App(Box<Term>, Box<Term>),
    Def(char, Option<usize>, Box<Term>),
}

pub fn var(name: char) -> Term {
    Term::Var(name, None)
}

pub fn lambda(name: char, body: Term) -> Term {
    Term::Lambda(name, None, Box::new(body))
}

pub fn def(name: char, term: Term) -> Term {
    Term::Def(name, None, Box::new(term))
}

pub fn app(func: Term, arg: Term) -> Term {
    Term::App(Box::new(func), Box::new(arg))
}

#[cfg(test)]
pub fn var_id(name: char, id: usize) -> Term {
    Term::Var(name, Some(id))
}

#[cfg(test)]
pub fn lambda_id(name: char, id: usize, body: Term) -> Term {
    Term::Lambda(name, Some(id), Box::new(body))
}

pub fn construct(tree: &Tree) -> Result<Term, String> {
    match tree {
        Tree::Name(ch) => Ok(var(*ch)),
        Tree::Branch(branch) => Ok(construct_branch(branch)?),
        Tree::Arrow | Tree::Colon => panic!(),
    }
}

fn construct_branch(branch: &[Tree]) -> Result<Term, String> {
    let mut branch = branch.iter();
    let node = if let Some(node) = branch.next() {
        node
    } else {
        return Err(String::from("Empty parentheses is invalid"));
    };
    match node {
        Tree::Name(ch) => {
            let node2 = branch.next();
            match node2 {
                None => Err(String::from("Expected more symbols after name")),
                Some(Tree::Arrow) => {
                    if let Some(tree) = branch.next() {
                        let term = lambda(*ch, construct(tree)?);
                        if branch.next().is_some() {
                            Err(String::from("Lambda body has too many terms"))
                        } else {
                            Ok(term)
                        }
                    } else {
                        Err(String::from("Expected lambda body after arrow"))
                    }
                }
                Some(Tree::Colon) => {
                    if let Some(tree) = branch.next() {
                        let term = def(*ch, construct(tree)?);
                        if branch.next().is_some() {
                            Err(String::from("Definition has too many terms"))
                        } else {
                            Ok(term)
                        }
                    } else {
                        Err(String::from("Expected term after colon"))
                    }
                }
                Some(Tree::Name(ch2)) => {
                    if branch.next().is_some() {
                        Err(String::from("Application has too many terms"))
                    } else {
                        Ok(app(var(*ch), var(*ch2)))
                    }
                }
                Some(Tree::Branch(_)) => {
                    if branch.next().is_some() {
                        Err(String::from("Application has too many terms"))
                    } else {
                        Ok(app(var(*ch), construct(node2.unwrap())?))
                    }
                }
            }
        }
        Tree::Branch(_) => {
            let func = construct(node)?;
            let arg = if let Some(node) = branch.next() {
                construct(node)?
            } else {
                return Err(String::from("Expected application argument"));
            };
            Ok(app(func, arg))
        }
        Tree::Arrow => Err(String::from("Unexpected arrow")),
        Tree::Colon => Err(String::from("Unexpected colon")),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pipeline;
    use crate::pipeline::Pipeline;
    use crate::pipeline::PipelineResult;

    macro_rules! ok {
        ($text:literal, $expected:expr) => {
            if let Ok(PipelineResult::Term(term)) = pipeline::pipeline(Pipeline::Construct, $text) {
                assert_eq!(term, $expected);
            } else {
                panic!();
            }
        };
    }

    macro_rules! err {
        ($text:literal, $expected:expr) => {
            if let Err(err) = pipeline::pipeline(Pipeline::Construct, $text) {
                assert_eq!(err, $expected);
            } else {
                panic!();
            }
        };
    }

    #[test]
    fn test() {
        ok!("x", var('x'));
        ok!("(f x)", app(var('f'), var('x')));
        ok!("(f (g x))", app(var('f'), app(var('g'), var('x'))));
        ok!("(x -> x)", lambda('x', var('x')));
        ok!("((x -> x) x)", app(lambda('x', var('x')), var('x')));
        ok!("(x -> (y -> z))", lambda('x', lambda('y', var('z'))));
        ok!(
            "((a -> b) (c -> d))",
            app(lambda('a', var('b')), lambda('c', var('d')))
        );
        ok!(
            "((a -> b) (c d))",
            app(lambda('a', var('b')), app(var('c'), var('d')))
        );
        err!("(x)", String::from("Expected more symbols after name"));
        err!("(x -> x y)", String::from("Lambda body has too many terms"));
        err!("(x ->)", String::from("Expected lambda body after arrow"));
        err!("(x y z)", String::from("Application has too many terms"));
        err!("((a -> b))", String::from("Expected application argument"));
        err!("(-> x)", String::from("Unexpected arrow"));
    }
}
