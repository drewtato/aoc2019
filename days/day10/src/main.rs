const DAY: u8 = 10;
use gcd::Gcd;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
	// let asteroids: HashSet<(usize, usize)> = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"
	// let asteroids: HashSet<(usize, usize)> = ".#..#\n.....\n#####\n....#\n...##"
	// let asteroids: HashSet<(usize, usize)> = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##"
	let asteroids: HashSet<(usize, usize)> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		.trim()
		.lines()
		.enumerate()
		.map(|(y, l)| {
			l.chars().enumerate().filter_map(move |(x, n)| match n {
				'#' => Some((y, x)),
				_ => None,
			})
		})
		.flatten()
		.collect();

	// println!("{:?}", asteroids);

	let best = asteroids
		.iter()
		// .filter(|&coords| coords.0 == 3 && coords.1 == 8)
		.map(|&coords| (detectable(coords, &asteroids), coords))
		.max_by_key(|&((det, ref _list), _)| det)
		.unwrap();
	println!("{}", (best.0).0);

	let (best_y, best_x) = best.1;
	let mut list: Asteroids = (best.0).1;
	for (_, sublist) in list.iter_mut() {
		sublist.sort_unstable_by_key(|&(ast_y, ast_x)| {
			(best_y as isize - ast_y as isize).abs() + (best_x as isize - ast_x as isize).abs()
		});
	}
	let mut sorted_list: Vec<_> = list
		.into_iter()
		.map(|((quad, y, x), sublist)| {
			let secondary = match quad {
				2 | 4 | 6 | 8 => y as f32 / x as f32,
				_ => 0.0,
			};
			(quad, secondary, y, x, VecDeque::from(sublist))
		})
		.collect();

	sorted_list.sort_unstable_by(|(quad, secondary, ..), (quad2, secondary2, ..)| {
		quad.cmp(quad2)
			.then(secondary.partial_cmp(secondary2).unwrap())
	});

	// for sublist in sorted_list.iter() {
	// 	println!("{:?}", (sublist.0, &sublist.4));
	// }
	let mut current = 0;
	let mut in_order = (1000, 1000);

	// let rows = asteroids.iter().map(|(y, _x)| y).max().unwrap() + 1;
	// let cols = asteroids.iter().map(|(_y, x)| x).max().unwrap() + 1;
	// let mut img_num = 0;
	// create_folder();
	
	for _ in 0..200 {
		// save_image(&sorted_list, rows, cols, &mut img_num, best_y, best_x, None);
		while sorted_list[current].4.is_empty() {
			current += 1;
			current %= sorted_list.len();
		}
		in_order = sorted_list[current].4.pop_front().unwrap();
		current += 1;
		current %= sorted_list.len();
		
	}
	println!("{}{:02}", in_order.1, in_order.0);
	// save_image(&sorted_list, rows, cols, &mut img_num, best_y, best_x, Some(in_order));

	// while sorted_list.iter().any(|item| !item.4.is_empty()) {
	// 	while sorted_list[current].4.is_empty() {
	// 		current += 1;
	// 		current %= sorted_list.len();
	// 	}
	// 	sorted_list[current].4.pop_front().unwrap();
	// 	current += 1;
	// 	current %= sorted_list.len();

	// 	save_image(&sorted_list, rows, cols, &mut img_num, best_y, best_x, None);
	// }
	// create_video();
}

// fn create_folder() {
// 	std::fs::create_dir("pics").unwrap();
// }

// fn create_video() {
// 	std::process::Command::new("ffmpeg.exe")
// 		.args("-hide_banner -i pics/%04d.png -c:v h264_nvenc -b:v 2M notes/day10.mp4 -y".split_whitespace())
// 		.status().unwrap();
	
// 	std::fs::remove_dir_all("pics").unwrap();
// }

// use image::{imageops::resize, FilterType::Nearest, ImageBuffer, Rgb, RgbImage};
// fn save_image(
// 	asteroids: AstSlice,
// 	rows: usize,
// 	cols: usize,
// 	img_num: &mut usize,
// 	best_y: usize,
// 	best_x: usize,
// 	answer: Option<(usize,usize)>
// ) {
// 	let mut img: RgbImage =
// 		ImageBuffer::from_pixel(cols as u32 + 2, rows as u32 + 2, Rgb([0x0f, 0x0f, 0x23]));
// 	for (.., points) in asteroids {
// 		for &(y, x) in points.iter() {
// 			img.put_pixel(x as u32 + 1, y as u32 + 1, Rgb([0xcc, 0xcc, 0xcc]));
// 		}
// 	}
// 	img.put_pixel(
// 		best_x as u32 + 1,
// 		best_y as u32 + 1,
// 		Rgb([0xff, 0xff, 0x66]),
// 	);
// 	let mult = 16;
// 	if let Some((y,x)) = answer {
// 		img.put_pixel(
// 			x as u32 + 1,
// 			y as u32 + 1,
// 			Rgb([0xff, 0x00, 0x00]),
// 		);
// 		let final_img = resize(&img, (cols as u32) * mult, (rows as u32) * mult, Nearest);
// 		for _ in 0..30 {
// 			final_img.save(format!("pics/{:04}.png", img_num)).unwrap();
// 			*img_num += 1;
// 		}
// 	} else {
// 		let final_img = resize(&img, (cols as u32) * mult, (rows as u32) * mult, Nearest);
// 		final_img.save(format!("pics/{:04}.png", img_num)).unwrap();
// 		*img_num += 1;
// 	}
// }
// type AstSlice<'a> = &'a [(usize, f32, isize, isize, VecDeque<(usize, usize)>)];

type Asteroids = HashMap<(usize, isize, isize), Vec<(usize, usize)>>;

fn detectable((y, x): (usize, usize), asteroids: &HashSet<(usize, usize)>) -> (usize, Asteroids) {
	let mut seen_paths: Asteroids = HashMap::new();
	for &(ast_y, ast_x) in asteroids.iter() {
		let (dy, dx) = (ast_y as isize - y as isize, ast_x as isize - x as isize);
		let quadrant = match (dy.checked_div(dy.abs()), dx.checked_div(dx.abs())) {
			(Some(1), Some(1)) => 4,
			(Some(1), Some(-1)) => 6,
			(Some(-1), Some(-1)) => 8,
			(Some(-1), Some(1)) => 2,
			(None, Some(1)) => 3,
			(None, Some(-1)) => 7,
			(Some(1), None) => 5,
			(Some(-1), None) => 1,
			(None, None) => continue,
			_ => unreachable!(),
		};
		let (new_dy, new_dx) = (dy.abs() as usize, dx.abs() as usize);
		let (reduced_y, reduced_x) = if quadrant % 2 == 0 {
			let greatest_common_divisor = new_dy.gcd(new_dx);
			(
				dy / greatest_common_divisor as isize,
				dx / greatest_common_divisor as isize,
			)
		} else {
			(0, 0)
		};
		seen_paths
			.entry((quadrant, reduced_y, reduced_x))
			.or_default()
			.push((ast_y, ast_x));
	}
	(seen_paths.len(), seen_paths)
}
