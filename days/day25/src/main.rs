const DAY: &str = "inputs/day25.txt";
use intcode::{IntcodeProgram, IntcodeTerminal};

fn main() {
	IntcodeTerminal::new(IntcodeProgram::vec_from_file(DAY).unwrap()).interactive().unwrap();
}
