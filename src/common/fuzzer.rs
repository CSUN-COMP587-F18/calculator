extern crate rand;

use self::rand::Rng;

pub trait Fuzzer {
    type Item;
    type R: Rng;
    
    fn gen_one(&mut self) -> Self::Item;

    // panics if test fails
    fn run_test(&mut self, item: Self::Item);
    
    fn random_generator(&mut self) -> &mut Self::R;

    fn num_tests(&self) -> u32 {
        100000
    }
    
    fn run_tests(&mut self) {
        for _ in 0..self.num_tests() {
            let item = self.gen_one();
            self.run_test(item);
        }
    }
}
