use std::fmt;

use super::parser::{Expression, Operation};

type InternalInterpreterResult = Result<i32, String>;

// newtype
pub struct InterpreterResult {
    result: InternalInterpreterResult
}

impl fmt::Display for InterpreterResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.result {
            Result::Ok(ref i) => f.write_fmt(format_args!("{}", i)),
            Result::Err(ref s) => f.write_fmt(format_args!("Error: {}", s))
        }
    }
}

pub trait OperationEvaluator {
    fn operation(&self, v1: i32, v2: i32) -> InternalInterpreterResult;
}

fn to_result(op: Option<i32>, if_error: String) -> InternalInterpreterResult {
    match op {
        Option::Some(i) => Result::Ok(i),
        Option::None => Result::Err(if_error)
    }
}

impl OperationEvaluator for Operation {
    fn operation(&self, v1: i32, v2: i32) -> InternalInterpreterResult {
        match self {
            Operation::Plus => to_result(v1.checked_add(v2),
                                         format!("addition overflow")),
            Operation::Minus => to_result(v1.checked_sub(v2),
                                          format!("subtraction overflow")),
            Operation::Times => to_result(v1.checked_mul(v2),
                                          format!("multiplication overflow")),
            Operation::Div => {
                if v2 == 0 {
                    Result::Err(format!("Division by zero"))
                } else {
                    to_result(v1.checked_div(v2),
                              format!("Division overflow / underflow"))
                }
            }
        }
    }
}

pub trait Interpreter {
    fn evaluate(&self) -> InternalInterpreterResult;
    fn evaluate_toplevel(&self) -> InterpreterResult {
        InterpreterResult {
            result: self.evaluate()
        }
    }
}

impl Interpreter for Expression {
    fn evaluate(&self) -> InternalInterpreterResult {
        match self {
            Expression::Integer(i) => Result::Ok(*i as i32),
            Expression::UnaryMinus(e) => {
                let e_value = try!(e.evaluate());
                Result::Ok(-e_value)
            },
            Expression::BinaryOperation(e1, op, e2) => {
                let e1_value = try!(e1.evaluate());
                let e2_value = try!(e2.evaluate());
                let result = try!(op.operation(e1_value, e2_value));
                Result::Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod test;
#[cfg(test)]
mod fuzzer;
