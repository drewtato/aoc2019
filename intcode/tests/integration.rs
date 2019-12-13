use intcode::{HybridMemory, IntcodeProgram};

#[test]
fn day09p1() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HybridMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(1);
	assert_eq!(3906448201, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p2() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HybridMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(2);
	assert_eq!(59785, computer.next().unwrap()?);
	Ok(())
}
