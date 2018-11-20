use super::{Parser, Expression, Operation};
use lexer::Token;

fn assert_parse(tokens: &[Token], expected_raw: Option<Expression>) {
    let parser = Parser::new(tokens);
    match (expected_raw, parser.parse_toplevel_expression()) {
        (Some(expected), Ok(received)) => assert_eq!(expected, received),
        (None, Err(_)) => {},
        (Some(expected), Err(error)) => assert!(false, "Expected {}, but got error {}", expected, error),
        (None, Ok(received)) => assert!(false, "Expected parse failure, but got {}", received)
    }
}

#[test]
fn parse_primary_parses_integer() {
    let parser = Parser::new(&[Token::Integer(42)]);
    assert_eq!(Result::Ok((Expression::Integer(42), 1)), parser.parse_primary(0));
}

#[test]
fn toplevel_parses_integer() {
    assert_parse(&[Token::Integer(42)],
                 Option::Some(Expression::Integer(42)));
}

#[test]
fn toplevel_parses_unary_minus() {
    assert_parse(&[Token::Minus, Token::Integer(5)],
                 Option::Some(Expression::UnaryMinus(Box::new(Expression::Integer(5)))));
}

fn assert_basic_op_parses(token: Token, op: Operation) {
    assert_parse(&[Token::Integer(1),
                   token,
                   Token::Integer(2)],
                 Option::Some(Expression::BinaryOperation(Box::new(Expression::Integer(1)),
                                                          op,
                                                          Box::new(Expression::Integer(2)))));
}

#[test]
fn toplevel_parses_addition() {
    assert_basic_op_parses(Token::Plus, Operation::Plus);
}

#[test]
fn toplevel_parses_subtraction() {
    assert_basic_op_parses(Token::Minus, Operation::Minus);
}

#[test]
fn toplevel_parses_times() {
    assert_basic_op_parses(Token::Times, Operation::Times);
}

#[test]
fn toplevel_parses_div() {
    assert_basic_op_parses(Token::Div, Operation::Div);
}

#[test]
fn addition_associates_left() {
    // 1 + 2 + 3 == (1 + 2) + 3
    let nested = Expression::BinaryOperation(Box::new(Expression::Integer(1)),
                                             Operation::Plus,
                                             Box::new(Expression::Integer(2)));
    assert_parse(&[Token::Integer(1),
                   Token::Plus,
                   Token::Integer(2),
                   Token::Plus,
                   Token::Integer(3)],
                 Option::Some(Expression::BinaryOperation(Box::new(nested),
                                                          Operation::Plus,
                                                          Box::new(Expression::Integer(3)))));
}

#[test]
fn multiplication_over_addition() {
    // 1 + 2 * 3 == 1 + (2 * 3)
    let nested = Expression::BinaryOperation(Box::new(Expression::Integer(2)),
                                             Operation::Times,
                                             Box::new(Expression::Integer(3)));
    assert_parse(&[Token::Integer(1),
                   Token::Plus,
                   Token::Integer(2),
                   Token::Times,
                   Token::Integer(3)],
                 Option::Some(Expression::BinaryOperation(Box::new(Expression::Integer(1)),
                                                          Operation::Plus,
                                                          Box::new(nested))));
}

#[test]
fn parens_override_precedence() {
    // (1 + 2) * 3 == (1 + 2) * 3
    let nested = Expression::BinaryOperation(Box::new(Expression::Integer(1)),
                                             Operation::Plus,
                                             Box::new(Expression::Integer(2)));
    assert_parse(&[Token::LeftParen,
                   Token::Integer(1),
                   Token::Plus,
                   Token::Integer(2),
                   Token::RightParen,
                   Token::Times,
                   Token::Integer(3)],
                 Option::Some(Expression::BinaryOperation(Box::new(nested),
                                                          Operation::Times,
                                                          Box::new(Expression::Integer(3)))));
}
    
#[test]
fn unary_minus_over_division() {
    // -1 / 2 == (-1) / 2
    let nested = Expression::UnaryMinus(Box::new(Expression::Integer(1)));
    assert_parse(&[Token::Minus,
                   Token::Integer(1),
                   Token::Div,
                   Token::Integer(2)],
                 Option::Some(Expression::BinaryOperation(Box::new(nested),
                                                          Operation::Div,
                                                          Box::new(Expression::Integer(2)))));
}
    
#[test]
fn unary_minus_with_minus() {
    // -1 - -2 == (-1) - (-2)
    let nested_left = Expression::UnaryMinus(Box::new(Expression::Integer(1)));
    let nested_right = Expression::UnaryMinus(Box::new(Expression::Integer(2)));
    assert_parse(&[Token::Minus,
                   Token::Integer(1),
                   Token::Minus,
                   Token::Minus,
                   Token::Integer(2)],
                 Option::Some(Expression::BinaryOperation(Box::new(nested_left),
                                                          Operation::Minus,
                                                          Box::new(nested_right))));
}

#[test]
fn extra_tokens_is_error() {
    assert_parse(&[Token::Integer(1),
                   Token::Integer(2)],
                 Option::None);
}

#[test]
fn missing_right_paren_is_error() {
    // (1
    assert_parse(&[Token::LeftParen,
                   Token::Integer(1)],
                 None);
}
