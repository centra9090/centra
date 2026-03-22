use super::Symbol;

pub fn encode<'a>(symbols: &[Symbol<'a>]) -> Vec<u8> {
    let word_count = symbols.len();
    let total_len = symbols.iter().map(|s| match s {
        Symbol::Word(w) => w.len(),
    }).sum::<usize>() + word_count;
    let mut buf = Vec::with_capacity(total_len);
    for symbol in symbols {
        match symbol {
            Symbol::Word(w) => {
                buf.extend_from_slice(w.as_bytes());
                buf.push(b' ');
            }
        }
    }
    buf
}