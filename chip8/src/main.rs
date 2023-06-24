use std::fs::File;
use std::io::prelude::*;

mod cpu;
use cpu::CPU;

fn main() {

    // Load ROM file
    let mut file_buffer = [0u8; 3584];
    let mut f = File::open("pong.rom").expect("Cannot find file");

    let _ = f.read(&mut file_buffer).expect("Something went wrong");

    // Create a new initalized CPU
    let mut cpu = CPU::new();
    // Load ROM into CPU memory
    cpu.load_memory(&file_buffer);

    // Run CPU
    // TODO: For now only runs a single cycle
    cpu.run_cycle();
}
