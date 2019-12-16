const DAY: u8 = 16;
use std::fs::read_to_string;
use std::iter::once;

fn main() {
	let mut signal: Vec<isize> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		// let mut signal: Vec<isize> = "12345678"
		.trim()
		.chars()
		.map(|c| c.to_digit(10).unwrap() as isize)
		.collect();

	// println!("{:?}", signal);
	let signal2 = signal.clone();

	for _ in 0..100 {
		let mut new_signal = Vec::new();
		for n in 0..signal.len() {
			let sum = signal
				.iter()
				.zip(
					[0, 1, 0, -1]
						.iter()
						.flat_map(|x| once(x).cycle().take(n + 1))
						.cycle()
						.skip(1),
				)
				.map(|(a, b)| a * b)
				.sum::<isize>();
			new_signal.push((sum % 10).abs());
		}
		signal = new_signal;
	}
	print_vec(&signal[0..8]);

	signal = signal2;
	let index: usize = signal
		.iter()
		.take(7)
		.fold(0, |acc, &x| acc * 10 + x as usize);
	let len = signal.len();
	signal = signal
		.into_iter()
		.cycle()
		.take(len * 10_000)
		.skip(index)
		.collect();
	// println!("{} - {} = {}", len * 10_000, index, signal.len());

	for _ in 0..100 {
		let mut new_signal = Vec::new();
		let mut sum = signal.iter().sum::<isize>();
		for n in signal.iter() {
			new_signal.push(sum % 10);
			sum -= n;
		}
		signal = new_signal;
	}
	print_vec(&signal[0..8]);
}

fn print_vec(v: &[impl std::fmt::Display]) {
	let mut s = String::new();
	for i in v.iter().take(8) {
		s.push_str(&i.to_string());
	}
	println!("{}", s);
}
