#[cfg(test)]
mod tests {
	use crate::consts::*;

	#[test]
	fn name_strs() {
		assert_eq!("ADD", NAMES[ADD as usize]);
		assert_eq!("MULT", NAMES[MULT as usize]);
		assert_eq!("INPUT", NAMES[INPUT as usize]);
		assert_eq!("OUTPUT", NAMES[OUTPUT as usize]);
		assert_eq!("JUMP_NOT_ZERO", NAMES[JUMP_NOT_ZERO as usize]);
		assert_eq!("JUMP_IS_ZERO", NAMES[JUMP_IS_ZERO as usize]);
		assert_eq!("LESS_THAN", NAMES[LESS_THAN as usize]);
		assert_eq!("EQUAL_TO", NAMES[EQUAL_TO as usize]);
		assert_eq!("HALT", NAMES[HALT as usize]);
	}

	mod programs {
		use crate::{parse_file, parse_program, run};

		#[test]
		fn add_mul() {
			let mut program = parse_file("day02.txt").unwrap();
			let mut program2 = program.clone();

			program[1] = 12;
			program[2] = 2;
			let res = run(&mut program, &mut None.into_iter());
			assert!(res.unwrap().is_empty());
			assert_eq!(program[0], 3_790_645);

			program2[1] = 65;
			program2[2] = 77;
			let res = run(&mut program2, &mut None.into_iter());
			assert!(res.unwrap().is_empty());
			assert_eq!(program2[0], 19_690_720);
		}

		#[test]
		fn in_out_modes() {
			let mut program = parse_file("day05.txt").unwrap();
			let res = run(&mut program, &mut vec![1].into_iter());
			assert_eq!(&res.unwrap(), &[0, 0, 0, 0, 0, 0, 0, 0, 0, 5_346_030]);
		}

		#[test]
		fn jumps() {
			let mut program = parse_file("day05.txt").unwrap();
			let res = run(&mut program, &mut vec![5].into_iter());
			assert_eq!(&res.unwrap(), &[513_116]);
		}

		#[test]
		fn not_enough_input() {
			let mut program = parse_program("3,0").unwrap();
			let res = run(&mut program, &mut None.into_iter());
			assert!(res.is_err());
		}
	}
}

use consts::*;
pub mod consts {
	pub const ADD: isize = 1;
	pub const MULT: isize = 2;
	pub const INPUT: isize = 3;
	pub const OUTPUT: isize = 4;
	pub const JUMP_NOT_ZERO: isize = 5;
	pub const JUMP_IS_ZERO: isize = 6;
	pub const LESS_THAN: isize = 7;
	pub const EQUAL_TO: isize = 8;
	pub const HALT: isize = 99;

	#[rustfmt::skip]
	pub const NAMES: [&str; 100] = [
		"",
		"ADD",
		"MULT",
		"INPUT",
		"OUTPUT",
		"JUMP_NOT_ZERO",
		"JUMP_IS_ZERO",
		"LESS_THAN",
		"EQUAL_TO","",
		"","","","","","","","","","", // 10-19
		"","","","","","","","","","", // 20-29
		"","","","","","","","","","", // 30-39
		"","","","","","","","","","", // 40-49
		"","","","","","","","","","", // 50-59
		"","","","","","","","","","", // 60-69
		"","","","","","","","","","", // 70-79
		"","","","","","","","","","", // 80-89
		"","","","","","","","","","HALT"
	];
}

use std::error::Error;

pub fn parse_file(filename: &str) -> Result<Vec<isize>, Box<dyn Error>> {
	Ok(parse_program(&std::fs::read_to_string(filename)?)?)
}

pub fn parse_program(prog_str: &str) -> Result<Vec<isize>, std::num::ParseIntError> {
	prog_str
		.trim()
		.split(',')
		.map(|n| n.trim().parse())
		.collect()
}

fn opcode_modes(mut instruction: isize) -> (isize, Vec<isize>) {
	let mut modes = Vec::with_capacity(3);
	let op = instruction % 100;
	instruction /= 100;
	for _ in 0..3 {
		modes.push(instruction % 10);
		instruction /= 10;
	}
	(op, modes)
}

