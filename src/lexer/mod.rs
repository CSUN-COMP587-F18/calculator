use std::fmt;

use super::common::PushbackIterator;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Token {
    Integer(u32),
    Plus,
    Minus,
    Times,
    Div,
    LeftParen,
    RightParen
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Token as fmt::Debug>::fmt(self, f)
    }
}

pub struct Lexer<'a> {
    around: PushbackIterator<'a, char>
}

#[derive(Debug, Eq, PartialEq)]
enum LexerResult {
    Ok(Token),
    UnknownCharacter(char),
    OutOfTokens
}

impl<'a> Lexer<'a> {
    pub fn tokenize(input: &mut Iterator<Item = char>) -> Result<Vec<Token>, String> {
        let mut lexer = Lexer::new(input);
        let mut tokens = vec![];

        loop {
            match lexer.next_token() {
                LexerResult::Ok(token) => tokens.push(token),
                LexerResult::UnknownCharacter(c) =>
                    return Result::Err(format!("Unknown character: {}", c)),
                LexerResult::OutOfTokens =>
                    return Result::Ok(tokens)
            }
        }
    }

    fn new(input: &mut Iterator<Item = char>) -> Lexer {
        Lexer {
            around: PushbackIterator::new(input)
        }
    }    

    fn next_char(&mut self) -> Option<char> {
        self.around.next()
    }
    
    fn read_integer(&mut self, start_digit: u32) -> u32 {
        let mut result: u32 = start_digit;
        let mut current_char = self.next_char();

        loop {
            match current_char {
                Some(c) => {
                    match c.to_digit(10) {
                        Some(d) => {
                            result *= 10;
                            result += d;
                            current_char = self.next_char();
                        },
                        None => {
                            self.around.push(c);
                            return result;
                        }
                    }
                },
                None => {
                    return result;
                }
            } // match
        } // loop
    } // read_integer

    fn next_token(&mut self) -> LexerResult {
        loop {
            match self.next_char() {
                Some('+') => return LexerResult::Ok(Token::Plus),
                Some('-') => return LexerResult::Ok(Token::Minus),
                Some('*') => return LexerResult::Ok(Token::Times),
                Some('/') => return LexerResult::Ok(Token::Div),
                Some('(') => return LexerResult::Ok(Token::LeftParen),
                Some(')') => return LexerResult::Ok(Token::RightParen),
                Some(w) if w.is_whitespace() => {},
                Some(o) => {
                    match o.to_digit(10) {
                        Some(d) =>
                            return LexerResult::Ok(
                                Token::Integer(self.read_integer(d))),
                        None => return LexerResult::UnknownCharacter(o)
                    }
                },
                None => return LexerResult::OutOfTokens
            } // match
        } // loop
    } // next_token
}
