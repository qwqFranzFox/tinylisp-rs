use crate::{parser::Parser, tokenizer::Tokenizer, types::Data};

mod env;
mod parser;
mod prims;
mod tokenizer;
mod types;

fn main() {
    let tru = Data::atom(&"#t".to_string());
    let mut env = Data::pair(tru.clone(), tru.clone(), &mut Data::nil());
    for line in include_str!("../new.lisp").lines() {
        let mut p = Parser::new(Tokenizer::new(line));
        let k = p.eval();
        println!("{}", k);
        println!("{}", Data::eval(k, &mut env));
        println!("{}", env);
    }
}
