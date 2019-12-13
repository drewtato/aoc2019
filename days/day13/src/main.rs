const DAY: u8 = 13;
use intcode::{IntcodeIterator, parse_file};

const EMPTY: i64 = 0;
const WALL: i64 = 1;
const BLOCK: i64 = 2;
const HZ_PAD: i64 = 3;
const BALL: i64 = 4;

fn main() {
	let mut program = IntcodeIterator::new(parse_file(&format!("inputs/day{:02}.txt", DAY)).unwrap());
	program.program.insert(0, 2);
	
	loop {
		
		draw_screen(&all);
	}
	// println!("{:?}", all);
	
	// let blocks = all.chunks(3).map(|chunk| chunk[2]).filter(|&tile| tile == BLOCK).count();
	// println!("{}", blocks);
	
	
}

fn draw_screen(tiles: &[i64]) {
	let mut screen: Vec<Vec<i64>> = Vec::new();
	for chunk in tiles.chunks(3) {
		let (x, y, tile) = (chunk[0], chunk[1], chunk[2]);
		let (x, y) = (x as usize, y as usize);
		if y >= screen.len() {
			screen.resize_with(y + 1, Vec::new);
		}
		if x >= screen[y].len() {
			screen[y].resize(x + 1, EMPTY);
		}
		screen[y][x] = tile;
	}
	for row in screen.into_iter() {
		for tile in row.into_iter() {
			print!("{}", match tile {
				EMPTY => "  ",
				WALL => "##",
				BLOCK => "[]",
				HZ_PAD => "==",
				BALL => "()",
				_ => panic!("Bad tile"),
			})
		}
		println!();
	}
	println!();
}