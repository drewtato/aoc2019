#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

const DAY: &str = "inputs/day20.txt";
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::fs::read_to_string;

fn main() {
	let mut map: Vec<Vec<u8>> = read_to_string(DAY)
		.unwrap()
		.lines()
		.map(|l| l.chars().map(|n| n as u8).collect())
		.collect();

	while map[map.len() - 1].is_empty() {
		map.pop();
	}
	let size = (map.len(), map[0].len());
	// println!("{:?}", input);

	let mut portal_to_coord: HashMap<[u8; 2], (usize, usize)> = HashMap::new();
	let mut coord_to_portal: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

	for row in 0..map.len() {
		for col in 0..map[row].len() {
			let c = map[row][col];

			if c.is_ascii_uppercase() {
				let entrance;
				let c2;

				let possible = map[row + 1][col];
				if possible.is_ascii_uppercase() {
					map[row + 1][col] = SPACE;
					c2 = Some(possible);
					entrance = if map
						.get(row + 2)
						.map(|row| row[col] == PATH)
						.unwrap_or(false)
					{
						Some((row + 2, col))
					} else {
						Some((row - 1, col))
					};
				} else {
					let possible = map[row][col + 1];
					map[row][col + 1] = SPACE;
					c2 = Some(possible);
					entrance = if map[row].get(col + 2).map(|&c| c == PATH).unwrap_or(false) {
						Some((row, col + 2))
					} else {
						Some((row, col - 1))
					};
				}

				let entrance = entrance.unwrap();
				let c2 = c2.unwrap();
				let label = [c, c2];
				if let Some(&other) = portal_to_coord.get(&label) {
					coord_to_portal.insert(entrance, other);
					coord_to_portal.insert(other, entrance);
				} else {
					portal_to_coord.insert(label, entrance);
				}
				map[row][col] = SPACE;
			}
		}
	}

	// println!("{:?}", coord_to_portal);

	let begin = portal_to_coord[&[b'A'; 2]];
	let goal = portal_to_coord[&[b'Z'; 2]];
	
	let mut queue: BinaryHeap<(isize, isize, (usize, usize))> = BinaryHeap::new();
	queue.push((0, 0, begin));
	
	let mut visited: HashMap<((usize, usize), isize), isize> = HashMap::new();
	visited.insert((begin, 0), 0);

	let cost = 'a: loop {
		let (cost, level, (y, x)) = queue
			.pop()
			.unwrap_or_else(|| panic!("No path to exit"));
		// println!("{:?} {}", (y, x), cost);
		
		for &(dy, dx) in NEIGHBORS.iter() {
			let mut new = ((y as isize + dy) as usize, (x as isize + dx) as usize);
			let mut level = level;
			let mut cost = cost;
			match map[new.0][new.1] {
				WALL => continue,
				PATH => (),
				SPACE => {
					let outside = is_outside(new, size);
					if level == 0 && outside {
						continue;
					}
					if let Some(&(py, px)) = coord_to_portal.get(&(y, x)) {
						new = (py, px);
						if outside {
							level += 1;
							cost += LEVEL_PENALTY;
						} else {
							level -= 1;
							cost -= LEVEL_PENALTY;
						}
					} else {
						continue;
					}
				}
				_ => unreachable!(),
			}
			
			if let Some(&c) = visited.get(&(new, level)) {
				if c >= cost {
					continue;
				}
			}
			visited.insert((new, level), cost);
			
			if new == goal && level == 0 {
				break 'a -cost + 1;
			}
			
			queue.push((cost - 1, level, new));
		}
	};
	println!("{}", cost);
}

const PATH: u8 = b'.';
const WALL: u8 = b'#';
const SPACE: u8 = b' ';
const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];
const LEVEL_PENALTY: isize = 34;

fn is_outside(coords: (usize, usize), size: (usize, usize)) -> bool {
	coords.0 == 2 || coords.1 == 2 || coords.0 == size.0 - 3 || coords.1 == size.1 - 3
}