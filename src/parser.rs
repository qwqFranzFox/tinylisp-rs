use crate::ports::ToString;
use crate::{
    data::{BoxedData, Data},
    tokenizer::{Token, Tokenizer},
};
use core::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokenizer) -> Parser {
        Parser {
            tokens: tokens.peekable(),
        }
    }
    pub fn parse(&mut self) -> Option<BoxedData> {
        Self::parse_loop(&mut self.tokens)
    }
    fn parse_loop(tokens: &mut Peekable<Tokenizer>) -> Option<BoxedData> {
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
    fn quote(tokens: &mut Peekable<Tokenizer>) -> Option<BoxedData> {
        tokens.next()?;
        return Some(Data::cons(
            Data::atom(&"quote".to_string()),
            Data::cons(Self::parse_loop(tokens)?, Data::nil()),
        ));
    }
    fn list(tokens: &mut Peekable<Tokenizer>) -> Option<BoxedData> {
        let peek = tokens.peek()?;
        match peek {
            Token::RBrace => {
                tokens.next()?;
                Some(Data::nil())
            }
            Token::Symbol(sym) => {
                if sym == "." {
                    tokens.next()?;
                    let x = Self::parse_loop(tokens);
                    // tokens.next();
                    x
                } else {
                    let car = Self::parse_loop(tokens)?;
                    Some(Data::cons(car, Self::list(tokens)?))
                }
            }
            _ => {
                let car = Self::parse_loop(tokens)?;
                Some(Data::cons(car, Self::list(tokens)?))
            }
        }
    }
    fn atomic(tokens: &mut Peekable<Tokenizer>) -> Option<BoxedData> {
        match tokens.next()? {
            Token::Symbol(sym) => Data::prim(&sym).or_else(|| Some(Data::atom(&sym))),
            Token::Number(num) => Some(Data::number(num)),
            _ => None,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = BoxedData;
    fn next(&mut self) -> Option<Self::Item> {
        return self.parse();
    }
}
