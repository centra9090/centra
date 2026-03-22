pub trait Transport {
    fn send(&mut self, data: &[u8]);
}