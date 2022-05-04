use crate::bus::{BusAddr, ByteReadable, ByteWritable};

pub struct DMA;

impl DMA {
    pub fn new() -> Self {
        Self
    }
}

impl ByteReadable for DMA {
    fn read_byte(&self, _addr: BusAddr) -> u8 {
        todo!()
    }
}

impl ByteWritable for DMA {
    fn write_byte(&mut self, _addr: BusAddr, _value: u8) {
        todo!()
    }
}
