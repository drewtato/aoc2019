use intcode::{parse_file, IntcodeIterator};

fn main() {
	let program = parse_file("inputs/day05.txt").unwrap();

	// println!("{:?}", program);

	let end = IntcodeIterator::new(program.clone()).with_input(1).last().unwrap();
	println!("{}", end);
	
	let end = IntcodeIterator::new(program).with_input(5).last().unwrap();
	println!("{}", end);
}
