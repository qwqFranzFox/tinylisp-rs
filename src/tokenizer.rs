use std::{iter::Peekable, str::Chars, vec};

pub struct Tokenizer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'_ str) -> Tokenizer<'_> {
        Tokenizer {
            iter: code.chars().peekable(),
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
    Number(usize),
    LBrace,
    RBrace,
}

impl Token {
    pub fn match_symbol(code: &mut Peekable<Chars>) -> Option<Token> {
        let mut cache: Vec<char> = vec![];
        loop {
            let &peek = code.peek()?;
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
    pub fn match_number(code: &mut Peekable<Chars>) -> Option<Token> {
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
    pub fn match_lbrace(code: &mut Peekable<Chars>) -> Option<Token> {
        let &peek = code.peek()?;
        if peek == '(' {
            code.next()?;
            Some(Token::LBrace)
        } else {
            None
        }
    }
    pub fn match_rbrace(code: &mut Peekable<Chars>) -> Option<Token> {
        let &peek = code.peek()?;
        if peek == ')' {
            code.next()?;
            Some(Token::RBrace)
        } else {
            None
        }
    }
}
