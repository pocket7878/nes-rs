use crate::bus::{BusAddr, ByteReadable, ByteWritable};

pub struct RAM {
    data: Vec<u8>,
}

const RAM_SIZE: usize = 0x800;

impl RAM {
    pub fn new() -> RAM {
        RAM {
            data: vec![0; RAM_SIZE],
        }
    }
}

impl ByteReadable for RAM {
    fn read_byte(&self, addr: BusAddr) -> u8 {
        self.data[addr as usize]
    }
}

impl ByteWritable for RAM {
    fn write_byte(&mut self, addr: BusAddr, value: u8) {
        self.data[addr as usize] = value;
    }
}
