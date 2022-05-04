use crate::bus::{BusAddr, ByteReadable};

pub struct ProgramROM {
    pub data: Vec<u8>,
}

impl ProgramROM {
    pub fn new(data: &[u8]) -> ProgramROM {
        ProgramROM {
            data: data.to_vec(),
        }
    }
}

impl ByteReadable for ProgramROM {
    fn read_byte(&self, addr: BusAddr) -> u8 {
        self.data[addr as usize]
    }
}
