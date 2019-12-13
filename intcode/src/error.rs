use crate::Data;
use std::{convert::From, error::Error, io, num::ParseIntError};

#[derive(Debug)]
pub enum IntcodeError {
	FileReadError(io::Error),
	FileParseError(ParseIntError),
	NeedsInput,
	InvalidIndex(Data),
	InvalidMode(Data),
	InvalidInstruction(Data),
	Halted,
	Exploded,
	OtherError,
}
use IntcodeError::*;

use std::fmt::{self, Display, Formatter};
impl Display for IntcodeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				FileReadError(s) => format!("Could not read file: {}", s),
				FileParseError(s) => format!("Could not parse file: {}", s),
				NeedsInput => "Intcode machine is waiting for input".to_owned(),
				InvalidIndex(d) => format!("Intcode machine received an invalid input: {}", d),
				InvalidMode(d) => format!("Intcode machine received an invalid mode: {}", d),
				InvalidInstruction(d) =>
					format!("Intcode machine received an invalid instruction: {}", d),
				Halted => "Intcode machine has halted".to_owned(),
				Exploded => "Intode machine has gone kaboom :(".to_owned(),
				OtherError => "Intcode machine encountered an error".to_owned(),
			}
		)
	}
}

impl Error for IntcodeError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			FileReadError(s) => Some(s),
			FileParseError(s) => Some(s),
			_ => None,
		}
	}
}

impl From<io::Error> for IntcodeError {
	fn from(error: io::Error) -> Self {
		Self::FileReadError(error)
	}
}

impl From<ParseIntError> for IntcodeError {
	fn from(error: ParseIntError) -> Self {
		Self::FileParseError(error)
	}
}
