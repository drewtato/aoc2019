use intcode::{parse_file, run, run_once};
use itertools::Itertools;
use std::iter::once;

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
	let mut best = 0;
	for phase_settings in possibles.iter().permutations(5) {
		let amps: Vec<Vec<isize>> = (0..5).map(|_| program.clone()).collect();
		// println!("{:?}", inputs);
		let mut last_output = 0;
		for (mut amp, &phase_setting) in amps.into_iter().zip(phase_settings) {
			let out = run(&mut amp, &mut once(phase_setting).chain(once(last_output))).unwrap();
			last_output = out[0];
		}
		best = *[best, last_output].iter().max().unwrap();
	}
	println!("{}", best);

	let possibles = [5, 6, 7, 8, 9];
	let mut best = 0;
	for phase_settings in possibles.iter().permutations(5) {
		// println!("{:?}", phase_settings);
		let mut amps_input_pc: Vec<_> = phase_settings
			.into_iter()
			.map(|&p| (program.clone(), vec![p].into_iter(), Some(0)))
			.collect();
		let mut current_index = 0;
		let mut cur_amp = &mut amps_input_pc[current_index];

		let mut tmp_input: Vec<_> = cur_amp.1.clone().collect();
		tmp_input.push(0);
		cur_amp.1 = tmp_input.into_iter();
		let mut out = Vec::new();
		// println!("{} {}", current, next());
		while let Some(pc) = cur_amp.2 {
			// println!("{}", current_index);
			cur_amp.2 = run_once(&mut cur_amp.0, &mut cur_amp.1, &mut out, pc).unwrap();

			if !out.is_empty() {
				current_index = (current_index + 1) % 5;
				cur_amp = &mut amps_input_pc[current_index];
				let mut tmp_input: Vec<_> = cur_amp.1.clone().collect();
				tmp_input.push(out.pop().unwrap());
				cur_amp.1 = tmp_input.into_iter();
			}
		}
		best = *[best, cur_amp.1.next().unwrap()].iter().max().unwrap();
	}
	println!("{}", best);
}
