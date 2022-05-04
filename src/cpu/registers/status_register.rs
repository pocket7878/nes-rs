pub struct StatusRegister {
    negative: bool,
    overflow: bool,
    reserved: bool,
    break_mode: bool,
    decimal_mode: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool,
}

impl StatusRegister {
    pub fn new() -> Self {
        Self {
            negative: false,
            overflow: false,
            reserved: true,
            break_mode: false,
            decimal_mode: false,
            irq_disable: false,
            zero: false,
            carry: false,
        }
    }

    pub fn reset(&mut self) {
        self.negative = false;
        self.overflow = false;
        self.reserved = true;
        self.break_mode = false;
        self.decimal_mode = false;
        self.irq_disable = false;
        self.zero = false;
        self.carry = false;
    }

    pub fn negative(&self) -> bool {
        self.negative
    }

    pub fn set_negative(&mut self, value: bool) {
        self.negative = value;
    }

    pub fn overflow(&self) -> bool {
        self.overflow
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    pub fn reserved(&self) -> bool {
        self.reserved
    }

    pub fn break_mode(&self) -> bool {
        self.break_mode
    }

    pub fn set_break_mode(&mut self, value: bool) {
        self.break_mode = value;
    }

    pub fn decimal_mode(&self) -> bool {
        self.decimal_mode
    }

    pub fn set_decimal_mode(&mut self, value: bool) {
        self.decimal_mode = value;
    }

    pub fn irq_disable(&self) -> bool {
        self.irq_disable
    }

    pub fn set_irq_disable(&mut self, value: bool) {
        self.irq_disable = value;
    }

    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}

pub struct Registers {}
