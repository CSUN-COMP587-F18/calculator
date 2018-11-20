extern crate rand;

use super::Lexer;

use self::rand::{thread_rng, Rng};

const NUM_TESTS: u32 = 100000;
const MAX_NUM_CHARACTERS: u32 = 50;

fn rand_characters<R: Rng + ?Sized>(rng: &mut R) -> Vec<char> {
    let num_characters = rng.gen_range(0, MAX_NUM_CHARACTERS + 1);
    let mut characters = vec![];
    
    for _ in 0..num_characters {
        let cur: char = rng.gen();
        characters.push(cur);
    }

    characters
}

#[test]
fn run_lexer_fuzzer() {
    let mut rng = thread_rng();

    for _ in 0..NUM_TESTS {
        let input = rand_characters(&mut rng);
        // this will panic if it crashes
        let _ = Lexer::tokenize(&mut input.into_iter());
    }
}
