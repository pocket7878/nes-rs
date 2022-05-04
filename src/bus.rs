use crate::{apu::APU, dma::DMA, ines::ProgramROM, pad::Pad, ppu::PPU, ram::RAM};

const MEMORY_MAX_ADDRESS: usize = 0xFFFF;

const WRAM_START_ADDR: usize = 0x0000;
const WRAM_END_ADDR: usize = 0x07FF;
const WRAM_MIRROR_START_ADDR: usize = 0x0800;
const WRAM_MIRROR_END_ADDR: usize = 0x1FFF;
const PPU_REGISTERS_START_ADDR: usize = 0x2000;
const PPU_REGISTERS_END_ADDR: usize = 0x2007;
const PPU_MIRROR_REGISTERS_START_ADDR: usize = 0x2008;
const PPU_MIRROR_REGISTERS_END_ADDR: usize = 0x3FFF;

const PROGRAM_ROM_SIZE: usize = 0x8000;

pub type BusAddr = u16;

pub trait ByteReadable {
    fn read_byte(&self, addr: BusAddr) -> u8;
}

pub trait ByteWritable {
    fn write_byte(&mut self, addr: BusAddr, value: u8);
}

pub struct Bus<'a> {
    wram: &'a mut RAM,
    program_rom: &'a ProgramROM,
    ppu: &'a mut PPU,
    apu: &'a mut APU,
    pad: &'a mut Pad,
    dma: &'a mut DMA,
}

impl<'a> Bus<'a> {
    pub fn new(
        wram: &'a mut RAM,
        program_rom: &'a ProgramROM,
        ppu: &'a mut PPU,
        apu: &'a mut APU,
        pad: &'a mut Pad,
        dma: &'a mut DMA,
    ) -> Bus<'a> {
        Bus {
            wram,
            program_rom,
            ppu,
            apu,
            pad,
            dma,
        }
    }
}

impl<'a> ByteReadable for Bus<'a> {
    fn read_byte(&self, addr: BusAddr) -> u8 {
        if addr < 0x800 {
            self.wram.read_byte(addr)
        } else if addr < 0x2000 {
            self.wram.read_byte(addr - 0x0800)
        } else if addr < 0x4000 {
            self.ppu.read_byte((addr - 0x2000) % 8)
        } else if addr == 0x4016 {
            self.pad.read_byte(addr)
        } else if addr >= 0xC000 {
            if self.program_rom.data.len() <= 0x4000 {
                self.program_rom.read_byte(addr - 0xC000)
            } else {
                self.program_rom.read_byte(addr - 0x8000)
            }
        } else if addr >= 0x8000 {
            self.program_rom.read_byte(addr - 0x8000)
        } else {
            // 一旦拡張ROM, RAMは仕様されていない前提とする
            0
        }
    }
}

impl<'a> ByteWritable for Bus<'a> {
    fn write_byte(&mut self, addr: BusAddr, value: u8) {
        if addr < 0x800 {
            self.wram.write_byte(addr, value)
        } else if addr < 0x2000 {
            self.wram.write_byte(addr - 0x8000, value)
        } else if addr < 0x2008 {
            self.ppu.write_byte(addr - 0x2000, value)
        } else if addr == 0x4014 {
            self.dma.write_byte(addr, value)
        } else if addr == 0x4016 {
            self.pad.write_byte(addr, value)
        } else if 0x4000 <= addr && addr < 0x4020 {
            self.apu.write_byte(addr, value)
        } else {
            // 一旦拡張ROM, RAMは仕様されていない前提とする
        }
    }
}
