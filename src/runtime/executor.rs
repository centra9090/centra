use crate::compiler::IR;
use crate::data::Symbol;

pub fn execute<'a>(ir: IR<'a>) -> Vec<Symbol<'a>> {
    ir.into_iter().map(|s| Symbol::Word(s)).collect()
}