mod status_register;

pub use status_register::StatusRegister;

pub type ProgramCounter = u16;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: StatusRegister,
    pub pc: ProgramCounter,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            p: StatusRegister::new(),
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.s = 0;
        self.p.reset();
        self.pc = 0;
    }

    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    pub fn set_pc(&mut self, value: ProgramCounter) {
        self.pc = value;
    }
}
