use super::mode::Mode;
use crate::run_source;

pub fn run(input: &str, mode: Mode) -> Vec<u8> {
    let data = run_source(input);
    match mode {
        Mode::Fast => data,
        Mode::Secure => apply_security(data),
        Mode::Intelligent => {
            trigger_intelligence(&data);
            data
        }
    }
}

fn apply_security(data: Vec<u8>) -> Vec<u8> {
    data
}

fn trigger_intelligence(_data: &[u8]) {
    // do nothing
}