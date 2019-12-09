const DAY: u8 = 8;
use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
	let input: Vec<u8> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		// let input: Vec<u8> = "0222112222120000"
		.trim()
		.chars()
		.map(|n| n.to_digit(10).unwrap() as u8)
		.collect();

	// println!("{:?}", input);

	let layers = input.chunks(WIDTH * HEIGHT).collect::<Vec<_>>();

	#[allow(clippy::naive_bytecount)]
	let fewest_zero = layers
		.iter()
		.enumerate()
		.map(|(i, l)| (l.iter().filter(|&&n| n == 0).count(), i))
		.min()
		.unwrap()
		.1;
	let (one, two) = layers[fewest_zero]
		.iter()
		.fold((0, 0), |(acc1, acc2), n| match n {
			1 => (acc1 + 1, acc2),
			2 => (acc1, acc2 + 1),
			_ => (acc1, acc2),
		});
	println!("{}", one * two);

	let mut image = [2; WIDTH * HEIGHT];
	for (i, im) in image.iter_mut().enumerate() {
		for layer in layers.iter() {
			if layer[i] != 2 {
				*im = layer[i];
				break;
			}
		}
	}

	eprintln!();
	for row in image.chunks(WIDTH) {
		eprint!("  ");
		for pix in row {
			eprint!(
				"{}",
				match pix {
					0 => "  ",
					1 => "██",
					2 => "__",
					_ => unreachable!(),
				}
			);
		}
		eprintln!();
	}
	eprintln!();

	// // Getting the input programmatically yey
	// use std::io::{Write, stderr, stdin};
	// eprint!("Input what you see: ");
	// stderr().flush().unwrap();
	// let mut answer = String::new();
	// stdin().read_line(&mut answer).unwrap();
	// println!("{}", answer.trim().to_ascii_uppercase());
	println!("HZCZU");
}
