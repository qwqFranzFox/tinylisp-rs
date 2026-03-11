use crate::{
    parser::Parser,
    tokenizer::Tokenizer,
    types::{Data, ENV},
};

mod parser;
mod prims;
mod tokenizer;
mod types;

fn main() {
    env_logger::init();
    let tru = Data::atom(&"#t".to_string());
    {
        let mut env = ENV.lock().unwrap();
        *env = Data::pair(tru.clone(), tru.clone(), Data::nil());
    };
    for line in include_str!("../new.lisp").lines() {
        let env = { ENV.lock().unwrap().clone() };
        let mut p = Parser::new(Tokenizer::new(line));
        let code = p.eval();
        println!("running: {code}");
        println!("{}", Data::eval(code, env.clone()));
        println!("Env is : {env}");
    }
}