/// Runs an intcode program consisting of an array of integers. Takes input as an array of integers
/// and returns output as an array of integers.
pub fn run(
	code: &mut [isize],
	input: &mut impl Iterator<Item = isize>,
) -> Result<Vec<isize>, Vec<isize>> {
	let mut pc = 0;
	let mut output = Vec::new();
	loop {
		let (result, _) = run_once(code, input, &mut output, pc);
		match result {
			Ok(Some(new_pc)) => pc = new_pc,
			Ok(None) => break,
			Err(e) => {
				eprintln!("Error `{}` on {} at {}", e, NAMES[code[pc] as usize], pc);
				return Err(output);
			}
		}
	}
	Ok(output)
}

pub fn run_once(
	code: &mut [isize],
	input: &mut impl Iterator<Item = isize>,
	output: &mut Vec<isize>,
	pc: usize,
	// Ok(Some((new pc, consumed input))), Ok(None) = Halted
) -> (Result<Option<usize>, &'static str>, bool) {
	let (op, modes) = opcode_modes(code[pc]);
	let mut consumed_input = false;

	// A vec of positions in `code`.
	let posns: Vec<_> = modes
		.iter()
		.enumerate()
		.filter_map(|(i, &mode)| {
			let pos = pc + i + 1;
			if mode == 0 {
				code.get(pos).map(|&x| x as usize)
			} else {
				Some(pos)
			}
		})
		.collect();

	// eprintln!("{} {:?} {} {:?} {:?}", pc, &code[pc..pc + 4], op, modes, posns);

	let new_pc = Ok(Some(match op {
		ADD => {
			code[posns[2]] = code[posns[0]] + code[posns[1]];
			4 + pc
		}
		MULT => {
			code[posns[2]] = code[posns[0]] * code[posns[1]];
			4 + pc
		}
		INPUT => {
			code[posns[0]] = match input.next() {
				Some(x) => x,
				None => return (Err("Not enough inputs"), consumed_input),
			};
			consumed_input = true;
			2 + pc
		}
		OUTPUT => {
			output.push(code[posns[0]]);
			2 + pc
		}
		JUMP_NOT_ZERO => {
			if code[posns[0]] != 0 {
				code[posns[1]] as usize
			} else {
				pc + 3
			}
		}
		JUMP_IS_ZERO => {
			if code[posns[0]] == 0 {
				code[posns[1]] as usize
			} else {
				pc + 3
			}
		}
		LESS_THAN => {
			code[posns[2]] = if code[posns[0]] < code[posns[1]] {
				1
			} else {
				0
			};
			4 + pc
		}
		EQUAL_TO => {
			code[posns[2]] = if code[posns[0]] == code[posns[1]] {
				1
			} else {
				0
			};
			4 + pc
		}
		HALT => return (Ok(None), consumed_input),
		_ => return (Err("Invalid instruction"), consumed_input),
	}));
	(new_pc, consumed_input)
}

use std::collections::VecDeque;

pub struct IntcodeIterator {
	pub program: Vec<isize>,
	pub pc: usize,
	pub input: VecDeque<isize>,
}

impl IntcodeIterator {
	pub fn new(program: Vec<isize>) -> Self {
		IntcodeIterator {
			program,
			pc: 0,
			input: VecDeque::new(),
		}
	}
	pub fn add_input(&mut self, input: isize) {
		self.input.push_back(input);
	}
	pub fn add_input_iter(&mut self, input: impl IntoIterator<Item = isize>) {
		self.input.extend(input);
	}
}

impl Iterator for IntcodeIterator {
	type Item = isize;

	fn next(&mut self) -> Option<Self::Item> {
		let mut output = Vec::new();
		while output.is_empty() {
			let (result, consumed) = run_once(
				&mut self.program,
				&mut self.input.get(0).cloned().into_iter(),
				&mut output,
				self.pc,
			);
			if consumed {
				self.input.pop_front();
			}
			match result.unwrap() {
				None => return None,
				Some(pc) => {
					self.pc = pc;
				}
			}
		}
		output.pop()
	}
}
