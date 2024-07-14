pub struct CPU {
    i: u16, // I register
    pc: usize, // Program Counter
    mem: [u8; 4096], // Memory
    v: [u8; 16], // 16 Registers, V0-VF
    stack: [u16; 16], // Stack, 16 16-bits
    sp: usize, // Stack Pointer
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

    pub fn load_memory(&mut self, data: &[u8]) {
        // Loads the rom into memory, one byte at a time
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.mem[0x200 + i] = byte;
            } else {
                break;
            }
        }
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
        println!("OPCODE: {:X}", opcode);
        match opcode & 0xF000 {
            0x0000 => self.op_0nnn(opcode),
            0x1000 => self.op_1nnn(opcode),
            0x2000 => self.op_2nnn(opcode),
            0x3000 => self.op_3xnn(opcode),
            0x4000 => self.op_4xnn(opcode),
            0x5000 => self.op_5xy0(opcode),
            0x6000 => self.op_6xnn(opcode),
            0x7000 => self.op_7xnn(opcode),
            0x8000 => self.op_8xyn(opcode),
            _ => println!("Unimplemented opcode: {:X}", opcode)
        }
    }

    fn op_0nnn(&mut self, opcode: u16) {
        match opcode & 0x00FF {
            0x00E0 => println!("Clear screen"),
            0x00EE => println!("Move stack pointer"),
            _ => println!("Unimplemented opcode: {:X}", opcode)
        }
    }

    // 1nnn - JP addr
    //  The interpreter sets the program counter to nnn.
    fn op_1nnn(&mut self, opcode: u16) {
        let nnn = ((opcode & 0x0FFF)) as usize;
        self.pc = nnn
    }

    // 2nnn - CALL addr
    //  The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_2nnn(&mut self, opcode: u16) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        let nnn = ((opcode & 0x0FFF)) as usize;
        self.pc = nnn;   
    }

    // 3xnn - SE Vx, nn
    // The interpreter compares register Vx to nn, and if they are equal, increments the program counter by 2, thus skipping the next instruction
    fn op_3xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        if self.v[x] == nn {
            self.pc += 2;
        }
    }

    // 4xnn - SNE Vx, byte
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_4xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        if self.v[x] != nn {
            self.pc += 2;
        }
    }

    // 5xy0 - SE Vx, Vy
    //  The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_5xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = (opcode & 0x00F0) as usize;
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }
    
    // 6xnn - LD Vx, byte
    // Set Vx = nn.
    fn op_6xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        self.v[x] = nn;
    }

    // 7xnn - DD Vx, byte
    //  Adds the value nn to the value of register Vx, then stores the result in Vx.
    fn op_7xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        self.v[x] += nn;
    }

    fn op_8xyn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4)as usize;
        let n = (opcode & 0x000F) as u16;

        match n {
            //  8xy0 - LD Vx, Vy
            // Set Vx = Vy.
            0 => self.v[x] = self.v[y],
            // 8xy1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            1 => self.v[x] |= self.v[y],
            // 8xy2 - AND Vx, Vy
            // Set Vx = Vx AND Vy
            2 => self.v[x] &= self.v[y],
            // 8xy3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy
            3 => self.v[x] ^= self.v[y],
            // 8xy4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = cary
            4 => {
                let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                }
                self.v[x] = result;
            },
            // 8xy5 - Sub Vx, Vy
            //  Set Vx = Vx - Vy, set VF if Vx is greater than Vy
            5 => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] -= self.v[y];
            },
            // 8xy6 - SHR Vx {, Vy}
            // Set Vx = Vx SHR 1, set VF to match least significant bit of Vx first
            6 => {
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            },
            // 8xy7 SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF if Vy is greater than Vx
            7 => {
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[y] - self.v[x]
            }
           _ => println!("Unimplemented opcode: {:X}", opcode)
        }

    }


}
