use super::{Lexer, LexerResult, Token};

#[test]
fn read_integer_works_on_empty() {
    let mut it = vec![].into_iter();
    let mut lexer = Lexer::new(&mut it);
    let received = lexer.read_integer(5);
    assert_eq!(5, received);
}

#[test]
fn read_integer_works_on_nonempty() {
    let mut it = vec!['6', '7'].into_iter();
    let mut lexer = Lexer::new(&mut it);
    let received = lexer.read_integer(5);
    assert_eq!(567, received);
}

#[test]
fn read_all_tokens() {
    let mut it = vec!['1', '2', '3',
                      '+', ' ',
                      '-', ' ', ' ',
                      '4',
                      '*',
                      '/',
                      '(',
                      ')',
                      '5'].into_iter();
    let expected = vec![Token::Integer(123),
                        Token::Plus,
                        Token::Minus,
                        Token::Integer(4),
                        Token::Times,
                        Token::Div,
                        Token::LeftParen,
                        Token::RightParen,
                        Token::Integer(5)];
    let mut lexer = Lexer::new(&mut it);

    for token in expected {
        assert_eq!(LexerResult::Ok(token), lexer.next_token());
    }
    assert_eq!(LexerResult::OutOfTokens, lexer.next_token());
}

#[test]
fn read_tokens_bad_character() {
    let mut it = vec!['+', 'f', '-'].into_iter();
    let mut lexer = Lexer::new(&mut it);
    assert_eq!(LexerResult::Ok(Token::Plus), lexer.next_token());
    assert_eq!(LexerResult::UnknownCharacter('f'), lexer.next_token());
    assert_eq!(LexerResult::Ok(Token::Minus), lexer.next_token());
    assert_eq!(LexerResult::OutOfTokens, lexer.next_token());
}

#[test]
fn full_tokenization_is_ok() {
    let mut it = vec!['1', '2', '3',
                      '+', ' ',
                      '-', ' ', ' ',
                      '4',
                      '*',
                      '/',
                      '(',
                      ')',
                      '5'].into_iter();
    let expected = vec![Token::Integer(123),
                        Token::Plus,
                        Token::Minus,
                        Token::Integer(4),
                        Token::Times,
                        Token::Div,
                        Token::LeftParen,
                        Token::RightParen,
                        Token::Integer(5)];
    assert_eq!(Result::Ok(expected),
               Lexer::tokenize(&mut it));
}
