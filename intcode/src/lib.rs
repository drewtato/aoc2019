mod tests;

use std::{collections::VecDeque, ops::{IndexMut, Index}};
use defaultmap::DefaultHashMap;

type Data = i64; 
type Indexer = usize;

macro_rules! instructions {
	( $( $name:ident = $code:literal; )* ) => {
		mod Instructions {
			$(
				const $name: isize = $code;
			)*
		}
	};
}
macro_rules! modes {
	( $( $name:ident = $code:literal; )* ) => {
		mod Modes {
			$(
				const $name: isize = $code;
			)*
		}
	};
}

instructions! {
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
}

modes! {
	POSITION = 0;
	IMMEDIATE = 1;
	RELATIVE = 2;
}

pub struct IntcodeProgram<M: IndexMut<Indexer, Output=Data>> {
	mem: M,
	pc: Option<Indexer>,
	rel: Data,
	input: VecDeque<Data>,
}

impl<M: IndexMut<Indexer, Output=Data>> IntcodeProgram<M> {
	pub fn step(&mut self) -> Result<Option<Data>, IntcodeError> {
		let (args, instruction) = self.extract_instruction();
		
		unimplemented!()
	}
	pub fn with_input(mut self, value: Data) -> Self {
		self.input.push_back(value);
		self
	}
	pub fn with_input_from<I>(mut self, values: I) -> Self where I: IntoIterator<Item=Data> {
		self.input.extend(values);
		self
	}
	pub fn add_input(&mut self, value: Data) {
		self.input.push_back(value);
	}
	pub fn add_input_from<I>(&mut self, values: I) where I: IntoIterator<Item=Data> {
		self.input.extend(values);
	}
	fn extract_instruction(&self) -> (Vec<usize>, isize) {
		unimplemented!()
	}
}

impl<M: IndexMut<Indexer, Output=Data>> Iterator for IntcodeProgram<M> {
	type Item = Result<Data, IntcodeError>;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.step() {
				Ok(Some(o)) => return Some(Ok(o)),
				Err(e) => return Some(Err(e)),
				_ => (),
			}
		}
	}
}

struct HybridMemory {
	first_chunk: Vec<Data>,
	rest: DefaultHashMap<Indexer, Data>,
}

impl HybridMemory {
	// fn new() -> Self {
	// 	HybridMemory {
	// 		first_chunk: Vec::new(),
	// 		rest: DefaultHashMap::default(),
	// 	}
	// }
	fn from_program(mut prog: Vec<Data>) -> Self {
		prog.resize(prog.len() * 2, 0);
		HybridMemory {
			first_chunk: prog,
			rest: DefaultHashMap::default(),
		}
	}
}

impl Index<Indexer> for HybridMemory {
	type Output = Data;
	
	fn index(&self, index: Indexer) -> &Self::Output {
		if let Some(x) = self.first_chunk.get(index) {
			x
		} else {
			&self.rest[index]
		}
	}
}

impl IndexMut<Indexer> for HybridMemory {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		if let Some(x) = self.first_chunk.get_mut(index) {
			x
		} else {
			&mut self.rest[index]
		}
	}
}

use std::{str::FromStr, num::ParseIntError};
impl FromStr for IntcodeProgram<HybridMemory> {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(IntcodeProgram {
			mem: HybridMemory::from_str(s)?,
			pc: Some(0),
			rel: 0,
			input: VecDeque::new(),
		})
	}
}

impl FromStr for HybridMemory {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s
			.split(',')
			.map(|n| n.trim().parse())
			.collect::<Result<Vec<_>, _>>()
			.map(Self::from_program)
	}
}

#[derive(Debug, Copy, Clone)]
pub enum IntcodeError {
	NeedsInput,
	InvalidIndex(Data),
	Exploded,
}
use IntcodeError::*;

use std::fmt::{self, Display, Formatter};
impl Display for IntcodeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			NeedsInput => "Intcode machine is waiting for input".to_owned(),
			InvalidIndex(d) => format!("Intcode machine received an invalid input: {}", d),
			Exploded => "Intode machine has gone kaboom :(".to_owned(),
		})
	}
}

use std::error::Error;
impl Error for IntcodeError {}