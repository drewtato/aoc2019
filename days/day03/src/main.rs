use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
	let input: Vec<Vec<_>> = read_to_string("inputs/day03.txt")
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.split(',')
				.map(|ins| {
					let dir = ins.chars().next().unwrap();
					let number: i64 = ins.chars().skip(1).collect::<String>().parse().unwrap();
					(dir, number)
				})
				.collect()
		})
		.collect();
	// let input: Vec<Vec<_>> = "R8,U5,L5,D3\nU7,R6,D4,L4\n"
	// 	.trim()
	// 	.lines()
	// 	.map(|l| {
	// 		l.split(',').map(|ins| {
	// 			let dir = ins.chars().next().unwrap();
	// 			let number: i64 = ins.chars().skip(1).collect::<String>().parse().unwrap();
	// 			(dir, number)
	// 		}).collect()
	// 	})
	// 	.collect();
	// println!("{:?}", input);
	let mut visited = HashMap::new();
	let (mut x, mut y): (i64, i64) = (0, 0);
	let mut dist = 0;
	for (dir, num) in input[0].iter() {
		let (off_y, off_x) = match dir {
			'R' => (0, 1),
			'L' => (0, -1),
			'U' => (-1, 0),
			'D' => (1, 0),
			_ => panic!("Invalid direction"),
		};
		for _ in 0..*num {
			dist += 1;
			x += off_x;
			y += off_y;
			visited.entry((x, y)).or_insert(dist);
		}
	}
	let mut second = HashMap::new();
	let (mut x, mut y): (i64, i64) = (0, 0);
	let mut dist = 0;
	for (dir, num) in input[1].iter() {
		let (off_y, off_x) = match dir {
			'R' => (0, 1),
			'L' => (0, -1),
			'U' => (-1, 0),
			'D' => (1, 0),
			_ => panic!("Invalid direction"),
		};
		for _ in 0..*num {
			dist += 1;
			x += off_x;
			y += off_y;
			second.entry((x, y)).or_insert(dist);
		}
	}

	let ans = visited
		.iter()
		.filter_map(|(k, _)| second.get(k).map(|_| k))
		.min_by_key(|(y, x)| y.abs() + x.abs())
		.unwrap();

	println!("{}", ans.0.abs() + ans.1.abs());

	let ans2 = visited
		.iter()
		.filter_map(|(k, v)| second.get(k).map(|v2| (k, v + v2)))
		.min_by_key(|(_, v)| *v)
		.unwrap();

	println!("{}", ans2.1);
}
