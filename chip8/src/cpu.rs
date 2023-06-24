pub struct CPU {
    i: u16, // I register
    pc: usize, // Program Counter
    mem: [u8; 4096], // Memory
    v: [u8; 16], // 16 Registers, V0-VF
    stack: [u16; 16], // Stack, 16 16-bits
    sp: u8, // Stack Pointer
    dt: u8 // Delay Timer
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            pc: 0x200, // Start PC at address 200
            stack: [0;16],
            sp: 0,
            v: [0;16],
            i: 0x200,
            mem: [0;4096],
            dt: 0,
        }

    }

    pub fn load_memory(&mut self) {
        self.mem[0x200] = 0xA2;
        self.mem[0x201] = 0xF0;
    }
    pub fn run_cycle(&mut self) {
        // Get the next opcode
        // Since opcodes are 16 bits, get two consecutive values from memory
        // The first instruction is bitshifted to the left to fill the first 8 bits
        // Then OR-ed with the second instruction to fill the second 8 bits
        let opcode = (self.mem[self.pc] as u16) << 8 | (self.mem[self.pc + 1] as u16);

        self.execute_opcode(opcode);
    }

    fn execute_opcode(&mut self, opcode: u16) {
        println!("{}", opcode);
    }
}
