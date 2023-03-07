mod memory;
mod opcodes;
mod registers;

use memory::Memory;
use registers::Register;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

pub struct Cpu {
    register: Register,
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            register: Register::new(),
            memory: Memory::new(),
        };
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load_program(program);
        self.memory.write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register.accumulator = 0;
        self.register.x = 0;
        self.register.y = 0;
        self.register.processor_status = 0;

        self.register.program_counter = self.memory.read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let ref opcodes = *opcodes::OPCODES_MAP;
        loop {
            let code = self.memory.read(self.register.program_counter);
            self.register.program_counter += 1;
            let program_counter_state = self.register.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                //LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }
                //STA
                0x85 | 0x96 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),
                //AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => {
                    return;
                }
                _ => todo!(),
            }

            if program_counter_state == self.register.program_counter {
                self.register.program_counter += (opcode.len - 1) as u16;
            }
        }
    }

    pub fn get_operand_anddress(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.register.program_counter,
            AddressingMode::ZeroPage => self.memory.read(self.register.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.memory.read(self.register.program_counter);
                let addr = pos.wrapping_add(self.register.x) as u16;
                return addr;
            }
            AddressingMode::ZeroPageY => {
                let pos = self.memory.read(self.register.program_counter);
                let addr = pos.wrapping_add(self.register.y) as u16;
                return addr;
            }
            AddressingMode::Absolute => self.memory.read_u16(self.register.program_counter),
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_u16(self.register.program_counter);
                let addr = base.wrapping_add(self.register.x as u16);
                return addr;
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_u16(self.register.program_counter);
                let addr = base.wrapping_add(self.register.y as u16);
                return addr;
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.register.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register.x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.register.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register.y);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn and(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_anddress(mode);
        let data = self.memory.read(address);
        return self
            .register
            .set_accumulator(data & self.register.accumulator);
    }

    pub fn tax(&mut self) {
        //0xAA
        self.register.x = self.register.accumulator;
        self.register
            .update_zero_and_negative_flags(self.register.x);
    }

    pub fn lda(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_anddress(mode);
        let value = self.memory.read(address);

        self.register.accumulator = value;
        self.register
            .update_zero_and_negative_flags(self.register.accumulator);
    }

    pub fn inx(&mut self) {
        self.register.x = self.register.x.wrapping_add(1);
        self.register
            .update_zero_and_negative_flags(self.register.x);
    }

    pub fn sta(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_anddress(mode);
        self.memory.write(address, self.register.accumulator);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = Cpu::new();
        //| 2b instr || exit |
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        //should load value from next cell into register
        assert_eq!(cpu.register.accumulator, 5);
        //zflag is set
        assert!(cpu.register.processor_status & 0b0000_0010 == 0);
        //negative flag is set
        assert!(cpu.register.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        //This makes sure the zflag is set if the valiue in the accumulator is 0
        //is 0
        assert!(cpu.register.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.register.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register.x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register.x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = Cpu::new();
        cpu.memory.write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register.accumulator, 0x55);
    }
}
