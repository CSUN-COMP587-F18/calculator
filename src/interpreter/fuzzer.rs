extern crate rand;

use common::fuzzer::Fuzzer;
use self::rand::{thread_rng, Rng};
use parser::{Expression, Operation};
use super::Interpreter;

const EXPRESSION_MAX_DEPTH: u32 = 10;

struct InterpreterFuzzer<R: Rng> {
    random: R
}

impl<R: Rng> InterpreterFuzzer<R> {
    fn rand_integer_expression(&mut self) -> Expression {
        Expression::Integer(self.random.gen())
    }

    fn rand_operation(&mut self) -> Operation {
        match self.random.gen_range(0, 4) {
            0 => Operation::Plus,
            1 => Operation::Minus,
            2 => Operation::Times,
            3 => Operation::Div,
            _ => panic!("Operation out of range")
        }
    }
    
    fn rand_expression(&mut self, depth_remaining: u32) -> Expression {
        if depth_remaining == 0 {
            self.rand_integer_expression()
        } else {
            match self.random.gen_range(0, 3) {
                0 => self.rand_integer_expression(),
                1 => {
                    let e = self.rand_expression(depth_remaining - 1);
                    Expression::UnaryMinus(Box::new(e))
                },
                2 => {
                    let new_bound = depth_remaining - 1;
                    let left = self.rand_expression(new_bound);
                    let op = self.rand_operation();
                    let right = self.rand_expression(new_bound);
                    Expression::BinaryOperation(Box::new(left),
                                                op,
                                                Box::new(right))
                },
                _ => panic!("Expression out of range")
            }
        }
    }
}

impl<ActualR: Rng> Fuzzer for InterpreterFuzzer<ActualR> {
    type Item = Expression;
    type R = ActualR;

    fn gen_one(&mut self) -> Expression {
        self.rand_expression(EXPRESSION_MAX_DEPTH)
    }

    fn run_test(&mut self, item: Expression) {
        let _ = item.evaluate();
    }
    
    fn random_generator(&mut self) -> &mut ActualR {
        &mut self.random
    }
}

#[test]
fn run_interpreter_fuzzer() {
    let mut fuzzer = InterpreterFuzzer {
        random: thread_rng()
    };
    fuzzer.run_tests();
}
