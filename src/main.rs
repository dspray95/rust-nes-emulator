mod cpu;

fn main() {
    println!("Loading and running program...");
    let mut processor = cpu::Cpu::new();
    let program: Vec<u8> = vec![0, 0, 0, 0];
    processor.load_and_run(program);
    println!("...finished!")
}
