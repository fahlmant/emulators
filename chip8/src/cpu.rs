
pub struct CPU {
    i: u16, // I register
    pc: u16, // Program Counter
    ram: [u8; 4096], // Memory
    v: [u8; 16], // 16 Registers V0-VF
    stack: [u16; 16], // Stack, 16 16-bits
    sp: u8, // Stack Pointer
    dt: u8 // Delay Timer
}

impl CPU {
    pub fn execute_cycle(&mut self) {
         // Get the next opcode

         // Run opcode
    }

    pub fn run_opcode(&mut self, opcode: u16) {
        
    }
}