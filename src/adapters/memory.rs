use super::transport::Transport;

pub struct MemoryTransport {
    pub buffer: Vec<u8>,
}

impl Transport for MemoryTransport {
    fn send(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }
}