use intcode::{parse_file, IntcodeIterator};
use itertools::Itertools;

fn main() {
	let program = parse_file("inputs/day07.txt").unwrap();
	// use intcode::parse_program;
	// let program = parse_program(
	// 	"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
	// 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
	// )
	// .unwrap();
	// let program = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap();

	// println!("{:?}", program);

	let possibles = [0, 1, 2, 3, 4];
	let best = possibles
		.iter()
		.permutations(5)
		.map(|phase_settings| {
			phase_settings
				.into_iter()
				.map(|&p| IntcodeIterator::new(program.clone()).with_input(p))
				.fold(0, |acc, i| i.with_input(acc).next().unwrap())
		})
		.max()
		.unwrap();

	println!("{}", best);

	let possibles = [5, 6, 7, 8, 9];
	let best = possibles
		.iter()
		.permutations(5)
		.map(|phase_settings| {
			let mut amps: Vec<_> = phase_settings
				.into_iter()
				.map(|&p| IntcodeIterator::new(program.clone()).with_input(p))
				.collect();

			amps[0].add_input(0);
			let mut output = None;
			for current in (0..5).cycle() {
				let next_amp = (current + 1) % 5;
				match amps[current].next() {
					None => {
						output = amps[current].input.pop_back();
						break;
					}
					Some(s) => amps[next_amp].add_input(s),
				}
			}
			output.unwrap()
		})
		.max()
		.unwrap();
	println!("{}", best);
}
