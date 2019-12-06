use std::collections::HashMap;

fn main() {
	let input: HashMap<String, String> = std::fs::read_to_string("inputs/day06.txt")
		.unwrap()
	// let input: HashMap<String, String> = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
	// let input: HashMap<String, String> = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
		.trim()
		.lines()
		.map(|l| {
			let mut vals = l.split(')').map(|v| v.trim().to_owned());
			let first = vals.next().unwrap();
			let second = vals.next().unwrap();
			(second, first)
		})
		.collect();

	let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
	for (k, v) in input.iter() {
		let e = orbits.entry(v).or_default();
		e.push(k);
	}

	// // println!("{:?}", orbits);

	let center = "COM";
	let mut total = 0;
	let mut current = 1;
	let mut working = Vec::new();
	working.push(center);

	while !working.is_empty() {
		println!("{:?}", working);
		let new_working: Vec<&str> = working
			.into_iter()
			.filter_map(|v| orbits.get(v).map(|vs| vs.iter().copied()))
			.flatten()
			.collect();
		total += new_working.len() * current;
		current += 1;
		working = new_working;
	}

	println!("{}", total);
	
	let mut you_things = HashMap::new();
	let mut here = "YOU";
	let mut counter = 0;
	while here != center {
		you_things.insert(here, counter);
		counter += 1;
		here = &input[here];
	}
	let mut here = "SAN";
	let mut counter = 0;
	while !you_things.contains_key(&here) {
		counter += 1;
		here = &input[here];
	}
	
	println!("{}", counter + you_things[here] - 2);
}
