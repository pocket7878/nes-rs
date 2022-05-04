use crate::bus::{BusAddr, ByteReadable, ByteWritable};

pub struct APU;

impl APU {
    pub fn new() -> Self {
        Self
    }
}

impl ByteReadable for APU {
    fn read_byte(&self, _addr: BusAddr) -> u8 {
        todo!()
    }
}

impl ByteWritable for APU {
    fn write_byte(&mut self, _addr: BusAddr, _value: u8) {
        todo!()
    }
}
