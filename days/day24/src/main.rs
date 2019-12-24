const DAY: &str = "inputs/day24.txt";
use std::fs::read_to_string;

fn main() {
	let input: Vec<Vec<isize>> = read_to_string(DAY)
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.split(',').map(|n| n.trim().parse().unwrap()).collect())
		.collect();

	println!("{:?}", input);
}
