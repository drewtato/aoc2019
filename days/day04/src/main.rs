// #![feature(is_sorted)]

use itertools::Itertools;

fn main() {
	let input: Vec<usize> = std::fs::read_to_string("inputs/day04.txt")
		.unwrap()
		.trim()
		.split('-')
		.map(|l| l.parse().unwrap())
		.collect();

	// let ans1: Vec<_> = (input[0]..=input[1])
	// 	.map(to_digits)
	// 	.filter(|v| check1(v))
	// 	.collect();

	let viable = input[0]..input[1];

	let ans1: Vec<_> = (0..=9)
	// Documentation:
	// https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html#method.combinations_with_replacement
		.combinations_with_replacement(6)
		.filter(|v| viable.contains(&from_digits(v)))
		.filter(|v| check1(v))
		.collect();

	println!("{}", ans1.len());

	let ans2: Vec<_> = ans1.into_iter().filter(|v| check2(v)).collect();
	println!("{}", ans2.len());

	// println!(
	// 	"{:?}",
	// 	ans2.iter()
	// 		.map(|v| from_digits(v))
	// 		.collect::<Vec<_>>()
	// );
}

fn from_digits(v: &[u8]) -> usize {
	let mut num = 0;
	for &n in v.iter() {
		num *= 10;
		num += n as usize;
	}
	num
}

// fn to_digits(mut num: usize) -> Vec<u8> {
// 	let mut v = vec![0; 6];
// 	let mut pos = v.len();
// 	while num > 0 {
// 		pos -= 1;
// 		v[pos] = (num % 10) as u8;
// 		num /= 10;
// 	}
// 	v
// }

fn check1(digits: &[u8]) -> bool {
	// Check for never decreasing
	// if !digits.is_sorted() {
	// 	return false;
	// }

	// Check for repeats
	for win in digits.windows(2) {
		if win[0] == win[1] {
			return true;
		}
	}
	false
}

fn check2(digits: &[u8]) -> bool {
	// Check for lone double
	let mut current_count = 0;
	let mut current_num = 0;
	for &d in digits {
		if d == current_num {
			current_count += 1;
		} else {
			if current_count == 2 {
				return true;
			}
			current_num = d;
			current_count = 1;
		}
	}
	current_count == 2
}
