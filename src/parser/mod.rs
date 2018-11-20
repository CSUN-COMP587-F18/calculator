use std::collections::HashMap;
use std::fmt;

use super::lexer::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Integer(u32),
    UnaryMinus(Box<Expression>),
    BinaryOperation(Box<Expression>, Operation, Box<Expression>)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Expression as fmt::Debug>::fmt(self, f)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Div
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Operation as fmt::Debug>::fmt(self, f)
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token]
}

type ParseResult<A> = Result<A, String>;

impl<'a> Parser<'a> {
    fn new(tokens: &[Token]) -> Parser {
        Parser { tokens: tokens }
    }

    pub fn parse(tokens: &[Token]) -> ParseResult<Expression> {
        Parser::new(tokens).parse_toplevel_expression()
    }

    fn get_token(&self, pos: usize) -> ParseResult<&Token> {
        match self.tokens.get(pos) {
            Some(token) => Result::Ok(token),
            None => Result::Err(format!("Out of tokens"))
        }
    }

    fn unexpected_token<A>(token: &Token, expected: &str) -> ParseResult<A> {
        Result::Err(
            format!("Unexpected token: {}.  Expected {}", token, expected))
    }

    fn parse_toplevel_expression(&self) -> ParseResult<Expression> {
        let (e, pos) = try!(self.parse_expression(0));
        if pos == self.tokens.len() {
            Result::Ok(e)
        } else {
            Result::Err(format!("Extra tokens after position {}", pos))
        }
    }

    fn parse_expression(&self, pos1: usize) -> ParseResult<(Expression, usize)> {
        self.parse_additive_expression(pos1)
    }

    // given a token representing an operator, it will give a function
    // that will make a binop holding that operator
    fn parse_arithmetic_expression(&self, start_pos: usize,
                                   base_parse: fn(&Parser<'a>, usize) -> ParseResult<(Expression, usize)>,
                                   map: &HashMap<Token, fn(Box<Expression>, Box<Expression>) -> Expression>) -> ParseResult<(Expression, usize)> {
        let (temp_result, temp_cur_pos) = try!(base_parse(self, start_pos));
        let mut result = temp_result;
        let mut cur_pos = temp_cur_pos;
        
        loop {
            // see if we have a token
            match self.get_token(cur_pos) {
                // we have a token
                Ok(token) => {
                    // see if we recognize it
                    match map.get(token) {
                        // token recognized as one we know
                        Some(make_expression) => {
                            // see if we can get the value afterward
                            match base_parse(self, cur_pos + 1) {
                                // had something valid
                                Ok((right_expression, next_pos)) => {
                                    // update value and position
                                    result = make_expression(Box::new(result), Box::new(right_expression));
                                    cur_pos = next_pos;
                                },
                                // had something invalid.  Bail with what we have so far.
                                Err(_) => {
                                    return Result::Ok((result, cur_pos));
                                }
                            }
                        },
                        // token not recognized: bail with what we have so far
                        None => {
                            return Result::Ok((result, cur_pos));
                        }
                    }
                },
                // we don't have a token; bail with what we have so far
                Err(_) => {
                    return Result::Ok((result, cur_pos));
                }
            }
        }
    }

    fn parse_additive_expression(&self, start_pos: usize) -> ParseResult<(Expression, usize)> {
        // addExp \in AdditiveExpression ::= multExp (('+' | '-') multExp)*
        fn make_plus(e1: Box<Expression>, e2: Box<Expression>) -> Expression {
            Expression::BinaryOperation(e1, Operation::Plus, e2)
        }
        fn make_minus(e1: Box<Expression>, e2: Box<Expression>) -> Expression {
            Expression::BinaryOperation(e1, Operation::Minus, e2)
        }
        let mut map: HashMap<Token, fn(Box<Expression>, Box<Expression>) -> Expression> = HashMap::new();
        map.insert(Token::Plus, make_plus);
        map.insert(Token::Minus, make_minus);
        self.parse_arithmetic_expression(start_pos,
                                         Parser::parse_multiplicative_expression,
                                         &map)
    }

    fn parse_multiplicative_expression(&self, start_pos: usize) -> ParseResult<(Expression, usize)> {
        // multExp \in MultiplicativeExpression ::= primary (('*' | '/') primary)*
        fn make_times(e1: Box<Expression>, e2: Box<Expression>) -> Expression {
            Expression::BinaryOperation(e1, Operation::Times, e2)
        }
        fn make_div(e1: Box<Expression>, e2: Box<Expression>) -> Expression {
            Expression::BinaryOperation(e1, Operation::Div, e2)
        }
        let mut map: HashMap<Token, fn(Box<Expression>, Box<Expression>) -> Expression> = HashMap::new();
        map.insert(Token::Times, make_times);
        map.insert(Token::Div, make_div);
        self.parse_arithmetic_expression(start_pos,
                                         Parser::parse_primary,
                                         &map)
    }
    
    fn parse_primary(&self, pos1: usize) -> ParseResult<(Expression, usize)> {
        match try!(self.get_token(pos1)) {
            Token::LeftParen => {
                let (expression, pos2) = try!(self.parse_expression(pos1 + 1));
                match try!(self.get_token(pos2)) {
                    Token::RightParen => Result::Ok((expression, pos2 + 1)),
                    other => Parser::unexpected_token(other, "right paren"),
                }
            },
            Token::Integer(ref i) => Result::Ok((Expression::Integer(*i), pos1 + 1)),
            Token::Minus => {
                println!("IN MINUS");
                let (primary, pos2) = try!(self.parse_primary(pos1 + 1));
                Result::Ok(
                    (Expression::UnaryMinus(
                        Box::new(primary)),
                     pos2))
            },
            other => Parser::unexpected_token(&other, "left paren; integer; unary minus"),
        }
    }
}

#[cfg(test)]
mod test;
#[cfg(test)]
mod fuzzer;
