fn main() {
	let input: Vec<isize> = std::fs::read_to_string("inputs/day05.txt")
		.unwrap()
	// let input: Vec<isize> = "3,9,8,9,10,9,4,9,99,-1,8"
		.trim()
		.split(',')
		.map(|n| n.trim().parse().unwrap())
		.collect();

	// println!("{:?}", input);

	let input2 = input.clone();
	run(input, 1);
	run(input2, 5);
}

fn run(mut code: Vec<isize>, input: isize) -> Vec<isize> {
	let mut pc = 0;
	loop {
		let mut modes = Vec::new();
		let mut op = code[pc];
		modes.push(op % 100);
		op /= 100;
		for _ in 0..3 {
			modes.push(op % 10);
			op /= 10;
		}
		// println!("{} {} {:?}", pc, code[pc], modes);
		pc = match modes[0] {
			1 => {
				let (mut a, mut b, c) = (code[pc + 1], code[pc + 2], code[pc + 3]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				code[c as usize] = a + b;
				4 + pc
			}
			2 => {
				let (mut a, mut b, c) = (code[pc + 1], code[pc + 2], code[pc + 3]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				code[c as usize] = a * b;
				4 + pc
			}
			3 => {
				let w = code[pc + 1] as usize;
				code[w] = input;
				println!("Inputted {}", input);
				2 + pc
			}
			4 => {
				let res = code[code[pc + 1] as usize];
				println!("{}", res);
				2 + pc
			}
			5 => {
				let (mut a, mut b) = (code[pc + 1],code[pc + 2]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				if a != 0 {
					b as usize
				} else {
					pc + 3
				}
			}
			6 => {
				let (mut a, mut b) = (code[pc + 1],code[pc + 2]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				if a == 0 {
					b as usize
				} else {
					pc + 3
				}
			}
			7 => {
				let (mut a, mut b, c) = (code[pc + 1],code[pc + 2], code[pc + 3]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				code[c as usize] = if a < b { 1 } else { 0 };
				4 + pc
			}
			8 => {
				let (mut a, mut b, c) = (code[pc + 1],code[pc + 2], code[pc + 3]);
				if modes[1] == 0 {
					a = code[a as usize];
				}
				if modes[2] == 0 {
					b = code[b as usize];
				}
				code[c as usize] = if a == b { 1 } else { 0 };
				4 + pc
			}
			99 => break,
			_ => panic!("Invalid instruction {} at {}", pc, code[pc]),
		};
	}
	code
}
