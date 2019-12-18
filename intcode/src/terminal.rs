use crate::{Data, IntcodeError, IntcodeProgram, Memory};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct IntcodeTerminal<M: Memory>(pub IntcodeProgram<M>);

impl<M: Memory> IntcodeTerminal<M> {
	pub fn new(program: IntcodeProgram<M>) -> Self {
		Self(program)
	}

	pub fn interactive(&mut self) -> Result<(), Box<dyn Error>> {
		let program = &mut self.0;
		let mut stdout = io::stdout();
		let mut out_buf: Vec<u8> = b"< ".to_vec();
		loop {
			match program.next() {
				Some(Ok(out)) => match u8::try_from(out) {
					Ok(out) => match out {
						b'\n' => {
							out_buf.push(out);
							stdout.write_all(&out_buf)?;
							stdout.flush()?;
							out_buf.truncate(2);
						}
						_ => out_buf.push(out),
					},
					Err(_) => {
						if out_buf.len() > 2 {
							out_buf.push(b'\n');
						} else {
							out_buf.clear();
						}
						out_buf.extend_from_slice(b"<# ");
						out_buf.extend_from_slice(out.to_string().as_bytes());
						out_buf.push(b'\n');
						stdout.write_all(&out_buf)?;
						stdout.flush()?;
						out_buf.truncate(2);
					}
				},
				x => {
					if out_buf.len() > 2 {
						out_buf.push(b'\n');
						stdout.write_all(&out_buf)?;
						stdout.flush()?;
						out_buf.truncate(2);
					}
					match x {
						Some(Err(IntcodeError::NeedsInput)) => provide_input(&mut stdout, program)?,
						None => {
							break;
						}
						// Already checked for this in last match
						_ => unsafe { std::hint::unreachable_unchecked() },
					}
				}
			}
		}
		stdout.write_all(b"Program ended.\n")?;
		Ok(())
	}
}

fn get_input() -> Result<String, Box<dyn Error>> {
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	Ok(input.trim().to_owned())
}

fn provide_input<M: Memory>(
	stdout: &mut io::Stdout,
	program: &mut IntcodeProgram<M>,
) -> Result<(), Box<dyn Error>> {
	stdout.write_all(b"> ")?;
	stdout.flush()?;
	let mut input = get_input()?;
	match input.as_str() {
		"/n" => {
			stdout.write_all(b"># ")?;
			stdout.flush()?;
			let numbers = get_input()?
				.split_whitespace()
				.map(|n| n.parse())
				.collect::<Result<Vec<_>, _>>()?;
			program.add_input_from(numbers.into_iter());
		}
		"/r" => {
			stdout.write_all(b">r ")?;
			stdout.flush()?;
			let input = get_input()?;
			program.add_input_from(input.as_bytes().iter().map(|&a| a as Data));
		}
		_ => {
			input.push('\n');
			program.add_input_from(input.as_bytes().iter().map(|&a| a as Data));
		}
	}
	Ok(())
}
