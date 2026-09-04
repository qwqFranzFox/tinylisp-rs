use crate::{
    data::{Data, LispContext},
    tokenizer::{Token, Tokenizer},
};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokenizer) -> Parser {
        Parser {
            tokens: tokens.peekable(),
        }
    }
    pub fn eval(&mut self, context: &mut LispContext) -> Data {
        Self::parse(context, &mut self.tokens).unwrap_or(context.get_err())
    }
    fn parse(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        if let Some(token) = tokens.peek() {
            match token {
                Token::Atomic(sym) => {
                    if sym == "\'" {
                        Self::quote(context, tokens)
                    } else {
                        Self::atomic(context, tokens)
                    }
                }
                Token::LBrace => {
                    tokens.next();
                    Self::list(context, tokens)
                }
                Token::Error => Some(context.err()),
                _ => Self::atomic(context, tokens),
            }
        } else {
            None
        }
    }
    fn quote(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        tokens.next()?;
        let atom = context.atom(&"quote".to_string());
        let parse = Self::parse(context, tokens)?;
        let cons = context.cons(parse, context.nil());
        return Some(context.cons(atom, cons));
    }
    fn list(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        let peek = tokens.peek()?;
        match peek {
            Token::RBrace => {
                tokens.next()?;
                Some(context.nil())
            }
            Token::Atomic(sym) => {
                if sym == "." {
                    tokens.next()?;
                    let x = Self::parse(context, tokens);
                    // tokens.next();
                    x
                } else {
                    let car = Self::parse(context, tokens)?;
                    let cdr = Self::list(context, tokens)?;
                    Some(context.cons(car, cdr))
                }
            }
            Token::Error => Some(context.err()),
            _ => {
                let car = Self::parse(context, tokens)?;
                let cdr = Self::list(context, tokens)?;
                Some(context.cons(car, cdr))
            }
        }
    }
    fn atomic(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        match tokens.next()? {
            Token::Atomic(sym) => context.prim(&sym).or_else(|| Some(context.atom(&sym))),
            Token::Number(num) => Some(context.number(num)),
            _ => None,
        }
    }
}
