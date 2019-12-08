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
	for layer in layers.iter() {
		for (im, &pix) in image.iter_mut().zip(layer.iter()) {
			if *im == 2 {
				*im = pix;
			}
		}
	}
	for row in image.chunks(WIDTH) {
		for pix in row {
			print!(
				"{}",
				match pix {
					0 => "  ",
					1 => "##",
					2 => "__",
					_ => panic!(),
				}
			);
		}
		println!();
	}
}
