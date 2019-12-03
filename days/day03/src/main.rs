use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
	let input: Vec<Vec<(char, i64)>> = read_to_string("inputs/day03.txt")
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.split(',')
				.map(|ins| {
					let dir = ins.chars().next().unwrap();
					let number: i64 = ins.chars().skip(1).collect::<String>().parse().unwrap();
					(dir, number)
				})
				.collect()
		})
		.collect();

	// let input: Vec<Vec<_>> = "R8,U5,L5,D3\nU7,R6,D4,L4\n"
	// 	.trim()
	// 	.lines()
	// 	.map(|l| {
	// 		l.split(',').map(|ins| {
	// 			let dir = ins.chars().next().unwrap();
	// 			let number: i64 = ins.chars().skip(1).collect::<String>().parse().unwrap();
	// 			(dir, number)
	// 		}).collect()
	// 	})
	// 	.collect();
	// println!("{:?}", input);

	let first = create_hash(input[0].iter());
	let second = create_hash(input[1].iter());

	/* Image stuff
	let min_x = first
		.keys()
		.chain(second.keys())
		.map(|(_, x)| x)
		.min()
		.unwrap();
	let min_y = first
		.keys()
		.chain(second.keys())
		.map(|(y, _)| y)
		.min()
		.unwrap();
	let max_x = first
		.keys()
		.chain(second.keys())
		.map(|(_, x)| x)
		.max()
		.unwrap();
	let max_y = first
		.keys()
		.chain(second.keys())
		.map(|(y, _)| y)
		.max()
		.unwrap();

	fn pixmap(val: i64) -> u32 {
		(val / 16 + 10) as u32
	}
	use image::{ImageBuffer, RgbImage, imageops};
	let mut img: RgbImage = ImageBuffer::from_fn(
		pixmap(max_x - min_x) + 21,
		pixmap(max_y - min_y) + 21,
		|_, _| image::Rgb([0x0f, 0x0f, 0x23]),
	);
	let offset = 3;
	for &(y, x) in first.keys().filter(|k| second.contains_key(k)) {
		for dx in -offset..=offset {
			for dy in -offset..=offset {
				let px = &mut img[(
					(pixmap(x - min_x) as i32 + dx) as u32,
					(pixmap(y - min_y) as i32 + dy) as u32,
				)];
				px[0] += 51;
				px[1] += 51;
				px[2] += 76;
			}
		}
		// println!("{:?}", (y,x));
	}
	for &(y, x) in first.keys() {
		img[(pixmap(x - min_x), pixmap(y - min_y))][0] = 100;
	}
	for &(y, x) in second.keys() {
		img[(pixmap(x - min_x), pixmap(y - min_y))][1] = 100;
	}
	for &(y, x) in first.keys().filter(|k| second.contains_key(k)) {
		img[(pixmap(x - min_x), pixmap(y - min_y))] = image::Rgb([0,0,0]);
	}
	*/
	
	let ans = first
	.keys()
		.filter(|k| second.contains_key(k))
		.min_by_key(|(y, x)| y.abs() + x.abs())
		.unwrap();

	println!("{}", ans.0.abs() + ans.1.abs());

	let ans2 = first
		.iter()
		.filter_map(|(k, v)| second.get(k).map(|v2| (k, v + v2)))
		.min_by_key(|(_, v)| *v)
		.unwrap();

	println!("{}", ans2.1);

	/* More image stuff
	let (y, x) = ans;
	for dx in -offset..=offset {
		for dy in -offset..=offset {
			img[(
				(pixmap(x - min_x) as i32 + dx) as u32,
				(pixmap(y - min_y) as i32 + dy) as u32,
			)] = image::Rgb([0xff, 0xff, 0x66]);
			// )] = image::Rgb([255,255,255]);
		}
	}
	img[(pixmap(x - min_x), pixmap(y - min_y))] = image::Rgb([0,0,0]);
	let (y, x) = ans2.0;
	for dx in -offset..=offset {
		for dy in -offset..=offset {
			img[(
				(pixmap(x - min_x) as i32 + dx) as u32,
				(pixmap(y - min_y) as i32 + dy) as u32,
			)] = image::Rgb([0xff, 0xff, 0x66]);
			// )] = image::Rgb([255,255,255]);
		}
	}
	img[(pixmap(x - min_x), pixmap(y - min_y))] = image::Rgb([0,0,0]);
	img[(pixmap(0 - min_x), pixmap(0 - min_y))] = image::Rgb([255, 255, 255]);
	let (dimx, dimy) = img.dimensions();
	let img = imageops::resize(&img, dimx * 2, dimy * 2, image::FilterType::Nearest);
	img.save("notes/day03wires.png").unwrap();
	*/
}

fn create_hash<'a, I>(list: I) -> HashMap<(i64, i64), usize>
where
	I: Iterator<Item = &'a (char, i64)>,
{
	let mut map = HashMap::new();
	let (mut x, mut y): (i64, i64) = (0, 0);
	let mut dist = 0;
	for (dir, num) in list {
		let (off_y, off_x) = match dir {
			'R' => (0, 1),
			'L' => (0, -1),
			'U' => (-1, 0),
			'D' => (1, 0),
			_ => panic!("Invalid direction"),
		};
		for _ in 0..*num {
			dist += 1;
			x += off_x;
			y += off_y;
			map.entry((x, y)).or_insert(dist);
		}
	}
	map
}
