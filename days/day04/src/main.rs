#![feature(is_sorted)]

fn main() {
	let input: Vec<usize> = std::fs::read_to_string("inputs/day04.txt")
		.unwrap()
		.trim()
		.split('-')
		.map(|l| l.parse().unwrap())
		.collect();

	let ans1: Vec<_> = (input[0]..=input[1])
		.map(to_digits)
		.filter(|n| check1(n))
		.collect();
	println!("{}", ans1.len());

	let ans2: Vec<_> = ans1.iter().filter(|n| check2(n)).collect();
	println!("{}", ans2.len());

	// println!(
	// 	"{:?}",
	// 	ans2.iter()
	// 		.map(|v| {
	// 			let mut num = 0;
	// 			for &n in v.iter() {
	// 				num *= 10;
	// 				num += n as usize;
	// 			}
	// 			num
	// 		})
	// 		.collect::<Vec<_>>()
	// );
}

fn to_digits(mut num: usize) -> Vec<u8> {
	let mut v = vec![0; 6];
	let mut pos = v.len();
	while num > 0 {
		pos -= 1;
		v[pos] = (num % 10) as u8;
		num /= 10;
	}
	v
}

fn check1(digits: &[u8]) -> bool {
	// Check for never decreasing
	if !digits.is_sorted() {
		return false;
	}

	// Check for repeats
	for (a, b) in digits.windows(2).map(|win| (win[0], win[1])) {
		if a == b {
			return true;
		}
	}
	false
}

use std::iter::once;

fn check2(digits: &[u8]) -> bool {
	// Check for lone double
	for (a, &b, &c, d) in once(None)
		.chain(digits.iter().map(Some))
		.chain(once(None))
		.collect::<Vec<_>>()
		.windows(4)
		.map(|win| (win[0], win[1].unwrap(), win[2].unwrap(), win[3]))
	{
		if let Some(&a) = a {
			if b == a {
				continue;
			}
		}
		if let Some(&d) = d {
			if b == d {
				continue;
			}
		}
		if b == c {
			return true;
		}
	}
	false
}
