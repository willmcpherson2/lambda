#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Name(char),
    Arrow,
    Colon,
    Open,
    Close,
}

pub type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    enum State {
        Any,
        Arrow,
    }

    let mut tokens = Vec::new();
    let mut state = State::Any;

    for ch in input.chars() {
        match state {
            State::Any => match ch {
                ' ' | '\n' => (),
                'a'..='z' => tokens.push(Token::Name(ch)),
                '(' => tokens.push(Token::Open),
                ')' => tokens.push(Token::Close),
                ':' => tokens.push(Token::Colon),
                '-' => state = State::Arrow,
                '>' => return Err(String::from("'>' must be preceded by '-'")),
                _ => return Err(format!("'{}' is never a valid character", ch)),
            },
            State::Arrow => match ch {
                '>' => {
                    tokens.push(Token::Arrow);
                    state = State::Any;
                }
                _ => return Err(String::from("'-' must be followed by '>'")),
            },
        }
    }

    if let State::Arrow = state {
        Err(String::from("'-' must be followed by '>'"))
    } else {
        Ok(tokens)
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
            if let Ok(PipelineResult::Tokens(tokens)) = pipeline::pipeline(Pipeline::Lex, $text) {
                assert_eq!(tokens, $expected);
            } else {
                panic!();
            }
        };
    }

    macro_rules! err {
        ($text:literal, $expected:expr) => {
            if let Err(err) = pipeline::pipeline(Pipeline::Lex, $text) {
                assert_eq!(err, $expected);
            } else {
                panic!();
            }
        };
    }

    #[test]
    fn test() {
        ok!("", vec![]);
        ok!("x", vec![Token::Name('x')]);
        ok!("x y", vec![Token::Name('x'), Token::Name('y')]);
        ok!("xy", vec![Token::Name('x'), Token::Name('y')]);
        ok!("()", vec![Token::Open, Token::Close]);
        ok!(
            "(())",
            vec![Token::Open, Token::Open, Token::Close, Token::Close]
        );
        ok!("(x)", vec![Token::Open, Token::Name('x'), Token::Close]);
        ok!(
            "(xy)",
            vec![
                Token::Open,
                Token::Name('x'),
                Token::Name('y'),
                Token::Close
            ]
        );
        ok!("->", vec![Token::Arrow]);
        ok!(
            "x -> y",
            vec![Token::Name('x'), Token::Arrow, Token::Name('y')]
        );
        ok!(
            "(x -> y)",
            vec![
                Token::Open,
                Token::Name('x'),
                Token::Arrow,
                Token::Name('y'),
                Token::Close
            ]
        );
        ok!(
            "(x->y)",
            vec![
                Token::Open,
                Token::Name('x'),
                Token::Arrow,
                Token::Name('y'),
                Token::Close
            ]
        );
        err!("A", String::from("'A' is never a valid character"));
        err!(">-", String::from("'>' must be preceded by '-'"));
        err!("-", String::from("'-' must be followed by '>'"));
        err!("-a", String::from("'-' must be followed by '>'"));
    }
}
