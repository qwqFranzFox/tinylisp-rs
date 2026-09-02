use crate::{
    data::{Data, Lisp, LispContext},
    ports::ToString,
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
    pub fn eval(&mut self, context: &mut LispContext) -> Data {
        Self::parse(context, &mut self.tokens).unwrap_or(context.err())
    }
    fn parse(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        if let Some(token) = tokens.peek() {
            match token {
                Token::Symbol(sym) => {
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
            Token::Symbol(sym) => {
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
            _ => {
                let car = Self::parse(context, tokens)?;
                let cdr = Self::list(context, tokens)?;
                Some(context.cons(car, cdr))
            }
        }
    }
    fn atomic(context: &mut LispContext, tokens: &mut Peekable<Tokenizer>) -> Option<Data> {
        match tokens.next()? {
            Token::Symbol(sym) => context.prim(&sym).or_else(|| Some(context.atom(&sym))),
            Token::Number(num) => Some(context.number(num)),
            _ => None,
        }
    }

    pub fn chain_eval(self, lisp: &'a mut Lisp) -> ParserIter<'a> {
        ParserIter {
            parser: self,
            lisp,
            err: false,
        }
    }
}

pub struct ParserIter<'a> {
    parser: Parser<'a>,
    lisp: &'a mut Lisp,
    err: bool,
}

impl<'a> Iterator for ParserIter<'a> {
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        if self.err {
            return None;
        } else {
            let code = self.parser.eval(self.lisp.get_context_mut());
            if code != self.lisp.get_context_mut().err() {
                let result = self.lisp.eval(code);
                if result != self.lisp.get_context_mut().err() {
                    return Some(result);
                }
            }
        }
        self.err = true;
        Some(self.lisp.get_context_mut().err())
    }
}
