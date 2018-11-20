use parser::{Expression, Operation};
use super::Interpreter;

#[test]
fn integers_interpret() {
    assert_eq!(Result::Ok(42),
               Expression::Integer(42).evaluate());
}

#[test]
fn unary_minus_interprets() {
    let e = Expression::UnaryMinus(Box::new(Expression::Integer(5)));
    assert_eq!(Result::Ok(-5), e.evaluate());
}

fn assert_op(op: Operation, expected: i32) {
    let e = Expression::BinaryOperation(Box::new(Expression::Integer(6)),
                                        op,
                                        Box::new(Expression::Integer(2)));
    assert_eq!(Result::Ok(expected), e.evaluate());
}

#[test]
fn plus_interprets() {
    assert_op(Operation::Plus, 8);
}

#[test]
fn minus_interprets() {
    assert_op(Operation::Minus, 4);
}

#[test]
fn times_interprets() {
    assert_op(Operation::Times, 12);
}

#[test]
fn div_by_nonzero_interprets() {
    assert_op(Operation::Div, 3);
}

#[test]
fn div_by_zero_fails_safely() {
    let e = Expression::BinaryOperation(Box::new(Expression::Integer(1)),
                                        Operation::Div,
                                        Box::new(Expression::Integer(0)));
    assert!(e.evaluate().is_err());
}
