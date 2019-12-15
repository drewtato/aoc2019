const DAY: u8 = 15;
use intcode::{IntcodeProgram, VecMemory};
use std::collections::HashMap;

fn main() {
	let program: IntcodeProgram<VecMemory> =
		IntcodeProgram::from_file(format!("inputs/day{:02}.txt", DAY)).unwrap();

	// println!("{:?}", program);

	let mut map: HashMap<(i64, i64), i64> = HashMap::new();
	map.insert((0, 0), 5);
	let mut all_programs = vec![(program, (0, 0))];
	let directions = [NORTH, SOUTH, EAST, WEST];
	let mut steps = None;
	let mut goal_prog = None;
	'a: for i in 1.. {
		// print_map(&map);
		for (prog, pos) in all_programs.drain(..).collect::<Vec<_>>().into_iter() {
			for &dir in directions.iter() {
				let new_pos = translate(pos, dir);
				if map.contains_key(&new_pos) {
					continue;
				}
				let mut new = prog.clone().with_input(dir);
				match new.next().unwrap().unwrap() {
					0 => {
						map.insert(new_pos, WALL);
					}
					1 => {
						map.insert(new_pos, SPACE);
						all_programs.push((new, new_pos));
					}
					2 => {
						map.insert(new_pos, OXYGEN);
						goal_prog = Some((new, new_pos));
						steps = Some(i);
						break 'a;
					}
					_ => unreachable!(),
				}
			}
		}
	}
	println!("{}", steps.unwrap());

	all_programs = vec![(goal_prog.unwrap())];

	map = map.into_iter().filter(|(_k, v)| *v != SPACE).collect();

	let mut minutes = None;
	for i in 0.. {
		if all_programs.is_empty() {
			minutes = Some(i - 1);
			break;
		}
		for (prog, pos) in all_programs.drain(..).collect::<Vec<_>>().into_iter() {
			for &dir in directions.iter() {
				let new_pos = translate(pos, dir);
				if map.contains_key(&new_pos) {
					continue;
				}
				let mut new = prog.clone().with_input(dir);
				match new.next().unwrap().unwrap() {
					0 => {
						map.insert(new_pos, WALL);
					}
					1 => {
						map.insert(new_pos, SPACE);
						all_programs.push((new, new_pos));
					}
					// 2 => {
					// 	map.insert(new_pos, OXYGEN);
					// 	steps = Some(i);
					// }
					_ => unreachable!(),
				}
			}
		}
	}
	println!("{}", minutes.unwrap());
	// // print_map(&map);
}

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

// fn print_map(map: &HashMap<(i64, i64), i64>) {
// 	let min_x = map.keys().map(|(_, x)| x).min().unwrap();
// 	let min_y = map.keys().map(|(y, _)| y).min().unwrap();
// 	// dbg!(&min_x, &min_y);
// 	let mut map_vec = Vec::new();
// 	for ((y, x), &v) in map.iter() {
// 		let (y, x) = ((y - min_y) as usize, (x - min_x) as usize);
// 		if y >= map_vec.len() {
// 			map_vec.resize_with(y + 1, Vec::new);
// 		}
// 		let row = &mut map_vec[y];
// 		if x >= row.len() {
// 			row.resize(x + 1, 0);
// 		}
// 		row[x] = v;
// 	}
// 	// map_vec[(pos.0 - min_y) as usize][(pos.1 - min_x) as usize] = 4;
// 	let s: String = map_vec
// 		.into_iter()
// 		.map(|row| {
// 			row.into_iter()
// 				.map(val_to_str)
// 				.chain(std::iter::once("\n"))
// 				.collect::<String>()
// 		})
// 		.collect();
// 	eprint!("{}", s);
// }

// const IDK: i64 = 0;
const WALL: i64 = 1;
const OXYGEN: i64 = 2;
const SPACE: i64 = 3;
// const BOT: i64 = 4;
// const START: i64 = 5;

// fn val_to_str(val: i64) -> &'static str {
// 	match val {
// 		IDK => "  ",
// 		WALL => "██",
// 		OXYGEN => "O2",
// 		SPACE => "__",
// 		BOT => ":D",
// 		START => "XX",
// 		_ => unreachable!(),
// 	}
// }

// fn get_direction() -> i64 {
// 	loop {
// 		let mut s = String::new();
// 		std::io::stdin().read_line(&mut s).unwrap();
// 		return match s.trim() {
// 			"w" => NORTH,
// 			"s" => SOUTH,
// 			"a" => WEST,
// 			"d" => EAST,
// 			_ => continue,
// 		};
// 	}
// }

fn translate(pos: (i64, i64), direction: i64) -> (i64, i64) {
	match direction {
		NORTH => (pos.0 - 1, pos.1),
		SOUTH => (pos.0 + 1, pos.1),
		WEST => (pos.0, pos.1 - 1),
		EAST => (pos.0, pos.1 + 1),
		_ => unreachable!(),
	}
}
