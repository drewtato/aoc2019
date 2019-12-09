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
	pub const ADD: i64 = 1;
	pub const MULT: i64 = 2;
	pub const INPUT: i64 = 3;
	pub const OUTPUT: i64 = 4;
	pub const JUMP_NOT_ZERO: i64 = 5;
	pub const JUMP_IS_ZERO: i64 = 6;
	pub const LESS_THAN: i64 = 7;
	pub const EQUAL_TO: i64 = 8;
	pub const ADJUST_REL_BASE: i64 = 9;
	pub const HALT: i64 = 99;

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
		"EQUAL_TO","ADJUST_REL_BASE",
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

pub fn parse_file(filename: &str) -> Result<Vec<i64>, Box<dyn Error>> {
	Ok(parse_program(&std::fs::read_to_string(filename)?)?)
}

pub fn parse_program(prog_str: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
	prog_str
		.trim()
		.split(',')
		.map(|n| n.trim().parse())
		.chain(std::iter::once(Ok(0)).cycle().take(10_000))
		.collect()
}

fn opcode_modes(mut instruction: i64) -> (i64, Vec<i64>) {
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
	code: &mut Vec<i64>,
	input: &mut impl Iterator<Item = i64>,
) -> Result<Vec<i64>, Vec<i64>> {
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
	code: &mut Vec<i64>,
	input: &mut impl Iterator<Item = i64>,
	output: &mut Vec<i64>,
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
			let index = match mode {
				0 => code.get(pos).map(|&x| x as usize),
				1 => Some(pos),
				2 => code.get(pos).map(|&x| (x + code[code.len() - 1]) as usize),
				_ => panic!("Bad mode"),
			};
			if let Some(index) = index {
			let spots_to_add = (index as i64) - (code.len() as i64) + 2;
			if spots_to_add > 0 {
				let relative_base = code[code.len() - 1];
				code.resize(code.len() + spots_to_add as usize, 0);
				let end = code.len() - 1;
				code[end] = relative_base;
			}}
			index
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
		ADJUST_REL_BASE => {
			let rel_base_index = code.len() - 1;
			code[rel_base_index] += code[posns[0]];
			2 + pc
		}
		HALT => return (Ok(None), consumed_input),
		_ => return (Err("Invalid instruction"), consumed_input),
	}));
	(new_pc, consumed_input)
}

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct IntcodeIterator {
	pub program: Vec<i64>,
	pub pc: Option<usize>,
	pub input: VecDeque<i64>,
}

impl IntcodeIterator {
	pub fn new(program: Vec<i64>) -> Self {
		IntcodeIterator {
			program,
			pc: Some(0),
			input: VecDeque::new(),
		}
	}
	pub fn add_input(&mut self, input: i64) {
		self.input.push_back(input);
	}
	pub fn with_input(mut self, input: i64) -> Self {
		self.add_input(input);
		self
	}
	pub fn add_input_iter(&mut self, input: impl IntoIterator<Item = i64>) {
		self.input.extend(input);
	}
	pub fn with_input_iter(mut self, input: impl IntoIterator<Item = i64>) -> Self {
		self.add_input_iter(input);
		self
	}
	pub fn is_halted(&self) -> bool {
		self.pc.is_none()
	}
}

impl Iterator for IntcodeIterator {
	type Item = i64;

	fn next(&mut self) -> Option<Self::Item> {
		let mut output = Vec::new();
		while output.is_empty() {
			// Returns None when halted
			self.pc?;
			let (result, consumed) = run_once(
				&mut self.program,
				&mut self.input.get(0).cloned().into_iter(),
				&mut output,
				self.pc.unwrap(),
			);
			if consumed {
				self.input.pop_front();
			}
			self.pc = result.unwrap();
		}
		output.pop()
	}
}
