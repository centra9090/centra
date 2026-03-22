use super::bitpack::BitReader;

pub fn decode(data: &[u8]) -> Vec<String> {
    if data.len() < 3 {
        return vec![];
    }
    let bit_width = data[0];
    let unique_count = data[1] as usize;
    let total_symbols = data[2] as usize;
    let mut pos = 3;
    let mut dict = Vec::with_capacity(unique_count);
    for _ in 0..unique_count {
        if pos >= data.len() {
            return vec![];
        }
        let len = data[pos] as usize;
        pos += 1;
        if pos + len > data.len() {
            return vec![];
        }
        let word_bytes = &data[pos..pos + len];
        let word = std::str::from_utf8(word_bytes).unwrap().to_string();
        dict.push(word);
        pos += len;
    }
    let mut bit_reader = BitReader::new(&data[pos..]);
    let mut result = Vec::with_capacity(total_symbols);
    for _ in 0..total_symbols {
        let id = bit_reader.read_bits(bit_width);
        if id == 0 || id as usize > dict.len() {
            break;
        }
        result.push(dict[(id - 1) as usize].clone());
    }
    result
}