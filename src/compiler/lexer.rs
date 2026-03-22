use super::Token;

pub fn lex<'a>(input: &'a str) -> Vec<Token<'a>> {
    input.split_whitespace().map(|s| Token::Word(s)).collect()
}