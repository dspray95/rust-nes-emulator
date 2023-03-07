mod cpu;

fn main() {
    println!("Hello, world!");
    let mut processor = cpu::Cpu::new();
    let program: Vec<u8> = vec![0, 0, 0, 0];
    processor.interpret(program);
}
