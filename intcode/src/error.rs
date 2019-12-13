use crate::Data;

#[derive(Debug, Copy, Clone)]
pub enum IntcodeError {
	NeedsInput,
	InvalidIndex(Data),
	InvalidMode(Data),
	InvalidInstruction(Data),
	Halted,
	Exploded,
}
use IntcodeError::*;

use std::fmt::{self, Display, Formatter};
impl Display for IntcodeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				NeedsInput => "Intcode machine is waiting for input".to_owned(),
				InvalidIndex(d) => format!("Intcode machine received an invalid input: {}", d),
				InvalidMode(d) => format!("Intcode machine received an invalid mode: {}", d),
				InvalidInstruction(d) => format!("Intcode machine received an invalid instruction: {}", d),
				Halted => "Intcode machine has halted".to_owned(),
				Exploded => "Intode machine has gone kaboom :(".to_owned(),
			}
		)
	}
}

use std::error::Error;
impl Error for IntcodeError {}
