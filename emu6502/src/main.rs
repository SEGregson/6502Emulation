pub mod hardware_structs;
pub use hardware_structs::CPU;
pub use hardware_structs::Memory;

fn main() {
    // let test: u8 = 0xa9;
    // println!("{}", test);

    let mut mem = Memory::init();
    let mut cpu = CPU::reset();

    mem.write_data(0xFFFC, 0xA9);
    mem.write_data(0xFFFD, 4);
    //mem.out();
    println!();
    cpu.execute(&mut mem, &mut 2);
    //mem.out();
}
