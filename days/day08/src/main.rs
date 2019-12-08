const DAY = 8;
use std::fs::read_to_string;

fn _main() {
	let input: Vec<Vec<isize>> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.split(',').map(|n| n.trim().parse().unwrap()).collect())
		.collect();
	
	println!("{:?}", input);
}
