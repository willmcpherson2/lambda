use crate::construct;
use crate::construct::Term;
use crate::define;
use crate::eval;
use crate::lex;
use crate::lex::Token;
use crate::parse;
use crate::parse::Tree;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Pipeline {
    Lex,
    Parse,
    Construct,
    Define,
    Eval,
}

#[derive(Debug, PartialEq)]
pub enum PipelineResult {
    Tokens(Vec<Token>),
    Tree(Tree),
    Term(Term),
}

pub fn pipeline(pipeline: Pipeline, input: &str) -> Result<PipelineResult, String> {
    let lexed = lex::lex(input)?;
    if let Pipeline::Lex = pipeline {
        return Ok(PipelineResult::Tokens(lexed));
    }

    let parsed = parse::parse(&lexed)?;
    if let Pipeline::Parse = pipeline {
        return Ok(PipelineResult::Tree(parsed));
    }

    let mut constructed = construct::construct(&parsed)?;
    if let Pipeline::Construct = pipeline {
        return Ok(PipelineResult::Term(constructed));
    }

    define::define(&mut constructed);
    if let Pipeline::Define = pipeline {
        return Ok(PipelineResult::Term(constructed));
    }

    eval::eval(&mut constructed)?;
    if let Pipeline::Eval = pipeline {
        return Ok(PipelineResult::Term(constructed));
    }

    Ok(PipelineResult::Term(constructed))
}
