use super::PushbackIterator;

#[test]
fn next_works_without_push() {
    let mut it = vec!['a'].into_iter();
    let mut pushback = PushbackIterator::new(&mut it);
    assert_eq!(Option::Some('a'), pushback.next());
    assert_eq!(Option::None, pushback.next());
}

#[test]
fn next_char_works_with_look_here_first() {
    let mut it = vec!['a'].into_iter();
    let mut pushback = PushbackIterator::new(&mut it);
    pushback.push('b');
    assert_eq!(Option::Some('b'), pushback.next());
    assert_eq!(Option::Some('a'), pushback.next());
    assert_eq!(Option::None, pushback.next());
}
