const DAY: &str = "inputs/day18.txt";
// use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::rc::Rc;

fn main() {
	let mut tunnels: Vec<Vec<u8>> = read_to_string(DAY)
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.chars().map(|n| n as u8).collect())
		.collect();

	// println!("{:?}", tunnels);

	let mut all_keys = HashSet::new();
	let mut entrance = None;
	for (y, row) in tunnels.iter().enumerate() {
		for (x, &spot) in row.iter().enumerate() {
			match spot {
				WALL | PATH => (),
				ENTRANCE => entrance = Some((y, x)),
				x if x >= b'a' => {
					all_keys.insert(spot);
				}
				_ => (),
			}
		}
	}
	// println!("{:?} {}", all_keys, all_keys.len());
	let entrance = entrance.unwrap();
	tunnels[entrance.0][entrance.1] = PATH;
	let num_keys = all_keys.len();

	type Location = Option<HashSet<Rc<Vec<u8>>>>;
	let mut visited: Vec<Vec<Location>> = tunnels
		.iter()
		.map(|row| {
			row.iter()
				.map(|&v| {
					if v == WALL {
						None
					} else {
						Some(HashSet::new())
					}
				})
				.collect()
		})
		.collect();
	type Searcher = (usize, usize, Rc<Vec<u8>>);
	let mut searchers: Vec<Searcher> = vec![(entrance.0, entrance.1, Rc::default())];
	'a: for i in 1.. {
		let mut searchers_new: Vec<Searcher> = Vec::new();
		for (y, x, letters) in searchers.into_iter() {
			for (dy, dx) in DIRECTIONS.iter() {
				let (ny, nx) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
				if visited[ny][nx]
					.as_ref()
					.map(|hs| hs.contains(&letters))
					.unwrap_or(true)
				{
					continue;
				}
				visited[ny][nx]
					.as_mut()
					.unwrap()
					.insert(Rc::clone(&letters));
				match tunnels[ny][nx] {
					WALL => unreachable!(),
					PATH => (),
					key if key >= b'a' => {
						if !letters.as_ref().iter().any(|&l| l == key) {
							if letters.as_ref().len() == num_keys - 1 {
								println!("{}", i);
								break 'a;
							}
							let mut letters_new = letters.as_ref().clone();
							letters_new.push(key);
							searchers_new.push((ny, nx, Rc::new(letters_new)));
							continue;
						}
					}
					door => {
						if !letters.as_ref().iter().any(|&l| l == door + TO_LOWER) {
							continue;
						}
					}
				}
				searchers_new.push((ny, nx, Rc::clone(&letters)));
			}
		}
		println!("{} {}", searchers_new.len(), visited.iter().flat_map(|row| row.iter().map(|l| l.as_ref().map(|v| v.len()).unwrap_or(0))).max().unwrap());
		searchers = searchers_new;
	}
}

const WALL: u8 = b'#';
const PATH: u8 = b'.';
const ENTRANCE: u8 = b'@';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
// const LETTER_COUNT: usize = 26;
const TO_LOWER: u8 = b'a' - b'A';
