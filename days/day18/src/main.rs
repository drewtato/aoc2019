const DAY: &str = "inputs/day18.txt";
use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::{rc::Rc, cell::RefCell};

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
	
	let searcher = Searcher::new(entrance);
	
	unimplemented!();
}

const WALL: u8 = b'#';
const PATH: u8 = b'.';
const ENTRANCE: u8 = b'@';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

struct Searcher {
	y: usize,
	x: usize,
	collected: Rc<[u8; 26]>,
	visited: Rc<RefCell<HashSet<(usize, usize)>>>,
}

impl Searcher {
	fn new(pos: (usize, usize)) -> Self {
		Self {
			y: pos.0,
			x: pos.1,
			collected: Rc::new([0; 26]),
			visited: Rc::new(RefCell::new(HashSet::new())),
		}
	}
}