use crate::{Data, Indexer};
use defaultmap::DefaultHashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct HybridMemory {
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

use std::{num::ParseIntError, str::FromStr};
impl FromStr for HybridMemory {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split(',')
			.map(|n| n.trim().parse())
			.collect::<Result<Vec<_>, _>>()
			.map(Self::from_program)
	}
}
