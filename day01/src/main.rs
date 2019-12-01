use std::fs::read_to_string;

fn main() {
	let input: Vec<i64> = read_to_string("day01/input.txt")
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.parse().unwrap())
		.collect();

	let ans1 = input.iter().map(|&mass| (mass / 3) - 2).sum::<i64>();
	println!("{}", ans1);

	let ans2 = input
		.iter()
		.map(|&mass| {
			let mut fuel = (mass / 3) - 2;
			let mut total_fuel = fuel;
			while fuel >= 9 {
				fuel = (fuel / 3) - 2;
				total_fuel += fuel;
			}
			total_fuel
		})
		.sum::<i64>();
	println!("{}", ans2);
}
