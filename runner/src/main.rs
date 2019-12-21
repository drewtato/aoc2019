use intcode::{IntcodeProgram, IntcodeTerminal};

fn main() {
	IntcodeTerminal::new(IntcodeProgram::vec_from_file("runner/program.int").unwrap())
		.interactive()
		.unwrap();
}
