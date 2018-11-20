pub struct PushbackIterator<'a, A> where A : 'a {
    around: &'a mut Iterator<Item = A>,
    stack: Vec<A>
}

impl<'a, A> PushbackIterator<'a, A> where A : 'a {
    pub fn new(input: &'a mut Iterator<Item = A>) -> PushbackIterator<'a, A> {
        PushbackIterator {
            around: input,
            stack: vec![]
        }
    }

    pub fn push(&mut self, item: A) {
        self.stack.push(item);
    }
}

impl<'a, A> Iterator for PushbackIterator<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        match self.stack.pop() {
            None => self.around.next(),
            some => some
        }
    }
}

#[cfg(test)]
mod test;
pub mod fuzzer;
