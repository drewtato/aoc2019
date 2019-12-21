const DAY: &str = "inputs/day21.txt";
use intcode::IntcodeProgram;

fn main() {
	let program = IntcodeProgram::vec_from_file(DAY).unwrap();
	let part2 = program.clone();

	let spring1 = "NOT C J\nAND D J\nNOT A T\nOR T J\nWALK\n";
	let spring2 = "NOT E J\nNOT J J\nAND I J\nOR H J\nAND D J\nNOT C T\nNOT T T\nAND B T\nNOT T T\nAND T J\nNOT A T\nOR T J \nRUN\n";

	for (prog, spring) in vec![(program, spring1), (part2, spring2)].into_iter() {
		println!(
			"{}",
			prog.with_input_from(spring.as_bytes().iter().map(|&n| n as i64))
				.last()
				.unwrap()
				.unwrap()
		);
	}
}
