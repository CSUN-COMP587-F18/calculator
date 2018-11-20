extern crate rand;

use common::fuzzer::Fuzzer;
use lexer::Token;
use super::Parser;
use self::rand::{thread_rng, Rng};

const MAX_NUM_TOKENS: u32 = 50;

struct ParserFuzzer<R: Rng> {
    random: R
}

impl<R: Rng> ParserFuzzer<R> {
    fn num_tokens(&mut self) -> u32 {
        self.random.gen_range(0, MAX_NUM_TOKENS + 1)
    }
    
    fn rand_token(&mut self) -> Token {
        let rng = self.random_generator();
        match rng.gen_range(0, 7) {
            0 => Token::Integer(rng.gen()),
            1 => Token::Plus,
            2 => Token::Minus,
            3 => Token::Times,
            4 => Token::Div,
            5 => Token::LeftParen,
            6 => Token::RightParen,
            _ => panic!("Generated value out of expected range")
        }
    }
}

impl<ActualR: Rng> Fuzzer for ParserFuzzer<ActualR> {
    type Item = Vec<Token>;
    type R = ActualR;

    fn gen_one(&mut self) -> Vec<Token> {
        let num_tokens = self.num_tokens();
        let mut tokens = vec![];
    
        for _ in 0..num_tokens {
            let cur = self.rand_token();
            tokens.push(cur);
        }

        tokens
    }

    fn run_test(&mut self, item: Vec<Token>) {
        let _ = Parser::parse(&item);
    }

    fn random_generator(&mut self) -> &mut ActualR {
        &mut self.random
    }
}

#[test]
fn run_parser_fuzzer() {
    let mut fuzzer = ParserFuzzer {
        random: thread_rng()
    };
    fuzzer.run_tests();
}
