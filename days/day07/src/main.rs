use intcode::{IntcodeProgram, VecMemory};
use itertools::Itertools;

fn main() {
	let program: IntcodeProgram<VecMemory> = IntcodeProgram::from_file("inputs/day07.txt").unwrap();
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
				.map(|&p| IntcodeProgram::new(program.clone()).with_input(p))
				.fold(0, |acc, i| i.with_input(acc).next().unwrap().unwrap())
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
				.map(|&p| IntcodeProgram::new(program.clone()).with_input(p))
				.collect();

			(0..5)
				.cycle()
				.map(|n| (&mut amps[n]) as *mut IntcodeProgram<_>)
				// Unsafe: We need to check if the machine has halted and run the machine.
				// We aren't deleting the amps until later, and these operations don't happen at
				// the same time, so it is safe.
				.take_while(|&cur| unsafe { !(*cur).halted() })
				.fold(0, |acc, cur| unsafe {
					(*cur).add_input(acc);
					(*cur).next().unwrap_or(Ok(acc)).unwrap()
				})
		})
		.max()
		.unwrap();

	println!("{}", best);
}
