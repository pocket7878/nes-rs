mod decoder;
pub use decoder::OpCodeDecoder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionType {
    ADC,
    SBC,
    AND,
    ORA,
    EOR,
    ASL,
    LSR,
    ROL,
    ROR,
    BCC,
    BCS,
    BEQ,
    BNE,
    BVC,
    BVS,
    BPL,
    BMI,
    BIT,
    JMP,
    JSR,
    RTS,
    BRK,
    RTI,
    CMP,
    CPX,
    CPY,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    CLC,
    SEC,
    SED,
    CLI,
    SEI,
    CLD,
    CED,
    CLV,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    NOP,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    IndexedIndirect,
    IndirectIndexed,
    AbsoluteIndirect,
}

impl AddressingMode {
    pub fn number_of_operands(&self) -> usize {
        match *self {
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate => 1,
            AddressingMode::Absolute => 2,
            AddressingMode::ZeroPage => 1,
            AddressingMode::ZeroPageX => 1,
            AddressingMode::ZeroPageY => 1,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Implied => 0,
            AddressingMode::Relative => 1,
            AddressingMode::IndexedIndirect => 1,
            AddressingMode::IndirectIndexed => 1,
            AddressingMode::AbsoluteIndirect => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpCode {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    pub fn new(instruction_type: InstructionType, addressing_mode: AddressingMode) -> Self {
        Self {
            instruction_type,
            addressing_mode,
        }
    }

    pub fn number_of_operands(&self) -> usize {
        self.addressing_mode.number_of_operands()
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Address(u16),
    Immediate(u8),
}

impl Operand {
    pub fn unwrap_addr(&self) -> u16 {
        if let Operand::Address(addr) = *self {
            return addr;
        } else {
            panic!("Expected Operand::Address");
        }
    }

    pub fn unwrap_immediate(&self) -> u8 {
        if let Operand::Immediate(data) = *self {
            return data;
        } else {
            panic!("Expected Operand::Immediate");
        }
    }
}
