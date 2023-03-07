pub struct Register { 

    pub program_counter: u16,
    pub pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub processor_status: u8

}

impl Register {

    pub fn new() -> Register{
        return Register{
            program_counter: 0,
            pointer: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            processor_status: 0
        };
    }


    pub fn update_zero_and_negative_flags(&mut self, result: u8){
        if result == 0 {
            self.processor_status = self.processor_status | 0b0000_0010;
        } else {
            self.processor_status = self.processor_status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.processor_status = self.processor_status | 0b1000_0000;
        } else {
            self.processor_status = self.processor_status & 0b0111_1111;
        }
    }

}