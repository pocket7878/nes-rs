use crate::bus::{BusAddr, ByteReadable, ByteWritable};

pub struct Pad;

impl Pad {
    pub fn new() -> Self {
        Self
    }
}

impl ByteReadable for Pad {
    fn read_byte(&self, _addr: BusAddr) -> u8 {
        todo!()
    }
}

impl ByteWritable for Pad {
    fn write_byte(&mut self, _addr: BusAddr, _value: u8) {
        todo!()
    }
}
