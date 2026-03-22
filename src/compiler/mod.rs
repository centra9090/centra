pub mod lexer;
pub mod parser;
pub mod ir;

pub enum Token<'a> {
    Word(&'a str),
}

pub type AST<'a> = Vec<&'a str>;
pub type IR<'a> = Vec<&'a str>;