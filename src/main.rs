use crate::{data::Lisp, parser::Parser, tokenizer::Tokenizer};

mod data;
mod parser;
mod prims_new;
mod tokenizer;

fn main() {
    env_logger::builder().init();
    let mut lisp = Lisp::new();
    for line in include_str!("../new.lisp").lines() {
        let mut p = Parser::new(Tokenizer::new(line));
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
}
