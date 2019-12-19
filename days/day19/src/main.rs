const DAY: &str = "inputs/day19.txt";
use intcode::{IntcodeProgram, Memory};

fn main() {
	let program = IntcodeProgram::vec_from_file(DAY).unwrap();

	// println!("{:?}", program);
	let mut sum = 0;
	
	for y in 0..50 {
		for x in 0..50 {
			let output = check(&program, (x, y));
			sum += output;
			// print!("{}", output);
		}
		// println!();
	}
	println!("{}", sum);
	
	let mut top = (100, 0);
	let bottom = |top: (i64, i64)| (top.0 - 99, top.1 + 99);
	while check(&program, bottom(top)) == 0 {
		top.0 += 1;
		while check(&program, top) == 0 {
			top.1 += 1;
		}
	}
	println!("{}", bottom(top).0 * 10_000 + top.1);
}

fn check<T: Memory + Clone>(program: &IntcodeProgram<T>, coords: (i64, i64)) -> i64 {
	program.clone().with_input(coords.0).with_input(coords.1).next().unwrap().unwrap()
}

// fn print_area<T: Memory + Clone>(program: &IntcodeProgram<T>, coords: (i64, i64), radius: i64) {
// 	for y in (coords.1 - radius)..=(coords.1 + radius) {
// 		for x in (coords.0 - radius)..=(coords.0 + radius) {
// 			print!("{}", if check(program, (x,y)) == 1 {
// 				"#"
// 			} else {
// 				"."
// 			});
// 		}
// 		println!();
// 	}
// }

// fn get_input() -> String {
// 	let mut s = String::new();
// 	std::io::stdin().read_line(&mut s).unwrap();
// 	s.trim().to_owned()
// }