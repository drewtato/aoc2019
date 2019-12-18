mod error;
pub use error::IntcodeError;

mod memory;
pub use memory::{
	file_to_collection, str_to_collection, HashMemory, HybridMemory, Memory, VecMemory,
};

mod terminal;
pub use terminal::IntcodeTerminal;

mod program;
pub use program::IntcodeProgram;

pub type Data = i64;
pub type Indexer = usize;
