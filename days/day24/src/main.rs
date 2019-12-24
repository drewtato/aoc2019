const DAY: &str = "inputs/day24.txt";
use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
	let mut bugs: Vec<Vec<Bug>> = read_to_string(DAY)
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.trim()
				.chars()
				.map(|n| match n {
					'.' => Dead,
					'#' => Alive,
					_ => unreachable!(),
				})
				.collect()
		})
		.collect();

	let (max_y, max_x) = (bugs.len(), bugs[0].len());
	bugs.insert(0, vec![Dead; max_x]);
	bugs.push(vec![Dead; max_x]);
	
	for row in bugs.iter_mut() {
		row.insert(0, Dead);
		row.push(Dead);
	}
	
	let mut seen_states = HashSet::new();
	
	loop {
		// print_map(&bugs);
		if !seen_states.insert(bugs.clone()) {
			break;
		}
		bugs = evolve(&bugs);
	}
	
	bugs.pop();
	bugs.remove(0);
	
	for row in bugs.iter_mut() {
		row.pop();
		row.remove(0);
	}
	
	println!("{}", biodiversity_rating(&bugs));
	
}

fn biodiversity_rating(bugs: &[Vec<Bug>]) -> usize {
	let mut current = 1;
	let mut total = 0;
	for &bug in bugs.iter().flat_map(|row| row.iter()) {
		if bug == Alive {
			total += current;
		}
		current *= 2;
	}
	total
}

fn evolve(bugs: &[Vec<Bug>]) -> Vec<Vec<Bug>> {
	let mut new_bugs = bugs.to_vec();
	for y in 1..(bugs.len() - 1) {
		for x in 1..(bugs[y].len() - 1) {
			let mut alive_neigbors = 0;
			for (dy, dx) in NEIGHBORS.iter() {
				let (ny, nx) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
				if bugs[ny][nx] == Alive {
					alive_neigbors += 1;
				}
			}
			new_bugs[y][x] = if (alive_neigbors == 1) || ((bugs[y][x] == Dead) && alive_neigbors == 2) {
				Alive
			} else {
				Dead
			};
		}
	}
	new_bugs
}

const NEIGHBORS: [(isize, isize); 4] = [(0, 1),(0, -1),(1, 0),(-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Bug {
	Alive,
	Dead,
}
use Bug::*;

use std::fmt;
impl fmt::Display for Bug {
	// lol
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Alive => '#',
			Dead => '.',
		})
	}
}

fn print_map(bugs: &[Vec<Bug>]) {
	let s: String = bugs.iter().map(|row| {
		let mut row: String = row.iter().map(|bug| bug.to_string()).collect();
		row.push('\n');
		row
	}).collect();
	print!("{}", s);
}