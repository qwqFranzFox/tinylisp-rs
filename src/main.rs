use std::io::stdin;

use crate::{data::Lisp, parser::Parser, tokenizer::Tokenizer};

mod data;
mod parser;
mod prims;
mod tokenizer;

fn main() {
    env_logger::builder().init();
    let mut lisp = Lisp::new();
    for line in include_str!("../new.lisp").lines() {
        // for token in Tokenizer::new(&line) {
        //     print!("{:?},", token);
        // }
        // println!();
        let mut p = Parser::new(Tokenizer::new(&line));
        let code = p.eval(lisp.get_context_mut());
        let result = lisp.eval(code);
        println!(">>> Result: {}", result.debug(lisp.get_context_mut()));
        // println!(
        //     ">>> env: {}",
        //     lisp.get_context_mut()
        //         .get_env()
        //         .debug(lisp.get_context_mut())
        // )
    }
    for line in stdin().lines() {
        match line {
            Ok(line) => {
                for k in Tokenizer::new(&line) {
                    println!("{:?}", k);
                }
                let tokens = Tokenizer::new(&line);
                let mut p = Parser::new(tokens);
                let code = p.eval(lisp.get_context_mut());
                let result = lisp.eval(code);
                println!(">>> Result: {}", result.debug(lisp.get_context_mut()));
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}
