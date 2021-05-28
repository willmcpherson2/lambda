mod construct;
mod define;
mod eval;
mod lex;
mod output;
mod parse;
mod pipeline;

use pipeline::pipeline;
use pipeline::Pipeline;
use pipeline::PipelineResult;
use std::io;
use std::io::BufRead;

fn main() {
    if let Some(input) = std::env::args().nth(1) {
        run(&input);
    } else {
        let stdin = io::stdin();
        for input in stdin.lock().lines() {
            run(&input.unwrap());
        }
    }
}

fn run(input: &str) {
    match pipeline(Pipeline::Eval, &input) {
        Ok(PipelineResult::Term(result)) => println!("{}", result),
        Err(err) => println!("{}", err),
        _ => panic!(),
    }
}
