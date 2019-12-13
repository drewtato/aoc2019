const DAY: u8 = 13;
use intcode::{IntcodeError, IntcodeProgram};

// const EMPTY: i64 = 0;
// const WALL: i64 = 1;
const BLOCK: i64 = 2;
const PADDLE: i64 = 3;
const BALL: i64 = 4;

fn main() {
	let program = IntcodeProgram::from_file(format!("inputs/day{:02}.txt", DAY)).unwrap();
	let program2 = program.clone();

	let all: Vec<_> = program.collect::<Result<_, IntcodeError>>().unwrap();
	let blocks = all
		.chunks(3)
		.map(|chunk| chunk[2])
		.filter(|&tile| tile == BLOCK)
		.count();
	println!("{}", blocks);

	let mut program = program2;
	program[0] = 2;

	// let mut screen: Vec<Vec<i64>> = Vec::new();
	// println!("Use 1, 2, and 3 to play");

	let mut score = 0;
	let mut ball = (0, 0);
	let mut paddle = (0, 0);
	loop {
		let mut tiles = Vec::new();
		while let Some(Ok(v)) = program.next() {
			tiles.push(v);
		}

		// draw_to_screen(&tiles, &mut screen, &mut score, &mut ball, &mut paddle);
		// print_screen(score, &screen);

		draw_minimal(&tiles, &mut score, &mut ball, &mut paddle);

		match program.next() {
			Some(Err(IntcodeError::NeedsInput)) => {
				use std::cmp::Ordering;
				program.add_input(match ball.0.cmp(&paddle.0) {
					Ordering::Less => -1,
					Ordering::Equal => 0,
					Ordering::Greater => 1,
				});

				// program.add_input(get_input().parse::<i64>().unwrap() - 2)
			}
			None => {
				// println!("Computer exited");
				break;
			}
			Some(Err(e)) => {
				println!("Computer has a problem: {}", e);
				break;
			}
			_ => unreachable!(),
		}
	}
	println!("{}", score);
}

fn draw_minimal(
	tiles: &[i64],
	score: &mut i64,
	ball: &mut (usize, usize),
	paddle: &mut (usize, usize),
) {
	for chunk in tiles.chunks(3) {
		let (x, y, tile) = (chunk[0], chunk[1], chunk[2]);
		if x == -1 {
			*score = tile;
		} else if tile == BALL {
			*ball = (x as usize, y as usize);
		} else if tile == PADDLE {
			*paddle = (x as usize, y as usize);
		}
	}
}

// use std::io::{stdin, Read};
// fn get_input() -> String {
// 	let mut buf = [0];
// 	stdin().lock().read_exact(&mut buf).unwrap();
// 	String::from_utf8(buf.to_vec()).unwrap()
// }

// fn draw_to_screen(
// 	tiles: &[i64],
// 	screen: &mut Vec<Vec<i64>>,
// 	score: &mut i64,
// 	ball: &mut (usize, usize),
// 	paddle: &mut (usize, usize),
// ) {
// 	for chunk in tiles.chunks(3) {
// 		let (x, y, tile) = (chunk[0], chunk[1], chunk[2]);
// 		if x == -1 {
// 			*score = tile;
// 			continue;
// 		}
// 		let (x, y) = (x as usize, y as usize);
// 		if y >= screen.len() {
// 			screen.resize_with(y + 1, Vec::new);
// 		}
// 		if x >= screen[y].len() {
// 			screen[y].resize(x + 1, EMPTY);
// 		}
// 		screen[y][x] = tile;
// 		if tile == BALL {
// 			*ball = (x, y);
// 		} else if tile == PADDLE {
// 			*paddle = (x, y);
// 		}
// 	}
// }

// fn print_screen(score: i64, screen: &[Vec<i64>]) {
// 	println!("Score: {}", score);
// 	for row in screen.iter() {
// 		for &tile in row.iter() {
// 			print!(
// 				"{}",
// 				match tile {
// 					EMPTY => "  ",
// 					WALL => "##",
// 					BLOCK => "[]",
// 					PADDLE => "==",
// 					BALL => "()",
// 					_ => panic!("Bad tile"),
// 				}
// 			)
// 		}
// 		println!();
// 	}
// 	println!();
// }
