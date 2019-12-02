use std::fs::read_to_string;
// use std::io::Write;

const ADD: usize = 1;
const MULT: usize = 2;
const HALT: usize = 99;

fn main() {
	let mut input: Vec<usize> = read_to_string("inputs/day02.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect();

	let input2 = input.clone();
	// Test input
	//                   0 1  2 3 4 5  6 7  8  9 10 11
	// let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];

	// println!("{:?}", input);
	input[1] = 12;
	input[2] = 2;

	run(&mut input);

	println!("{}", input[0]);
	// println!("{:?}", input);

	let desired = 19_690_720;
	input = input2.clone();

	let (mut noun, mut verb) = (0, 0);
	// let mut pic = std::fs::File::create("notes/day02grid.txt").unwrap();
	loop {
		input[1] = noun;
		input[2] = verb;

		run(&mut input);
		// pic.write_all(&format!("{:8} ", input[0]).into_bytes()).unwrap();
		
		if input[0] == desired {
			break;
		}
		
		verb += 1;
		if verb == 100 {
			// pic.write_all(b"\n").unwrap();
			noun += 1;
			if noun == 100 {
				panic!("No good noun-verb pairs");
				// break;
			}
			verb = 0;
		}
		input = input2.clone();
	}
	println!("{}{:02}", noun, verb);
}

fn run(input: &mut Vec<usize>) {
	let mut pc = 0;
	loop {
		match input[pc] {
			ADD => {
				let add = input[input[pc + 1]] + input[input[pc + 2]];
				let i = input[pc + 3];
				input[i] = add;
				pc += 4;
			}
			MULT => {
				let mult = input[input[pc + 1]] * input[input[pc + 2]];
				let i = input[pc + 3];
				input[i] = mult;
				pc += 4;
			}
			HALT => break,
			_ => panic!("Invalid code {}\n{:?}", input[pc], input),
		}
	}
}
