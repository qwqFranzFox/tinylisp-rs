use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use crate::data::IntType;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Bad identifier {0}")]
    BadIdentifier(String),
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid number: {0}")]
    ParseNumberError(<IntType as FromStr>::Err),
}

#[derive(Debug)]
pub enum Token {
    LBrace,
    RBrace,
    Number(IntType),
    Atomic(String),
    Error,
}

struct CharStreamWrapper<'a> {
    iter: Chars<'a>,
}

impl Iterator for CharStreamWrapper<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Tokenizer<'a> {
    iter: Peekable<CharStreamWrapper<'a>>,
}

fn is_atomic_char(ch: char) -> bool {
    !(ch.is_whitespace() || ch == '(' || ch == ')')
}
fn into_number(cache: String) -> Token {
    cache.parse().map(Token::Number).unwrap_or(Token::Error)
}
impl<'a> Tokenizer<'a> {
    pub fn new<P>(str: &'a P) -> Tokenizer<'a>
    where
        P: AsRef<str> + 'a,
    {
        Tokenizer {
            iter: CharStreamWrapper {
                iter: str.as_ref().chars(),
            }
            .peekable(),
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.iter.peek() {
            if ch.is_whitespace() {
                self.iter.next()?;
            } else {
                break;
            }
        }
        Some(match *self.iter.peek()? {
            '0'..='9' => {
                // match number
                let mut cache: String = String::new();
                while let Some(&ch) = self.iter.peek() {
                    if ch.is_numeric() {
                        cache.push(ch);
                        self.iter.next()?;
                    } else {
                        break;
                    }
                }
                if let Some(&ch) = self.iter.peek() {
                    if !is_atomic_char(ch) {
                        into_number(cache)
                    } else {
                        while let Some(&ch) = self.iter.peek() {
                            if is_atomic_char(ch) {
                                cache.push(ch);
                                self.iter.next();
                            } else {
                                break;
                            }
                        }
                        // Err(TokenizerError::BadIdentifier(cache))
                        Token::Error
                    }
                } else {
                    into_number(cache)
                }
            }
            '(' => {
                self.iter.next();
                Token::LBrace
            }
            ')' => {
                self.iter.next();
                Token::RBrace
            }
            _ => {
                // match atomic
                let mut cache: String = String::new();
                while let Some(&ch) = self.iter.peek() {
                    if is_atomic_char(ch) {
                        cache.push(ch);
                        self.iter.next()?;
                    } else {
                        break;
                    }
                }
                Token::Atomic(cache)
            }
        })
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
