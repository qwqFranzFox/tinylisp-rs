use crate::data::IntType;
use crate::ports::{String, Vec, vec};
use core::{iter::Peekable, str::Chars};

struct FilterComment<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> FilterComment<'a> {
    pub fn new(iter: Chars) -> FilterComment {
        FilterComment {
            iter: iter.peekable(),
        }
    }
}

impl<'a> Iterator for FilterComment<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        while *self.iter.peek()? == ';' {
            while *self.iter.peek()? != '\n' {
                self.iter.next()?;
            }
        }
        self.iter.next()
    }
}

pub struct Tokenizer<'a> {
    iter: Peekable<FilterComment<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'_ str) -> Tokenizer<'_> {
        Tokenizer {
            iter: FilterComment::new(code.chars()).peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while self.iter.peek()?.is_ascii_whitespace() {
            self.iter.next()?;
        }
        let k = Token::match_lbrace(&mut self.iter)
            .or_else(|| Token::match_rbrace(&mut self.iter))
            .or_else(|| Token::match_number(&mut self.iter))
            .or_else(|| Token::match_symbol(&mut self.iter));
        k
    }
}

#[derive(Debug)]
pub enum Token {
    Symbol(String),
    Number(IntType),
    LBrace,
    RBrace,
}

impl Token {
    fn match_symbol(code: &mut Peekable<FilterComment>) -> Option<Token> {
        let mut cache: Vec<char> = vec![];
        loop {
            if let Some(&peek) = code.peek() {
                if peek.is_whitespace() || peek == '(' || peek == ')' {
                    if cache.is_empty() {
                        break None;
                    } else {
                        break Some(Token::Symbol(cache.iter().collect()));
                    }
                } else {
                    cache.push(code.next()?);
                }
            }
        }
    }
    fn match_number(code: &mut Peekable<FilterComment>) -> Option<Token> {
        let mut cache: Vec<char> = vec![];
        loop {
            let &peek = code.peek()?;
            if !peek.is_numeric() {
                if cache.is_empty() {
                    break None;
                } else {
                    break Some(Token::Number(
                        cache.iter().collect::<String>().parse().unwrap(),
                    ));
                }
            } else {
                cache.push(code.next()?);
            }
        }
    }
    fn match_lbrace(code: &mut Peekable<FilterComment>) -> Option<Token> {
        let &peek = code.peek()?;
        if peek == '(' {
            code.next()?;
            Some(Token::LBrace)
        } else {
            None
        }
    }
    fn match_rbrace(code: &mut Peekable<FilterComment>) -> Option<Token> {
        let &peek = code.peek()?;
        if peek == ')' {
            code.next()?;
            Some(Token::RBrace)
        } else {
            None
        }
    }
}
