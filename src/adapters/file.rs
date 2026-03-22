use super::transport::Transport;
use std::fs;

pub struct FileTransport {
    pub path: String,
}

impl Transport for FileTransport {
    fn send(&mut self, data: &[u8]) {
        fs::write(&self.path, data).unwrap();
    }
}