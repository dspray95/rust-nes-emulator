mod registers;
mod memory;

use registers::Register;
use memory::Memory;

pub struct Cpu {
    register: Register,
    memory: Memory
}

impl Cpu {
    pub fn new() -> Cpu{
        return Cpu{
            register: Register::new(),
            memory: Memory::new()
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load_program(program);
        self.register.program_counter = 0x8000;
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.memory.read(self.register.program_counter);
            self.register.program_counter += 1;
            match opscode {
                0x00 => {
                    return;
                },
                0xA9 => {
                    let param = self.memory.read(self.register.program_counter);
                    self.register.program_counter += 1;
                    self.lda(param);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                _ => todo!()
            }
        }

        
    }

    pub fn tax(&mut self){
        //0xAA
        self.register.x = self.register.accumulator;
        self.register.update_zero_and_negative_flags(self.register.x);
    }

    pub fn lda(&mut self, value: u8){
        self.register.accumulator = value;
        self.register.update_zero_and_negative_flags(self.register.accumulator);
    }

    pub fn inx(&mut self){
        self.register.x += 1;
        self.register.update_zero_and_negative_flags(self.register.x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_iimediate_load_data() {
        let mut cpu = Cpu::new();
        //                | 2b instr || exit | 
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        //should load value from next cell into register
        assert_eq!(cpu.register.accumulator, 0x05); 
        //zflag is set
        assert!(cpu.register.processor_status & 0b0000_0010 == 0b00);
        //negative flag is set
        assert!(cpu.register.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        //This makes sure the zflag is set if the valiue in the accumulator is 0
        //is 0
        assert!(cpu.register.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag(){
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0b0000_0011, 0x00]);
        assert!(cpu.register.processor_status & 0b1000_0000 == 0b10);
    }

    #[test]
    fn test_0xaa_tax(){
        let mut cpu = Cpu::new();
        cpu.register.accumulator = 9;
        cpu.interpret(vec![0xAA, 0x00]);
        assert!(cpu.register.x == 9);
    }

    #[test]
    fn test_0xe8_inx(){
        let mut cpu = Cpu::new();
        cpu.register.x = 1;
        cpu.interpret(vec![0xE8, 0x00]);
        assert_eq!(cpu.register.x, 2);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
  
        assert_eq!(cpu.register.x, 0xc1)
    }
 
     #[test]
     fn test_inx_overflow() {
         let mut cpu = Cpu::new();
         cpu.register.x = 0xff;
         cpu.interpret(vec![0xe8, 0xe8, 0x00]);
 
         assert_eq!(cpu.register.x, 1)
     }
}