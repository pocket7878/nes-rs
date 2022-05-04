mod instruction;
mod registers;

use crate::bus::{Bus, BusAddr, ByteReadable, ByteWritable};
use registers::{ProgramCounter, Registers};

use self::instruction::{AddressingMode, InstructionType, OpCode, OpCodeDecoder, Operand};

pub struct CPU<'a> {
    registers: Registers,
    bus: &'a mut Bus<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(bus: &'a mut Bus<'a>) -> Self {
        Self {
            registers: Registers::new(),
            bus,
        }
    }

    pub fn boot(&mut self) {
        self.registers.reset();
        self.registers.set_pc(self.reset_interrupt_pc());
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.registers.set_pc(self.reset_interrupt_pc());
    }

    pub fn run_single_cycle(&mut self) -> Result<(), String> {
        let tmp_pc = self.registers.pc;
        let opcode = self.fetch_opcode()?;
        let operand = self.fetch_operand(opcode.addressing_mode.clone());
        eprintln!("[0x{:02X}]: {:?} {:?}", tmp_pc, opcode, operand);
        self.execute(opcode.clone(), operand);

        Ok(())
    }

    fn fetch_opcode(&mut self) -> Result<OpCode, String> {
        let opcode_byte = self.fetch();
        let opcode = OpCodeDecoder::decode(opcode_byte);
        opcode.ok_or(format!("Unexpected opcode: 0x{:02X}", opcode_byte))
    }

    fn fetch_operand(&mut self, addressing_mode: AddressingMode) -> Option<Operand> {
        match addressing_mode {
            AddressingMode::Accumulator => None,
            AddressingMode::Immediate => {
                let data = self.fetch();
                Some(Operand::Immediate(data))
            }
            AddressingMode::Absolute => {
                let lower_half_addr = self.fetch();
                let upper_half_addr = self.fetch();
                let addr = (upper_half_addr as u16) << 8 | lower_half_addr as u16;

                Some(Operand::Address(addr))
            }
            AddressingMode::ZeroPage => {
                let lower_half_addr = self.fetch();
                let addr = lower_half_addr as u16;

                Some(Operand::Address(addr))
            }
            AddressingMode::ZeroPageX => {
                let lower_half_addr = self.fetch();
                let addr = lower_half_addr.wrapping_add(self.registers.x) as u16;

                Some(Operand::Address(addr))
            }
            AddressingMode::ZeroPageY => {
                let lower_half_addr = self.fetch();
                let addr = lower_half_addr.wrapping_add(self.registers.y) as u16;

                Some(Operand::Address(addr))
            }
            AddressingMode::AbsoluteX => {
                let lower_half_addr = self.fetch();
                let upper_half_addr = self.fetch();
                let mut addr = (upper_half_addr as u16) << 8 | lower_half_addr as u16;
                addr = addr.wrapping_add(self.registers.x as u16);

                Some(Operand::Address(addr))
            }
            AddressingMode::AbsoluteY => {
                let lower_half_addr = self.fetch();
                let upper_half_addr = self.fetch();
                let mut addr = (upper_half_addr as u16) << 8 | lower_half_addr as u16;
                addr = addr.wrapping_add(self.registers.y as u16);

                Some(Operand::Address(addr))
            }
            AddressingMode::Implied => None,
            AddressingMode::Relative => {
                let offset = self.fetch() as u16;
                let next_pc = self.registers.pc + 1;
                let addr: u16;
                if offset < 0x80 {
                    addr = next_pc + offset;
                } else {
                    addr = (next_pc + offset) - 256;
                }

                Some(Operand::Address(addr))
            }
            AddressingMode::IndexedIndirect => {
                let lower_half_addr = self.fetch();
                let base_addr = ((lower_half_addr as u16) + (self.registers.x as u16)) & 0xFF;
                let base_addr_byte = self.bus.read_byte(base_addr) as u16;
                let next_addr_byte = self.bus.read_byte((base_addr + 1) & 0xFF) as u16;
                let addr = base_addr_byte + (next_addr_byte << 8);

                Some(Operand::Address(addr as u16))
            }
            AddressingMode::IndirectIndexed => {
                let lower_half_addr = self.fetch() as u16;
                let next_byte = self.bus.read_byte((lower_half_addr + 1) & 0xFF) as u16;
                let base_addr = (self.bus.read_byte(lower_half_addr) as u16) + (next_byte << 8);
                let addr = base_addr + (self.registers.y as u16);

                Some(Operand::Address(addr))
            }
            AddressingMode::AbsoluteIndirect => {
                let lower_half_addr = self.fetch();
                let upper_half_addr = self.fetch();
                let base_addr = (upper_half_addr as u16) << 8 | lower_half_addr as u16;
                let base_addr_byte = self.bus.read_byte(base_addr) as u16;
                let next_addr = (base_addr & 0xFF00) | (((base_addr & 0xFF) + 1) & 0xFF);
                let next_addr_byte = self.bus.read_byte(next_addr) as u16;

                let addr = base_addr_byte + (next_addr_byte << 8);

                Some(Operand::Address(addr))
            }
        }
    }

    fn fetch(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.advance_pc();

        byte
    }

    fn execute(&mut self, op_code: OpCode, operand: Option<Operand>) {
        match op_code.instruction_type {
            InstructionType::NOP => { /* No operation */ }
            /* Load */
            InstructionType::LDA => {
                if op_code.addressing_mode == AddressingMode::Immediate {
                    self.registers.a = operand.unwrap().unwrap_immediate();
                } else {
                    let addr = operand.unwrap().unwrap_addr();
                    self.registers.a = self.bus.read_byte(addr);
                }
                self.registers.p.set_negative(is_negative(self.registers.a));
                self.registers.p.set_negative(is_zero(self.registers.a));
            }
            InstructionType::LDX => {
                if op_code.addressing_mode == AddressingMode::Immediate {
                    self.registers.x = operand.unwrap().unwrap_immediate();
                } else {
                    let addr = operand.unwrap().unwrap_addr();
                    self.registers.x = self.bus.read_byte(addr);
                }
                self.registers.p.set_negative(is_negative(self.registers.x));
                self.registers.p.set_negative(is_zero(self.registers.x));
            }
            InstructionType::LDY => {
                if op_code.addressing_mode == AddressingMode::Immediate {
                    self.registers.y = operand.unwrap().unwrap_immediate();
                } else {
                    let addr = operand.unwrap().unwrap_addr();
                    self.registers.y = self.bus.read_byte(addr);
                }
                self.registers.p.set_negative(is_negative(self.registers.y));
                self.registers.p.set_negative(is_zero(self.registers.y));
            }
            /* Store */
            InstructionType::STA => {
                self.bus
                    .write_byte(operand.unwrap().unwrap_addr(), self.registers.a);
            }
            InstructionType::STX => {
                self.bus
                    .write_byte(operand.unwrap().unwrap_addr(), self.registers.x);
            }
            InstructionType::STY => {
                self.bus
                    .write_byte(operand.unwrap().unwrap_addr(), self.registers.y);
            }
            /* Flag Controls */
            InstructionType::CLC => {
                self.registers.p.set_carry(false);
            }
            InstructionType::SEC => {
                self.registers.p.set_carry(true);
            }
            InstructionType::CLI => {
                self.registers.p.set_irq_disable(false);
            }
            InstructionType::SEI => {
                self.registers.p.set_irq_disable(true);
            }
            InstructionType::CLD => {
                self.registers.p.set_decimal_mode(false);
            }
            InstructionType::SED => {
                self.registers.p.set_decimal_mode(true);
            }
            InstructionType::CLV => {
                self.registers.p.set_overflow(false);
            }
            /* Branch */
            InstructionType::BCC => {
                if !self.registers.p.carry() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BCS => {
                if self.registers.p.carry() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BEQ => {
                if self.registers.p.zero() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BNE => {
                if !self.registers.p.zero() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BVC => {
                if !self.registers.p.overflow() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BVS => {
                if self.registers.p.overflow() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BPL => {
                if !self.registers.p.negative() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            InstructionType::BMI => {
                if self.registers.p.negative() {
                    self.branch(operand.unwrap().unwrap_addr());
                }
            }
            // Register transfer
            InstructionType::TAX => {
                self.registers.x = self.registers.a;
                self.registers.p.set_zero(is_zero(self.registers.x));
                self.registers.p.set_negative(is_negative(self.registers.x));
            }
            InstructionType::TXA => {
                self.registers.a = self.registers.x;
                self.registers.p.set_zero(is_zero(self.registers.a));
                self.registers.p.set_negative(is_negative(self.registers.a));
            }
            InstructionType::TAY => {
                self.registers.y = self.registers.a;
                self.registers.p.set_zero(is_zero(self.registers.y));
                self.registers.p.set_negative(is_negative(self.registers.y));
            }
            InstructionType::TYA => {
                self.registers.a = self.registers.y;
                self.registers.p.set_zero(is_zero(self.registers.a));
                self.registers.p.set_negative(is_negative(self.registers.a));
            }
            InstructionType::TSX => {
                self.registers.x = self.registers.s;
                self.registers.p.set_zero(is_zero(self.registers.x));
                self.registers.p.set_negative(is_negative(self.registers.x));
            }
            InstructionType::TXS => {
                self.registers.x = self.registers.s;
                // Not Changing flags
            }
            // Increment & Decrement
            InstructionType::INX => {
                self.registers.x = self.registers.x.saturating_add(1);
                self.registers.p.set_zero(is_zero(self.registers.x));
                self.registers.p.set_negative(is_negative(self.registers.x));
            }
            InstructionType::INY => {
                self.registers.y = self.registers.y.saturating_add(1);
                self.registers.p.set_zero(is_zero(self.registers.y));
                self.registers.p.set_negative(is_negative(self.registers.y));
            }
            InstructionType::DEX => {
                self.registers.x = self.registers.x.saturating_sub(1);
                self.registers.p.set_zero(is_zero(self.registers.x));
                self.registers.p.set_negative(is_negative(self.registers.x));
            }
            InstructionType::DEY => {
                self.registers.y = self.registers.y.saturating_sub(1);
                self.registers.p.set_zero(is_zero(self.registers.y));
                self.registers.p.set_negative(is_negative(self.registers.y));
            }
            /* Logic arithmetics */
            InstructionType::AND => {
                let data: u8;
                if op_code.addressing_mode == AddressingMode::Immediate {
                    data = operand.unwrap().unwrap_immediate();
                } else {
                    data = self.bus.read_byte(operand.unwrap().unwrap_addr());
                }

                let result = data & self.registers.a;
                self.registers.p.set_zero(is_zero(result));
                self.registers.p.set_negative(is_negative(result));
                self.registers.a = result;
            }
            InstructionType::ORA => {
                let data: u8;
                if op_code.addressing_mode == AddressingMode::Immediate {
                    data = operand.unwrap().unwrap_immediate();
                } else {
                    data = self.bus.read_byte(operand.unwrap().unwrap_addr());
                }

                let result = data | self.registers.a;
                self.registers.p.set_zero(is_zero(result));
                self.registers.p.set_negative(is_negative(result));
                self.registers.a = result;
            }
            InstructionType::EOR => {
                let data: u8;
                if op_code.addressing_mode == AddressingMode::Immediate {
                    data = operand.unwrap().unwrap_immediate();
                } else {
                    data = self.bus.read_byte(operand.unwrap().unwrap_addr());
                }

                let result = data ^ self.registers.a;
                self.registers.p.set_zero(is_zero(result));
                self.registers.p.set_negative(is_negative(result));
                self.registers.a = result;
            }
            /* Arithmetic */
            InstructionType::ADC => {
                let data: u16;
                if op_code.addressing_mode == AddressingMode::Immediate {
                    data = operand.unwrap().unwrap_immediate() as u16;
                } else {
                    data = self.bus.read_byte(operand.unwrap().unwrap_addr()) as u16;
                }
                let a = self.registers.a as u16;
                let mut operated = data + a;
                if self.registers.p.carry() {
                    operated += 1;
                }
                let overflow = !(((a ^ data) & 0x80) != 0) && ((a ^ operated) & 0x80) != 0;
                self.registers.a = (operated & 0xFF) as u8;
                self.registers.p.set_overflow(overflow);
                self.registers.p.set_carry(operated > 0xFF);
                self.registers.p.set_negative(is_negative(self.registers.a));
                self.registers.p.set_zero(is_zero(self.registers.a));
            }
            // Jump
            InstructionType::JMP => {
                self.registers.pc = operand.unwrap().unwrap_addr();
            }
            unhandled => {
                panic!("Not yet supported: {:?}", unhandled);
            }
        }
    }

    fn branch(&mut self, new_pc: ProgramCounter) {
        self.registers.set_pc(new_pc);
    }

    pub fn cycle_demo(&mut self, cycles: usize) {
        let mut cycle_count = 0;
        while cycle_count <= cycles {
            let tmp_pc = self.registers.pc;
            let opcode = self.fetch_opcode().unwrap();
            let operend = self.fetch_operand(opcode.addressing_mode.clone());
            eprintln!("[0x{:02X}]: {:?} {:?}", tmp_pc, opcode, operend);
            cycle_count += 1;
        }
    }

    fn nmi_interrupt_pc(&self) -> ProgramCounter {
        self.read_interrupt_pc(0xFFFA, 0xFFFB)
    }

    fn reset_interrupt_pc(&self) -> ProgramCounter {
        self.read_interrupt_pc(0xFFFC, 0xFFFD)
    }

    fn irq_interrupt_pc(&self) -> ProgramCounter {
        self.read_interrupt_pc(0xFFFE, 0xFFFF)
    }

    fn brk_interrupt_pc(&self) -> ProgramCounter {
        self.read_interrupt_pc(0xFFFE, 0xFFFF)
    }

    fn read_interrupt_pc(
        &self,
        lower_byte_addr: BusAddr,
        upper_byte_addr: BusAddr,
    ) -> ProgramCounter {
        let lower_byte = self.bus.read_byte(lower_byte_addr);
        let upper_byte = self.bus.read_byte(upper_byte_addr);

        let pc: ProgramCounter = (upper_byte as u16) << 8 | lower_byte as u16;

        pc
    }
}

fn is_negative(value: u8) -> bool {
    value & 0x80 != 0
}

fn is_zero(value: u8) -> bool {
    value == 0
}
