pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            memory: [0; 0xFFFF],
        };
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn read_u16(&self, pos: u16) -> u16 {
        let lo = self.read(pos) as u16;
        let hi = self.read(pos + 1) as u16;
        return (hi << 8) | (lo as u16);
    }

    pub fn write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.write(pos, lo);
        self.write(pos + 1, hi);
    }
}
