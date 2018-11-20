extern crate rand;

use super::Lexer;
use common::fuzzer::Fuzzer;

use self::rand::{thread_rng, Rng};

const MAX_NUM_CHARACTERS: u32 = 50;

struct LexerFuzzer<R: Rng> {
    random: R
}

impl<ActualR: Rng> Fuzzer for LexerFuzzer<ActualR> {
    type Item = Vec<char>;
    type R = ActualR;

    fn gen_one(&mut self) -> Vec<char> {
        let rng = self.random_generator();
        let num_characters = rng.gen_range(0, MAX_NUM_CHARACTERS + 1);
        let mut characters = vec![];
    
        for _ in 0..num_characters {
            let cur: char = rng.gen();
            characters.push(cur);
        }

        characters
    }

    fn run_test(&mut self, item: Vec<char>) {
        let _ = Lexer::tokenize(&mut item.into_iter());
    }
    
    fn random_generator(&mut self) -> &mut ActualR {
        &mut self.random
    }
}
    
#[test]
fn run_lexer_fuzzer() {
    let mut fuzzer = LexerFuzzer {
        random: thread_rng()
    };
    fuzzer.run_tests();
}
