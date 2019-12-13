#![allow(dead_code, unused_mut, unused_variables)]

mod tests;
mod error;
use error::IntcodeError::{self, *};
mod memory;
pub use memory::HybridMemory;

use std::{
	collections::VecDeque,
	ops::{Index, IndexMut},
};

type Data = i64;
type Indexer = usize;

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

pub struct IntcodeProgram<M: IndexMut<Indexer, Output = Data>> {
	mem: M,
	pc: Indexer,
	halted: bool,
	rel: Data,
	input: VecDeque<Data>,
}

impl<M: IndexMut<Indexer, Output = Data>> IntcodeProgram<M> {
	pub fn step(&mut self) -> Result<Option<Data>, IntcodeError> {
		if self.halted {
			return Err(Halted);
		}
		let (arg_indexes, instruction) = self.extract_instruction()?;
		
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
				self[arg_indexes[2]] = if self[arg_indexes[0]] < self[arg_indexes[1]] { 1 } else { 0 };
				self.pc + 4
			}
			EQU => {
				self[arg_indexes[2]] = if self[arg_indexes[0]] == self[arg_indexes[1]] { 1 } else { 0 };
				self.pc + 4
			}
			ADJ => {
				self.rel += self[arg_indexes[0]];
				self.pc + 2
			}
			HLT => {
				self.halted = true;
				return Err(Halted)
			}
			
			_ => return Err(InvalidInstruction(self[self.pc]))
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
	
	fn extract_instruction(&self) -> Result<(Vec<usize>, usize), IntcodeError> {
		let mut opcode = self[self.pc] as usize;
		let instruction = opcode % 100;
		opcode /= 100;
		let mut modes = Vec::with_capacity(4);
		while opcode > 0 {
			modes.push(opcode % 10);
			opcode /= 10;
		}
		modes.resize(4, 0);
		let arg_indexes = modes
			.into_iter()
			.enumerate()
			.map(|(mut i, mode)| {
				i += 1;
				Ok(match mode {
					IMMEDIATE => i + self.pc,
					POSITION => self[i + self.pc] as usize,
					RELATIVE => self.rel as usize + self[i + self.pc] as usize,
					_ => return Err(InvalidMode(self[self.pc])),
				})
			})
			.collect::<Result<Vec<usize>, IntcodeError>>()?;
		Ok((arg_indexes, instruction))
	}
}

impl<M: IndexMut<Indexer, Output = Data>> Iterator for IntcodeProgram<M> {
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

impl<M: IndexMut<Indexer, Output = Data>> Index<Indexer> for IntcodeProgram<M> {
	type Output = Data;
	
	fn index(&self, index: Indexer) -> &Self::Output {
		self.mem.index(index)
	}
}

impl<M: IndexMut<Indexer, Output = Data>> IndexMut<Indexer> for IntcodeProgram<M> {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		self.mem.index_mut(index)
	}
}

use std::str::FromStr;
impl<M: FromStr + IndexMut<Indexer, Output = Data>> FromStr for IntcodeProgram<M> {
	type Err = M::Err;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(IntcodeProgram {
			mem: M::from_str(s)?,
			pc: 0,
			halted: false,
			rel: 0,
			input: VecDeque::new(),
		})
	}
}
