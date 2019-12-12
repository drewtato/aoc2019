use std::{collections::VecDeque, ops::{IndexMut, Index}};
use defaultmap::DefaultHashMap;

type Data = i64; 
type Indexer = usize;

struct IntcodeProgram<M: IndexMut<Indexer, Output=Data>> {
	mem: M,
	pc: Option<Indexer>,
	rel: Data,
}

impl<M: IndexMut<Indexer, Output=Data>> Iterator for IntcodeProgram<M> {
	type Item = Result<Data, IntcodeError>;
	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!()
	}
}

struct HybridMemory {
	first_chunk: Vec<Data>,
	rest: DefaultHashMap<Indexer, Data>,
}

impl HybridMemory {
	fn new() -> Self {
		HybridMemory {
			first_chunk: Vec::new(),
			rest: DefaultHashMap::default(),
		}
	}
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

enum IntcodeError {
	NeedsInput,
}