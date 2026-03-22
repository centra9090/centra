use std::collections::HashMap;
use super::{Symbol, bitpack::BitWriter};

pub struct Dictionary<'a> {
    map: HashMap<&'a str, u8>,
    reverse: Vec<&'a str>,
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Self {
        Dictionary {
            map: HashMap::new(),
            reverse: Vec::new(),
        }
    }

    pub fn get_or_insert(&mut self, word: &'a str) -> u8 {
        if let Some(&id) = self.map.get(word) {
            id
        } else {
            let id = (self.reverse.len() + 1) as u8;
            self.map.insert(word, id);
            self.reverse.push(word);
            id
        }
    }
}

pub fn encode<'a>(symbols: Vec<Symbol<'a>>) -> Vec<u8> {
    let mut dict = Dictionary::new();
    for symbol in &symbols {
        if let Symbol::Word(word) = symbol {
            dict.get_or_insert(*word);
        }
    }
    let unique_count = dict.reverse.len() as u8;
    let bit_width = if unique_count <= 2 {
        1
    } else if unique_count <= 4 {
        2
    } else if unique_count <= 16 {
        4
    } else {
        8
    };
    let total_symbols = symbols.len() as u8;
    let mut buffer = Vec::new();
    buffer.push(bit_width);
    buffer.push(unique_count);
    buffer.push(total_symbols);
    for word in &dict.reverse {
        buffer.push(word.len() as u8);
        buffer.extend_from_slice(word.as_bytes());
    }
    let mut bit_writer = BitWriter::new();
    for symbol in symbols {
        if let Symbol::Word(word) = symbol {
            let id = dict.map[word];
            bit_writer.write_bits(id, bit_width);
        }
    }
    buffer.extend(bit_writer.finalize());
    buffer
}