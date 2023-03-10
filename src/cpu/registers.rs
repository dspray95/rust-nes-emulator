use bitflags::bitflags;

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

pub struct Register {
    pub program_counter: u16,
    pub pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub processor_status: CpuFlags,
}

impl Register {
    pub fn new() -> Register {
        return Register {
            program_counter: 0,
            pointer: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            processor_status: CpuFlags::from_bits_truncate(0b100100),
        };
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
    }

    pub fn set_carry_flag(&mut self) {
        self.processor_status.insert(CpuFlags::CARRY);
    }

    pub fn clear_carry_flag(&mut self) {
        self.processor_status.remove(CpuFlags::CARRY);
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.processor_status.insert(CpuFlags::ZERO);
        } else {
            self.processor_status.remove(CpuFlags::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.processor_status.insert(CpuFlags::NEGATIVE);
        } else {
            self.processor_status.remove(CpuFlags::NEGATIVE);
        }
    }
}
