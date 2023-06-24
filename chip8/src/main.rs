mod cpu;
use cpu::CPU;

fn main() {

    // Create a new initalized CPU
    let mut cpu = CPU::new();
    // Load ROM into CPU memory
    cpu.load_memory();

    // Run CPU
    // TODO: For now only runs a single cycle
    cpu.run_cycle();
}
