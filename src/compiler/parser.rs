use super::{Token, AST};

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> AST<'a> {
    tokens.into_iter().map(|t| match t {
        Token::Word(s) => s,
    }).collect()
}