use crate::{
	Data, Indexer,
	IntcodeError::{self, *},
	Memory,
};
use std::{
	collections::VecDeque,
	ops::{Index, IndexMut},
};

macro_rules! constants {
	( $( $name:ident = $code:literal; )* ) => {
		$(
			const $name: usize = $code;
		)*
	};
}

constants! {
	ADD = 1;
	MUL = 2;
	INP = 3;
	OUT = 4;
	JIT = 5;
	JIF = 6;
	LES = 7;
	EQU = 8;
	ADJ = 9;
	HLT = 99;
	POSITION = 0;
	IMMEDIATE = 1;
	RELATIVE = 2;
}

#[derive(Debug, Clone)]
pub struct IntcodeProgram<M: Memory> {
	mem: M,
	pc: Indexer,
	rel: Data,
	input: VecDeque<Data>,
	halted: bool,
	exploded: bool,
}

impl<M: Memory> IntcodeProgram<M> {
	pub fn new(program: M) -> Self {
		Self {
			mem: program,
			pc: 0,
			rel: 0,
			input: VecDeque::new(),
			halted: false,
			exploded: false,
		}
	}

	pub fn step(&mut self) -> Result<Option<Data>, IntcodeError> {
		if self.halted {
			return if self.exploded {
				Err(Exploded)
			} else {
				Err(Halted)
			};
		}
		let (arg_indexes, instruction) = self.separate_instruction()?;

		let mut output = None;
		let new_pc = match instruction {
			ADD => {
				self[arg_indexes[2]] = self[arg_indexes[0]] + self[arg_indexes[1]];
				self.pc + 4
			}
			MUL => {
				self[arg_indexes[2]] = self[arg_indexes[0]] * self[arg_indexes[1]];
				self.pc + 4
			}
			INP => {
				self[arg_indexes[0]] = self.input.pop_front().ok_or(NeedsInput)?;
				self.pc + 2
			}
			OUT => {
				output = Some(self[arg_indexes[0]]);
				self.pc + 2
			}
			JIT => {
				if self[arg_indexes[0]] != 0 {
					self[arg_indexes[1]] as usize
				} else {
					self.pc + 3
				}
			}
			JIF => {
				if self[arg_indexes[0]] == 0 {
					self[arg_indexes[1]] as usize
				} else {
					self.pc + 3
				}
			}
			LES => {
				self[arg_indexes[2]] = if self[arg_indexes[0]] < self[arg_indexes[1]] {
					1
				} else {
					0
				};
				self.pc + 4
			}
			EQU => {
				self[arg_indexes[2]] = if self[arg_indexes[0]] == self[arg_indexes[1]] {
					1
				} else {
					0
				};
				self.pc + 4
			}
			ADJ => {
				self.rel += self[arg_indexes[0]];
				self.pc + 2
			}
			HLT => {
				self.halted = true;
				return Err(Halted);
			}

			_ => return Err(InvalidInstruction(self[self.pc])),
		};
		self.pc = new_pc;
		Ok(output)
	}

	pub fn with_input(mut self, value: Data) -> Self {
		self.input.push_back(value);
		self
	}
	pub fn with_input_from<I>(mut self, values: I) -> Self
	where
		I: IntoIterator<Item = Data>,
	{
		self.input.extend(values);
		self
	}
	pub fn add_input(&mut self, value: Data) {
		self.input.push_back(value);
	}
	pub fn add_input_from<I>(&mut self, values: I)
	where
		I: IntoIterator<Item = Data>,
	{
		self.input.extend(values);
	}

	pub fn halted(&self) -> bool {
		self.halted
	}

	pub fn exploded(&self) -> bool {
		self.exploded
	}

	pub fn explode(&mut self) {
		self.exploded = true;
	}

	pub fn pc(&self) -> Indexer {
		self.pc
	}

	pub fn rel(&self) -> Data {
		self.rel
	}

	pub fn input(&self) -> &VecDeque<Data> {
		&self.input
	}

	pub fn input_mut(&mut self) -> &mut VecDeque<Data> {
		&mut self.input
	}
	
	pub fn memory(&self) -> &M {
		&self.mem
	}
	
	pub fn memory_mut(&mut self) -> &mut M {
		&mut self.mem
	}

	fn separate_instruction(&self) -> Result<(Vec<usize>, usize), IntcodeError> {
		let mut opcode = self[self.pc] as usize;
		let instruction = opcode % 100;
		opcode /= 100;
		let mut modes = Vec::with_capacity(3);
		while opcode > 0 {
			modes.push(opcode % 10);
			opcode /= 10;
		}
		modes.resize(3, 0);
		let arg_indexes = modes
			.into_iter()
			.enumerate()
			.map(|(mut i, mode)| {
				i += 1;
				Ok(match mode {
					POSITION => self[i + self.pc] as usize,
					IMMEDIATE => i + self.pc,
					RELATIVE => (self.rel + self[i + self.pc]) as usize,
					_ => return Err(InvalidMode(self[self.pc])),
				})
			})
			.collect::<Result<Vec<usize>, IntcodeError>>()?;
		Ok((arg_indexes, instruction))
	}
}

impl<M: Memory> Iterator for IntcodeProgram<M> {
	type Item = Result<Data, IntcodeError>;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.step() {
				Ok(Some(o)) => return Some(Ok(o)),
				Err(Halted) => return None,
				Err(e) => return Some(Err(e)),
				_ => (),
			}
		}
	}
}

impl<M: Memory> Index<Indexer> for IntcodeProgram<M> {
	type Output = Data;

	fn index(&self, index: Indexer) -> &Self::Output {
		self.mem.index(index)
	}
}

impl<M: Memory> IndexMut<Indexer> for IntcodeProgram<M> {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		self.mem.index_mut(index)
	}
}

use std::str::FromStr;
impl<M: FromStr + Memory> FromStr for IntcodeProgram<M> {
	type Err = M::Err;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::new(s.parse()?))
	}
}

use std::fs::read_to_string;
impl<M: FromStr + Memory> IntcodeProgram<M> {
	pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, IntcodeError> {
		Ok(read_to_string(&path)?
			.parse()
			.map_err(|_| IntcodeError::OtherError)?)
	}
}
