const DAY: u8 = 11;
use intcode::{parse_file, IntcodeIterator};
use std::collections::HashMap;

fn main() {
	let mut program =
		IntcodeIterator::new(parse_file(&format!("inputs/day{:02}.txt", DAY)).unwrap());
	let mut program2 = program.clone();
	// println!("{:?}", program);

	let mut visited_panels: HashMap<(isize, isize), i64> = HashMap::new();
	let panel_count = run_bot(&mut program, &mut visited_panels);
	println!("{}", panel_count);
	// print_map(&visited_panels, &mut 0);

	let mut visited_panels: HashMap<(isize, isize), i64> =
		[((0, 0), WHITE)].iter().cloned().collect();
	run_bot(&mut program2, &mut visited_panels);

	print_map(&visited_panels);
	println!("GREJALPR");
}

// fn print_map(visited_panels: &HashMap<(isize, isize), i64>, counter: &mut usize) {
fn print_map(visited_panels: &HashMap<(isize, isize), i64>) {
	// if *counter != 0 {
	// 	return;
	// }
	let min_x = *visited_panels
		.iter()
		.map(|((_y, x), _p)|x)
		.min()
		.unwrap();
	let max_x = *visited_panels
		.iter()
		.map(|((_y, x), _p)|x)
		.max()
		.unwrap();
	let min_y = *visited_panels
		.iter()
		.map(|((y, _x), _p)|y)
		.min()
		.unwrap();
	let max_y = *visited_panels
		.iter()
		.map(|((y, _x), _p)|y)
		.max()
		.unwrap();
	
	// let (min_x, max_x) = (-52, 42);
	// let (min_y, max_y) = (-18, 48);
	
	// println!("{:?} {:?}", (min_x, max_x), (min_y, max_y));
	for row in min_y..=max_y {
		for col in min_x..=max_x {
			eprint!("{}", match visited_panels.get(&(row, col)) {
				Some(&WHITE) => "██",
				_ => "  ",
			});
		}
		eprintln!();
	}

	// use image::{imageops::resize, FilterType::Nearest, ImageBuffer, Rgb, RgbImage};
	// let (y_size, x_size) = ((max_y - min_y + 3) as u32, (max_x - min_x + 3) as u32);
	// let mut img: RgbImage = ImageBuffer::from_pixel(x_size, y_size, Rgb([0x0f, 0x0f, 0x23]));
	// for ((y, x), p) in visited_panels.iter() {
	// 	img.put_pixel(
	// 		(x - min_x + 1) as u32,
	// 		(y - min_y + 1) as u32,
	// 		if *p == WHITE {
	// 			Rgb([0xcc, 0xcc, 0xcc])
	// 		} else {
	// 			Rgb([0x22, 0x22, 0x44])
	// 		},
	// 	);
	// }
	// let mult = 8;
	// resize(&img, x_size * mult, y_size * mult, Nearest)
	// 	.save(&format!("pics/{:05}.png", counter))
	// 	.unwrap();
	// *counter += 1;
}

fn run_bot(
	program: &mut IntcodeIterator,
	visited_panels: &mut HashMap<(isize, isize), i64>,
) -> usize {
	let mut panel_count = 0;
	// let mut counter = 0;
	let mut robot = Robot {
		direction: NORTH,
		y: 0,
		x: 0,
	};
	
	// std::fs::create_dir("pics").unwrap();

	loop {
		let new_panel = !visited_panels.contains_key(&(robot.y, robot.x));
		let current_panel = visited_panels.entry((robot.y, robot.x)).or_insert(BLACK);
		program.add_input(*current_panel);
		if let Some(new_color) = program.next() {
			*current_panel = new_color;
			if new_panel {
				panel_count += 1;
			}
		} else {
			break;
		}
		if let Some(turn) = program.next() {
			match turn {
				LEFT => robot.direction = (robot.direction + 3) % 4,
				RIGHT => robot.direction = (robot.direction + 1) % 4,
				_ => unreachable!(),
			}
		} else {
			break;
		}
		robot.forward();
		// eprintln!("{:?}", robot);
		// print_map(visited_panels, &mut counter);
	}
	// std::process::Command::new("ffmpeg").args("-hide_banner -r 60 -i pics/%05d.png -c:v h264_nvenc notes/day11.mp4".split_whitespace()).status().unwrap();
	panel_count
}

#[derive(Debug)]
struct Robot {
	direction: u8,
	y: isize,
	x: isize,
}

impl Robot {
	fn forward(&mut self) {
		match self.direction {
			NORTH => self.y -= 1,
			EAST => self.x += 1,
			SOUTH => self.y += 1,
			WEST => self.x -= 1,
			_ => unreachable!(),
		}
	}
}


const NORTH: u8 = 0;
const EAST: u8 = 1;
const SOUTH: u8 = 2;
const WEST: u8 = 3;

const LEFT: i64 = 0;
const RIGHT: i64 = 1;

const BLACK: i64 = 0;
const WHITE: i64 = 1;
