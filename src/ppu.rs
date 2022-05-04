mod registers;

use self::registers::Registers;
use crate::bus::{BusAddr, ByteReadable, ByteWritable};

pub struct PPU {
    registers: Registers,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }
}

impl ByteReadable for PPU {
    fn read_byte(&self, addr: BusAddr) -> u8 {
        match addr {
            0x0002 => self.registers.ppu_status,
            0x0004 => self.registers.oam_data,
            0x0007 => self.registers.ppu_data,
            _ => {
                panic!("PPU {} is not readable", addr);
            }
        }
    }
}

impl ByteWritable for PPU {
    fn write_byte(&mut self, addr: BusAddr, value: u8) {
        match addr {
            0x0000 => self.registers.ppu_ctrl = value,
            0x0001 => self.registers.ppu_mask = value,
            0x0003 => self.registers.oam_addr = value,
            0x0004 => self.registers.oam_data = value,
            0x0005 => self.registers.ppu_scroll = value,
            0x0006 => self.registers.ppu_addr = value,
            0x0007 => self.registers.ppu_data = value,
            _ => {
                panic!("PPU {} is not writable", addr);
            }
        }
    }
}
