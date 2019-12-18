const DAY: u8 = 17;
use intcode::IntcodeProgram;

fn main() {
	let mut program = IntcodeProgram::vec_from_file(format!("inputs/day{:02}.txt", DAY)).unwrap();

	program[0] = 2;
	let input = b"A,B,A,C,A,B,C,B,C,A\nL,12,R,4,R,4,L,6\nL,12,R,4,R,4,R,12\nL,10,L,6,R,4\nn\n"
		.iter()
		.map(|&c| c as i64);
	program.add_input_from(input);

	let mut map = vec![vec![]];
	for val in &mut program {
		let val = val.unwrap() as u8;
		let last = map.len() - 1;
		if val == NEWLINE {
			if map[last].is_empty() {
				map.pop();
				break;
			}
			map.push(vec![]);
		} else {
			map[last].push(val);
		}
	}

	let mut sum = 0;
	for y in 1..(map.len() - 1) {
		for x in 1..(map[y].len() - 1) {
			let intersection = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)]
				.iter()
				.all(|(dy, dx)| {
					map[(y as isize + dy) as usize][(x as isize + dx) as usize] == SCAFFOLD
				});
			if intersection {
				sum += y * x;
			}
		}
	}

	println!("{}", sum);
	println!("{}", program.last().unwrap().unwrap());

	// print_output(&output);
}

const SCAFFOLD: u8 = b'#';
// const EMPTY: u8 = b'.';
const NEWLINE: u8 = b'\n';
// const U: u8 = b'^';
// const D: u8 = b'v';
// const L: u8 = b'<';
// const R: u8 = b'>';

// fn print_output(output: &[Vec<u8>]) {
// 	let s: String = output.iter().map(|row| {
// 		let mut partial = String::from_utf8(row.clone()).unwrap();
// 		partial.push(NEWLINE as char);
// 		partial
// 	}).collect();
// 	print!("{}", s);
// }
