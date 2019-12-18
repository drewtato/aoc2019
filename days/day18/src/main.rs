const DAY: &str = "inputs/day18.txt";
// use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
// use std::rc::Rc;

fn main() {
	let mut tunnels: Vec<Vec<u8>> = read_to_string(DAY)
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.chars().map(|n| n as u8).collect())
		.collect();

	// println!("{:?}", tunnels);

	let mut all_keys: HashMap<u8, (usize, usize)> = HashMap::new();
	let mut entrance = None;
	for (y, row) in tunnels.iter().enumerate() {
		for (x, &spot) in row.iter().enumerate() {
			match spot {
				WALL | PATH => (),
				ENTRANCE => entrance = Some((y, x)),
				key if key >= b'a' => {
					all_keys.insert(key - b'a', (y, x));
				}
				_ => (),
			}
		}
	}
	// println!("{:?} {}", all_keys, all_keys.len());
	let entrance = entrance.unwrap();
	tunnels[entrance.0][entrance.1] = PATH;
	let num_keys = all_keys.len() as u8;

	let mut distances: Vec<Option<usize>> = vec![None; num_keys as usize];
	for (this_key, (ky, kx)) in (0..num_keys).map(|n| (n, all_keys[&n])) {
		let mut searches: Vec<(usize, usize)> = vec![(ky, kx)];
		let mut searches_new = Vec::new();
		let mut visited = HashSet::new();
		visited.insert((ky, kx));
		
		if distances[this_key as usize].is_some() {
			continue;
		}
		
		'a: for i in 1.. {
			for (y, x) in searches.drain(..) {
				for (dy, dx) in DIRECTIONS.iter() {
					let (ny, nx) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
					if !visited.insert((ny, nx)) {
						continue;
					}
					match tunnels[ny][nx] {
						WALL => continue,
						PATH => (),
						key if key >= b'a' => {
							let key = key - b'a';
							distances[this_key as usize] = Some(i);
							distances[key as usize] = Some(if let Some(old) = distances[key as usize] {
								std::cmp::min(i, old)
							} else {
								i
							});
							break 'a;
						},
						_door => (),
					}
					searches_new.push((ny, nx));
				}
			}
			std::mem::swap(&mut searches, &mut searches_new);
			searches_new.clear();
		}
	}
	println!("{:?}", distances.iter().map(|d| d.unwrap()).collect::<Vec<_>>());
	// let key_to_key_heuristic: usize = distances.into_iter().map(|d| d.unwrap()).sum();
}

const WALL: u8 = b'#';
const PATH: u8 = b'.';
const ENTRANCE: u8 = b'@';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
// const LETTER_COUNT: usize = 26;
// const TO_LOWER: u8 = b'a' - b'A';
