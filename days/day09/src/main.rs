const DAY: u8 = 9;
use intcode::{parse_file, IntcodeIterator};

fn main() {
	let mut program =
		IntcodeIterator::new(parse_file(&format!("inputs/day{:02}.txt", DAY)).unwrap());

	// println!("{:?}", program);

	let mut program1 = program.clone().with_input(1);
	println!("{}", program1.next().unwrap());
	// eprintln!("Length: {}, {:?}", program1.program.len(), &program1.program[program1.program.len() - 10 ..]);

	program.add_input(2);
	println!("{}", program.next().unwrap());
	// eprintln!("Length: {}, {:?}", program.program.len(), &program.program[program.program.len() - 10 ..]);
}
