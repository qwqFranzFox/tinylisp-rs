use std::iter::Peekable;

use crate::{
    tokenizer::{Token, Tokenizer},
    types::Data,
};
use std::sync::Arc;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokenizer) -> Parser {
        Parser {
            tokens: tokens.peekable(),
        }
    }
    pub fn eval(&mut self) -> Arc<Data> {
        Self::parse(&mut self.tokens).unwrap_or(Data::err())
    }
    fn parse(tokens: &mut Peekable<Tokenizer>) -> Option<Arc<Data>> {
        if let Some(token) = tokens.peek() {
            match token {
                Token::Symbol(sym) => {
                    if sym == "\'" {
                        Self::quote(tokens)
                    } else {
                        Self::atomic(tokens)
                    }
                }
                Token::LBrace => {
                    tokens.next();
                    Self::list(tokens)
                }
                _ => Self::atomic(tokens),
            }
        } else {
            None
        }
    }
    fn quote(tokens: &mut Peekable<Tokenizer>) -> Option<Arc<Data>> {
        tokens.next()?;
        return Some(Data::cons(
            Data::atom(&"quote".to_string()),
            Data::cons(Self::parse(tokens)?, Data::nil()),
        ));
    }
    fn list(tokens: &mut Peekable<Tokenizer>) -> Option<Arc<Data>> {
        let peek = tokens.peek()?;
        match peek {
            Token::RBrace => {
                tokens.next()?;
                Some(Data::nil())
            }
            Token::Symbol(sym) => {
                if sym == "." {
                    tokens.next()?;
                    let x = Self::parse(tokens);
                    // tokens.next();
                    x
                } else {
                    let car = Self::parse(tokens)?;
                    Some(Data::cons(car, Self::list(tokens)?))
                }
            }
            _ => {
                let car = Self::parse(tokens)?;
                Some(Data::cons(car, Self::list(tokens)?))
            }
        }
    }
    fn atomic(tokens: &mut Peekable<Tokenizer>) -> Option<Arc<Data>> {
        match tokens.next()? {
            Token::Symbol(sym) => Data::prim(&sym).or_else(|| Some(Data::atom(&sym))),
            Token::Number(num) => Some(Data::number(num)),
            _ => None,
        }
    }
}
