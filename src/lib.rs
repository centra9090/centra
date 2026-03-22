pub mod compiler;
pub mod runtime;
pub mod data;
pub mod adapters;
pub mod system;

pub use system::mode::Mode;
pub use system::runner::run;

pub fn run_source(input: &str) -> Vec<u8> {
    let tokens = compiler::lexer::lex(input);
    let ast = compiler::parser::parse(tokens);
    let ir = compiler::ir::lower(ast);
    let symbols = runtime::executor::execute(ir);
    data::csm_v2::encode(symbols)
}

pub fn decode_input(input: &str) -> Vec<String> {
    let compressed = run_source(input);
    data::decode::decode(&compressed)
}

pub fn run_with_transport<T: adapters::transport::Transport>(input: &str, transport: &mut T) {
    let output = run_source(input);
    transport.send(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_source() {
        let compressed = run_source("hello world");
        let decoded = decode_input("hello world");
        assert_eq!(decoded, vec!["hello".to_string(), "world".to_string()]);
    }
}