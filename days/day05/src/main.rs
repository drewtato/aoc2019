use intcode::{IntcodeProgram, VecMemory};

fn main() {
	let program: IntcodeProgram<VecMemory> = IntcodeProgram::from_file("inputs/day05.txt").unwrap();

	// println!("{:?}", program);

	let end = program.clone().with_input(1).last().unwrap().unwrap();
	println!("{}", end);

	let end = program.with_input(5).last().unwrap().unwrap();
	println!("{}", end);
}
