use crate::lex::Token;
use crate::lex::TokenIter;

#[derive(Debug, PartialEq)]
pub enum Tree {
    Arrow,
    Colon,
    Name(char),
    Branch(Vec<Tree>),
}

pub fn parse(tokens: &[Token]) -> Result<Tree, String> {
    match do_parse(tokens.iter().peekable(), None) {
        Ok((_, tree)) => Ok(tree),
        Err(s) => Err(s),
    }
}

pub fn do_parse(
    mut tokens: TokenIter,
    mut tree: Option<Tree>,
) -> Result<(TokenIter, Tree), String> {
    match tokens.next() {
        Some(token) => match token {
            Token::Name(ch) => match &mut tree {
                None => Ok(do_parse(tokens, Some(Tree::Name(*ch)))?),
                Some(Tree::Branch(branch)) => {
                    branch.push(Tree::Name(*ch));
                    Ok(do_parse(tokens, tree)?)
                }
                Some(Tree::Name(_)) | Some(Tree::Arrow) | Some(Tree::Colon) => {
                    Err(String::from("Missing parentheses"))
                }
            },
            Token::Arrow => match &mut tree {
                None => Err(String::from("Missing lambda parameter before arrow")),
                Some(Tree::Branch(branch)) => {
                    branch.push(Tree::Arrow);
                    Ok(do_parse(tokens, tree)?)
                }
                Some(Tree::Name(_)) | Some(Tree::Arrow) | Some(Tree::Colon) => {
                    Err(String::from("Missing parentheses"))
                }
            },
            Token::Colon => match &mut tree {
                None => Err(String::from("Missing name before colon")),
                Some(Tree::Branch(branch)) => {
                    branch.push(Tree::Colon);
                    Ok(do_parse(tokens, tree)?)
                }
                Some(Tree::Name(_)) | Some(Tree::Arrow) | Some(Tree::Colon) => {
                    Err(String::from("Missing parentheses"))
                }
            },
            Token::Open => match &mut tree {
                None => {
                    let (tokens, new_tree) = do_parse(tokens, Some(Tree::Branch(Vec::new())))?;
                    Ok(do_parse(tokens, Some(new_tree))?)
                }
                Some(Tree::Branch(branch)) => {
                    let (tokens, new_tree) = do_parse(tokens, Some(Tree::Branch(Vec::new())))?;
                    branch.push(new_tree);
                    Ok(do_parse(tokens, tree)?)
                }
                Some(Tree::Name(_)) | Some(Tree::Arrow) | Some(Tree::Colon) => {
                    Err(String::from("Unexpected parenthesis"))
                }
            },
            Token::Close => match tree {
                None => Err(String::from(
                    "Closing parenthesis is invalid at start of program",
                )),
                Some(tree) => Ok((tokens, tree)),
            },
        },
        None => match tree {
            None => Err(String::from("Empty program!")),
            Some(tree) => Ok((tokens, tree)),
        },
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
            if let Ok(PipelineResult::Tree(tree)) = pipeline::pipeline(Pipeline::Parse, $text) {
                assert_eq!(tree, $expected);
            } else {
                panic!();
            }
        };
    }

    macro_rules! err {
        ($text:literal, $expected:expr) => {
            if let Err(err) = pipeline::pipeline(Pipeline::Parse, $text) {
                assert_eq!(err, $expected);
            } else {
                panic!();
            }
        };
    }

    #[test]
    fn test() {
        ok!("x", Tree::Name('x'));
        ok!("()", Tree::Branch(vec![]));
        ok!("(x)", Tree::Branch(vec![Tree::Name('x')]));
        ok!(
            "(x y)",
            Tree::Branch(vec![Tree::Name('x'), Tree::Name('y')])
        );
        ok!(
            "(x -> y)",
            Tree::Branch(vec![Tree::Name('x'), Tree::Arrow, Tree::Name('y')])
        );
        err!("", String::from("Empty program!"));
        err!("->", String::from("Missing lambda parameter before arrow"));
        err!(
            "-> ->",
            String::from("Missing lambda parameter before arrow")
        );
        err!("x (", String::from("Unexpected parenthesis"));
        err!("x ->", String::from("Missing parentheses"));
        err!("x y", String::from("Missing parentheses"));
        err!(
            ")",
            String::from("Closing parenthesis is invalid at start of program")
        );
    }
}
