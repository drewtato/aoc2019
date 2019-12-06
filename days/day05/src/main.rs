use intcode::{parse_file, run};

fn main() {
	let mut program = parse_file("inputs/day05.txt").unwrap();

	// println!("{:?}", program);

	let mut program2 = program.clone();
	let res = run(&mut program, &mut vec![1].into_iter()).expect("Program 1 failed");
	println!("{}", res[res.len() - 1]);
	let res2 = run(&mut program2, &mut vec![5].into_iter()).expect("Program 2 failed");
	println!("{}", res2[0]);
}
