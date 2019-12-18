#![feature(drain_filter)]
const DAY: &str = "inputs/day18.txt";
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::{cell::RefCell, rc::Rc};

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
					all_keys.insert(spot - b'a');
				}
				_ => (),
			}
		}
	}
	// println!("{:?} {}", all_keys, all_keys.len());
	let entrance = entrance.unwrap();
	tunnels[entrance.0][entrance.1] = PATH;
	let num_keys = all_keys.len();

	let first_searcher = Searcher::new(
		entrance.0,
		entrance.1,
		Rc::new([0; LETTER_COUNT]),
		Visited::default(),
		0,
		// vec![entrance],
	);
	first_searcher
		.visited
		.borrow_mut()
		.insert((entrance.0, entrance.1));
	let mut all_searchers: Vec<Searcher> = vec![first_searcher];
	let mut subsequences: HashSet<Vec<u8>> = HashSet::new();
	'a: for i in 1.. {
		let mut new_searchers: Vec<Searcher> = Vec::new();
		for searcher in all_searchers.drain(..) {
			for (dy, dx) in DIRECTIONS.iter() {
				let (ny, nx) = (
					(searcher.y as isize + dy) as usize,
					(searcher.x as isize + dx) as usize,
				);
				if searcher.visited.borrow().contains(&(ny, nx)) {
					continue;
				}
				match tunnels[ny][nx] {
					WALL => continue,
					PATH => (),
					x if x >= b'a' => {
						let key = x - b'a';
						if searcher.collected[key as usize] == 0 {
							let mut new_collected = *searcher.collected.as_ref();
							new_collected[key as usize] = searcher.count + 1;
							if subsequences.contains(&letters_from_collected(&new_collected)) {
								continue;
							}
							if searcher.count == (num_keys - 1) as u8 {
								println!("{}", i);
								// println!("{} {:?} {:?}", i, letters_from_collected(&new_collected), searcher.path);
								break 'a;
							}
							for sub in all_subsequences(&new_collected).into_iter() {
								subsequences.insert(sub);
							}
							let mut new_visited = HashSet::new();
							new_visited.insert((ny, nx));
							new_searchers.push(Searcher::new(
								ny,
								nx,
								Rc::new(new_collected),
								Rc::new(RefCell::new(new_visited)),
								searcher.count + 1,
								// searcher.path.iter().cloned().chain(std::iter::once((ny, nx))).collect(),
							));
							continue;
						}
					}
					door => {
						let door = door - b'A';
						if searcher.collected[door as usize] == 0 {
							continue;
						}
					}
				}
				searcher.visited.borrow_mut().insert((ny, nx));
				new_searchers.push(Searcher::new(
					ny,
					nx,
					Rc::clone(&searcher.collected),
					Rc::clone(&searcher.visited),
					searcher.count,
					// searcher.path.iter().cloned().chain(std::iter::once((ny, nx))).collect(),
				));
			}
		}
		println!("{} {}", new_searchers.len(), subsequences.len());
		all_searchers = new_searchers;
	}
}

const WALL: u8 = b'#';
const PATH: u8 = b'.';
const ENTRANCE: u8 = b'@';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const LETTER_COUNT: usize = 26;

type Visited = Rc<RefCell<HashSet<(usize, usize)>>>;
#[derive(Debug, Clone)]
struct Searcher {
	y: usize,
	x: usize,
	collected: Rc<[u8; LETTER_COUNT]>,
	count: u8,
	visited: Visited,
	// path: Vec<(usize, usize)>,
}

impl Searcher {
	fn new(
		y: usize,
		x: usize,
		collected: Rc<[u8; LETTER_COUNT]>,
		visited: Visited,
		count: u8,
		// path: Vec<(usize, usize)>
	) -> Self {
		Self {
			y,
			x,
			collected,
			count,
			visited,
			// path,
		}
	}
}

fn letters_from_collected(collected: &[u8]) -> Vec<u8> {
	let mut letters: Vec<u8> = vec![0; LETTER_COUNT];
	for (i, &order) in collected.iter().enumerate() {
		if order != 0 {
			letters[order as usize - 1] = i as u8 + 1;
		}
	}
	let letters: Vec<_> = letters
		.into_iter()
		.filter(|&n| n != 0)
		.map(|n| n - 1)
		.collect();
	letters
}

fn all_subsequences(collected: &[u8]) -> Vec<Vec<u8>> {
	let mut letters = letters_from_collected(collected);
	let last = letters.pop().unwrap();
	let len = letters.len();
	(1..len).flat_map(|n| letters.iter().cloned().combinations(n).map(|mut s| {
		s.push(last);
		s
	})).collect()
}
