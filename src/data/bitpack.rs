pub struct BitWriter {
    buffer: Vec<u8>,
    current: u8,
    position: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter {
            buffer: Vec::new(),
            current: 0,
            position: 0,
        }
    }

    pub fn write_bits(&mut self, value: u8, bits: u8) {
        let mut remaining = bits;
        let mut val = value;
        while remaining > 0 {
            let space = 8 - self.position;
            let to_write = remaining.min(space);
            let mask = (1u8 << to_write) - 1;
            let bits_val = (val & mask) << self.position;
            self.current |= bits_val;
            val >>= to_write;
            self.position += to_write;
            remaining -= to_write;
            if self.position == 8 {
                self.buffer.push(self.current);
                self.current = 0;
                self.position = 0;
            }
        }
    }

    pub fn finalize(mut self) -> Vec<u8> {
        if self.position > 0 {
            self.buffer.push(self.current);
        }
        self.buffer
    }
}

pub struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    pub fn read_bits(&mut self, bits: u8) -> u8 {
        let mut result = 0u8;
        let mut remaining = bits;
        let mut shift = 0;
        while remaining > 0 && self.byte_pos < self.data.len() {
            let available = 8 - self.bit_pos;
            let to_read = remaining.min(available);
            let mask = ((1u8 << to_read) - 1) << self.bit_pos;
            let byte = self.data[self.byte_pos];
            let bits_val = (byte & mask) >> self.bit_pos;
            result |= bits_val << shift;
            shift += to_read;
            self.bit_pos += to_read;
            remaining -= to_read;
            if self.bit_pos == 8 {
                self.byte_pos += 1;
                self.bit_pos = 0;
            }
        }
        result
    }
}