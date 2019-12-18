use crate::{Data, Indexer, IntcodeError};
use std::{
	iter::FromIterator,
	num::ParseIntError,
	ops::{Index, IndexMut},
	str::FromStr,
};

pub trait Memory: IndexMut<Indexer, Output = Data> {}

#[derive(Debug, Clone)]
pub struct HybridMemory {
	first_chunk: Vec<Data>,
	rest: HashMap<Indexer, Data>,
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
			rest: HashMap::new(),
		}
	}
}

impl Index<Indexer> for HybridMemory {
	type Output = Data;

	fn index(&self, index: Indexer) -> &Self::Output {
		self.first_chunk
			.get(index)
			.unwrap_or_else(|| self.rest.get(&index).unwrap_or(&0))
	}
}

impl IndexMut<Indexer> for HybridMemory {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		if let Some(x) = self.first_chunk.get_mut(index) {
			x
		} else {
			self.rest.entry(index).or_default()
		}
	}
}

impl Memory for HybridMemory {}

impl FromStr for HybridMemory {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split(',')
			.map(|n| n.trim().parse())
			.collect::<Result<Vec<_>, _>>()
			.map(Self::from_program)
	}
}

impl FromIterator<Data> for HybridMemory {
	fn from_iter<I: IntoIterator<Item = Data>>(iter: I) -> Self {
		let mut v: Vec<_> = iter.into_iter().collect();
		v.resize(v.len() * 2, 0);
		Self {
			first_chunk: v,
			rest: HashMap::new(),
		}
	}
}

use std::fs::read_to_string;
impl crate::IntcodeProgram<HybridMemory> {
	pub fn hybrid(s: &str) -> Result<Self, IntcodeError> {
		Ok(s.parse()?)
	}
	pub fn hybrid_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, IntcodeError> {
		Ok(read_to_string(path)?.parse()?)
	}
}


#[derive(Debug, Clone)]
pub struct VecMemory(Vec<Data>);

impl crate::IntcodeProgram<VecMemory> {
	pub fn from_vec(v: Vec<Data>) -> Self {
		Self::new(VecMemory(v))
	}
	pub fn vec_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, IntcodeError> {
		Ok(read_to_string(path)?.parse()?)
	}
}

impl Index<Indexer> for VecMemory {
	type Output = Data;

	fn index(&self, index: Indexer) -> &Self::Output {
		self.0.get(index).unwrap_or(&0)
	}
}

impl IndexMut<Indexer> for VecMemory {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		if self.0.get(index).is_none() {
			self.0.resize(index + 1, 0);
		}
		self.0.get_mut(index).unwrap()
	}
}

impl Memory for VecMemory {}

impl FromStr for VecMemory {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(VecMemory(str_to_collection(s)?))
	}
}

use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct HashMemory(HashMap<Indexer, Data>);

impl Index<Indexer> for HashMemory {
	type Output = Data;

	fn index(&self, index: Indexer) -> &Self::Output {
		self.0.get(&index).unwrap_or(&0)
	}
}

impl IndexMut<Indexer> for HashMemory {
	fn index_mut(&mut self, index: Indexer) -> &mut Self::Output {
		self.0.entry(index).or_default()
	}
}

impl Memory for HashMemory {}

impl FromStr for HashMemory {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(HashMemory(
			s.split(',')
				.enumerate()
				.map(|(i, n)| n.trim().parse().map(|n| (i, n)))
				.collect::<Result<_, _>>()?,
		))
	}
}

pub fn file_to_collection<P, C>(path: P) -> Result<C, IntcodeError>
where
	P: AsRef<std::path::Path>,
	C: FromIterator<Data>,
{
	Ok(str_to_collection(&std::fs::read_to_string(path)?)?)
}

pub fn str_to_collection<C: FromIterator<Data>>(s: &str) -> Result<C, ParseIntError> {
	s.split(',')
		.map(|n| n.trim().parse())
		.collect::<Result<C, _>>()
}
