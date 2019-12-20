const DAY: &str = "inputs/day18.txt";
// use itertools::Itertools;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
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
	let num_keys = all_keys.len() as u8;

	let mut distances: Vec<Option<usize>> = vec![None; num_keys as usize];
	for (this_key, (ky, kx)) in (0..num_keys).map(|n| (n, all_keys[&n])) {
		let mut searches: Vec<(usize, usize)> = vec![(ky, kx)];
		let mut searches_new = Vec::new();
		let mut visited = HashSet::new();
		visited.insert((ky, kx));

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
						key if key >= b'a' || key == ENTRANCE => {
							distances[this_key as usize] = Some(i);
							break 'a;
						}
						_door => (),
					}
					searches_new.push((ny, nx));
				}
			}
			std::mem::swap(&mut searches, &mut searches_new);
			searches_new.clear();
		}
	}
	let distances = distances.iter().map(|d| d.unwrap()).collect::<Vec<_>>();
	println!("{:?}", distances);
	let all_key_cost: usize = distances.iter().sum();
	
	tunnels[entrance.0][entrance.1] = PATH;

	let first = Searcher::start(entrance.0, entrance.1, all_key_cost);
	let mut heap: MinHeap<Searcher> = MinHeap::new();
	heap.push(first);

	let mut visited: Vec<Vec<HashMap<[bool; LETTER_COUNT], usize>>> = tunnels
		.iter()
		.map(|row| row.iter().map(|_| HashMap::new()).collect())
		.collect();
	
	while let Some(mut search) = heap.pop() {
		search.dist_from_last_key += 1;
		search.current_cost += 1;
		for &(dy, dx) in DIRECTIONS.iter() {
			let mut search: Searcher = search.clone();
			let (ny, nx) = (
				(search.y as isize + dy) as usize,
				(search.x as isize + dx) as usize
			);
			match tunnels[ny][nx] {
				WALL => continue,
				PATH => (),
				mut key if key >= b'a' => {
					key -= b'a';
					if !search.collected[key as usize] {
						let mut new_collected = *search.collected.as_ref();
						new_collected[key as usize] = true;
						search.collected = Rc::new(new_collected);
						search.dist_from_last_key = 0;
						search.left_key_cost -= distances[key as usize];
					}
				}
				door => {
					unimplemented!();
				}
			}
			if let Some(&v) = visited[ny][nx].get(search.collected.as_ref()) {
				if v <= search.heuristic() {
					continue;
				}
			};
		}
	}
}

const WALL: u8 = b'#';
const PATH: u8 = b'.';
const ENTRANCE: u8 = b'@';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const LETTER_COUNT: usize = 26;
// const TO_LOWER: u8 = b'a' - b'A';

#[derive(Debug, Clone)]
struct Searcher {
	y: usize,
	x: usize,
	// To be subtracted from heuristic
	dist_from_last_key: usize,
	// Heuristic
	left_key_cost: usize,
	// Real steps so far
	current_cost: usize,
	// Keys collected
	collected: Rc<[bool; LETTER_COUNT]>,
}

impl Searcher {
	fn new(y: usize, x: usize, dist: usize, left: usize, current: usize, collected: [bool; LETTER_COUNT]) -> Self {
		Self {
			y,
			x,
			dist_from_last_key: dist,
			left_key_cost: left,
			current_cost: current,
			collected: Rc::new(collected),
		}
	}

	fn start(y: usize, x: usize, all_key_cost: usize) -> Self {
		Self::new(y, x, 0, all_key_cost, 0, [false; LETTER_COUNT])
	}

	fn heuristic(&self) -> usize {
		self.current_cost + self.left_key_cost.checked_sub(self.dist_from_last_key).unwrap_or(0)
	}
	
	fn adjust_cost(&mut self, key: u8, distances: &[usize]) {
		self.left_key_cost -= distances[key as usize];
	}
}

impl PartialEq for Searcher {
	fn eq(&self, other: &Self) -> bool {
		self.heuristic().eq(&other.heuristic())
	}
}

impl Eq for Searcher {}

impl PartialOrd for Searcher {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.heuristic().partial_cmp(&other.heuristic())
	}
}

impl Ord for Searcher {
	fn cmp(&self, other: &Self) -> Ordering {
		self.heuristic().cmp(&other.heuristic())
	}
}

#[derive(Debug, Clone)]
struct MinHeap<T> {
	heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MinHeap<T> {
	fn new() -> Self {
		Self {
			heap: BinaryHeap::new(),
		}
	}

	fn pop(&mut self) -> Option<T> {
		self.heap.pop().map(|r| r.0)
	}

	fn push(&mut self, item: T) {
		self.heap.push(Reverse(item))
	}

	fn len(&self) -> usize {
		self.heap.len()
	}
}
