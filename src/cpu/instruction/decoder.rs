use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::{AddressingMode, InstructionType, OpCode};

static DECODE_TABLE: Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
    let table_vec: Vec<(u8, InstructionType, AddressingMode)> = vec![
        // 0xX0
        (0x00, InstructionType::BRK, AddressingMode::Implied),
        (0x10, InstructionType::BPL, AddressingMode::Relative),
        (0x20, InstructionType::JSR, AddressingMode::Absolute),
        (0x30, InstructionType::BMI, AddressingMode::Relative),
        (0x40, InstructionType::RTI, AddressingMode::Implied),
        (0x50, InstructionType::BVC, AddressingMode::Relative),
        (0x60, InstructionType::RTS, AddressingMode::Implied),
        (0x70, InstructionType::BVS, AddressingMode::Relative),
        (0x90, InstructionType::BCC, AddressingMode::Relative),
        (0xA0, InstructionType::LDY, AddressingMode::Immediate),
        (0xB0, InstructionType::BCS, AddressingMode::Relative),
        (0xC0, InstructionType::CPY, AddressingMode::Immediate),
        (0xD0, InstructionType::BNE, AddressingMode::Relative),
        (0xE0, InstructionType::CPX, AddressingMode::Immediate),
        (0xF0, InstructionType::BEQ, AddressingMode::Relative),
        // 0xX1
        (0x01, InstructionType::ORA, AddressingMode::IndexedIndirect),
        (0x11, InstructionType::ORA, AddressingMode::IndirectIndexed),
        (0x21, InstructionType::AND, AddressingMode::IndexedIndirect),
        (0x31, InstructionType::AND, AddressingMode::IndirectIndexed),
        (0x41, InstructionType::EOR, AddressingMode::IndexedIndirect),
        (0x51, InstructionType::EOR, AddressingMode::IndirectIndexed),
        (0x61, InstructionType::ADC, AddressingMode::IndexedIndirect),
        (0x71, InstructionType::ADC, AddressingMode::IndirectIndexed),
        (0x81, InstructionType::STA, AddressingMode::IndexedIndirect),
        (0x91, InstructionType::STA, AddressingMode::IndirectIndexed),
        (0xA1, InstructionType::LDA, AddressingMode::IndexedIndirect),
        (0xB1, InstructionType::LDA, AddressingMode::IndirectIndexed),
        (0xC1, InstructionType::CMP, AddressingMode::IndexedIndirect),
        (0xD1, InstructionType::CMP, AddressingMode::IndirectIndexed),
        (0xE1, InstructionType::SBC, AddressingMode::IndexedIndirect),
        (0xF1, InstructionType::SBC, AddressingMode::IndirectIndexed),
        // 0xX2
        (0xA2, InstructionType::LDA, AddressingMode::Immediate),
        // 0xX4
        (0x24, InstructionType::BIT, AddressingMode::ZeroPage),
        (0x84, InstructionType::STY, AddressingMode::ZeroPage),
        (0x94, InstructionType::STY, AddressingMode::ZeroPageX),
        (0xA4, InstructionType::LDY, AddressingMode::ZeroPage),
        (0xB4, InstructionType::LDY, AddressingMode::ZeroPageX),
        (0xC4, InstructionType::CPY, AddressingMode::ZeroPage),
        (0xE4, InstructionType::CPX, AddressingMode::ZeroPage),
        // 0xX5
        (0x05, InstructionType::ORA, AddressingMode::ZeroPage),
        (0x15, InstructionType::ORA, AddressingMode::ZeroPageX),
        (0x25, InstructionType::AND, AddressingMode::ZeroPage),
        (0x35, InstructionType::AND, AddressingMode::ZeroPageX),
        (0x45, InstructionType::EOR, AddressingMode::ZeroPage),
        (0x55, InstructionType::EOR, AddressingMode::ZeroPageX),
        (0x65, InstructionType::ADC, AddressingMode::ZeroPage),
        (0x75, InstructionType::ADC, AddressingMode::ZeroPageX),
        (0x85, InstructionType::STA, AddressingMode::ZeroPage),
        (0x95, InstructionType::STA, AddressingMode::ZeroPageX),
        (0xA5, InstructionType::LDA, AddressingMode::ZeroPage),
        (0xB5, InstructionType::LDA, AddressingMode::ZeroPageX),
        (0xC5, InstructionType::CMP, AddressingMode::ZeroPage),
        (0xD5, InstructionType::CMP, AddressingMode::ZeroPageX),
        (0xE5, InstructionType::SBC, AddressingMode::ZeroPage),
        (0xF5, InstructionType::SBC, AddressingMode::ZeroPageX),
        // 0x06
        (0x06, InstructionType::ASL, AddressingMode::ZeroPage),
        (0x16, InstructionType::ASL, AddressingMode::ZeroPageX),
        (0x26, InstructionType::ROL, AddressingMode::ZeroPage),
        (0x36, InstructionType::ROL, AddressingMode::ZeroPageX),
        (0x46, InstructionType::LSR, AddressingMode::ZeroPage),
        (0x56, InstructionType::LSR, AddressingMode::ZeroPageX),
        (0x66, InstructionType::ROR, AddressingMode::ZeroPage),
        (0x76, InstructionType::ROR, AddressingMode::ZeroPageX),
        (0x86, InstructionType::STX, AddressingMode::ZeroPage),
        (0x96, InstructionType::STX, AddressingMode::ZeroPageY),
        (0xA6, InstructionType::LDX, AddressingMode::ZeroPage),
        (0xB6, InstructionType::LDX, AddressingMode::ZeroPageY),
        (0xC6, InstructionType::DEC, AddressingMode::ZeroPage),
        (0xD6, InstructionType::DEC, AddressingMode::ZeroPageX),
        (0xE6, InstructionType::INC, AddressingMode::ZeroPage),
        (0xF6, InstructionType::INC, AddressingMode::ZeroPageX),
        // 0xX8
        (0x08, InstructionType::PHP, AddressingMode::Implied),
        (0x18, InstructionType::CLC, AddressingMode::Implied),
        (0x28, InstructionType::PLP, AddressingMode::Implied),
        (0x38, InstructionType::SEC, AddressingMode::Implied),
        (0x48, InstructionType::PHA, AddressingMode::Implied),
        (0x58, InstructionType::CLI, AddressingMode::Implied),
        (0x68, InstructionType::PLA, AddressingMode::Implied),
        (0x78, InstructionType::SEI, AddressingMode::Implied),
        (0x88, InstructionType::DEY, AddressingMode::Implied),
        (0x98, InstructionType::TYA, AddressingMode::Implied),
        (0xA8, InstructionType::TAY, AddressingMode::Implied),
        (0xB8, InstructionType::CLV, AddressingMode::Implied),
        (0xC8, InstructionType::INY, AddressingMode::Implied),
        (0xD8, InstructionType::CLD, AddressingMode::Implied),
        (0xE8, InstructionType::INX, AddressingMode::Implied),
        (0xF8, InstructionType::SED, AddressingMode::Implied),
        // 0xX9
        (0x09, InstructionType::ORA, AddressingMode::Immediate),
        (0x19, InstructionType::ORA, AddressingMode::AbsoluteY),
        (0x29, InstructionType::AND, AddressingMode::Immediate),
        (0x39, InstructionType::AND, AddressingMode::AbsoluteY),
        (0x49, InstructionType::EOR, AddressingMode::Immediate),
        (0x59, InstructionType::EOR, AddressingMode::AbsoluteY),
        (0x69, InstructionType::ADC, AddressingMode::Immediate),
        (0x79, InstructionType::ADC, AddressingMode::AbsoluteY),
        (0x89, InstructionType::STA, AddressingMode::Immediate),
        (0x99, InstructionType::STA, AddressingMode::AbsoluteY),
        (0xA9, InstructionType::LDA, AddressingMode::Immediate),
        (0xB9, InstructionType::LDA, AddressingMode::AbsoluteY),
        (0xC9, InstructionType::CMP, AddressingMode::Immediate),
        (0xD9, InstructionType::CMP, AddressingMode::AbsoluteY),
        (0xE9, InstructionType::SBC, AddressingMode::Immediate),
        (0xF9, InstructionType::SBC, AddressingMode::AbsoluteY),
        // 0xXA
        (0x0A, InstructionType::ASL, AddressingMode::Accumulator),
        (0x2A, InstructionType::ROL, AddressingMode::Accumulator),
        (0x4A, InstructionType::LSR, AddressingMode::Accumulator),
        (0x6A, InstructionType::ROR, AddressingMode::Accumulator),
        (0x8A, InstructionType::TXA, AddressingMode::Implied),
        (0x9A, InstructionType::TXS, AddressingMode::Implied),
        (0xAA, InstructionType::TAX, AddressingMode::Implied),
        (0xBA, InstructionType::TSX, AddressingMode::Implied),
        (0xCA, InstructionType::DEX, AddressingMode::Implied),
        (0xEA, InstructionType::NOP, AddressingMode::Implied),
        // 0xXC
        (0x2C, InstructionType::BIT, AddressingMode::Absolute),
        (0x4C, InstructionType::JMP, AddressingMode::Absolute),
        (0x6C, InstructionType::JMP, AddressingMode::AbsoluteIndirect),
        (0x8C, InstructionType::STY, AddressingMode::Absolute),
        (0xAC, InstructionType::LDY, AddressingMode::Absolute),
        (0xBC, InstructionType::LDY, AddressingMode::AbsoluteX),
        (0xCC, InstructionType::CPY, AddressingMode::Absolute),
        (0xEC, InstructionType::CPX, AddressingMode::Absolute),
        // 0xXD
        (0x0d, InstructionType::ORA, AddressingMode::Absolute),
        (0x1D, InstructionType::ORA, AddressingMode::AbsoluteX),
        (0x2D, InstructionType::AND, AddressingMode::Absolute),
        (0x3D, InstructionType::AND, AddressingMode::AbsoluteX),
        (0x4D, InstructionType::EOR, AddressingMode::Absolute),
        (0x5D, InstructionType::EOR, AddressingMode::AbsoluteX),
        (0x6D, InstructionType::ADC, AddressingMode::Absolute),
        (0x7D, InstructionType::ADC, AddressingMode::AbsoluteX),
        (0x8D, InstructionType::STA, AddressingMode::Absolute),
        (0x9D, InstructionType::STA, AddressingMode::AbsoluteX),
        (0xAD, InstructionType::LDA, AddressingMode::Absolute),
        (0xBD, InstructionType::LDA, AddressingMode::AbsoluteX),
        (0xCD, InstructionType::CMP, AddressingMode::Absolute),
        (0xDD, InstructionType::CMP, AddressingMode::AbsoluteX),
        (0xED, InstructionType::SBC, AddressingMode::Absolute),
        (0xFD, InstructionType::SBC, AddressingMode::AbsoluteX),
        // 0xXE
        (0x0E, InstructionType::ASL, AddressingMode::Absolute),
        (0x1E, InstructionType::ASL, AddressingMode::AbsoluteX),
        (0x2E, InstructionType::ROL, AddressingMode::Absolute),
        (0x3E, InstructionType::ROL, AddressingMode::AbsoluteX),
        (0x4E, InstructionType::LSR, AddressingMode::Absolute),
        (0x5E, InstructionType::LSR, AddressingMode::AbsoluteX),
        (0x6E, InstructionType::ROR, AddressingMode::Absolute),
        (0x7E, InstructionType::ROR, AddressingMode::AbsoluteX),
        (0x8E, InstructionType::STX, AddressingMode::Absolute),
        (0xAE, InstructionType::LDX, AddressingMode::Absolute),
        (0xBE, InstructionType::LDX, AddressingMode::AbsoluteX),
        (0xCE, InstructionType::DEC, AddressingMode::Absolute),
        (0xDE, InstructionType::DEC, AddressingMode::AbsoluteX),
        (0xEE, InstructionType::INC, AddressingMode::Absolute),
        (0xFE, InstructionType::INC, AddressingMode::AbsoluteX),
    ];

    let mut table = HashMap::new();
    for (type_u8, itype, amode) in table_vec.into_iter() {
        table.insert(type_u8, OpCode::new(itype, amode));
    }

    table
});

pub struct OpCodeDecoder;

impl OpCodeDecoder {
    pub fn decode(opcode: u8) -> Option<OpCode> {
        DECODE_TABLE.get(&opcode).cloned()
    }
}
