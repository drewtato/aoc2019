use std::fs::read_to_string;

fn main() {
	let input = read_to_string("inputs/day02.txt")
		.unwrap()
		.trim()
		.to_owned();
	println!("{}", input);
}
