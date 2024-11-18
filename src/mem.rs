pub struct Mem {
    pub data: Vec<u8>,
}

impl Mem {
    pub fn new() -> Self {
        Self {
            data: vec![0; 8096],
        }
    }

    pub fn memcpy_origin(&mut self, src: Vec<u8>) {
        self.data[0..src.len()].copy_from_slice(&src);
    }

    pub fn get(&self, addr: u64) -> &[u8] {
        &self.data[addr as usize..]
    }
}
