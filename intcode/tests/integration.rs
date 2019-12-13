use intcode::{IntcodeProgram, HybridMemory, VecMemory, HashMemory};

#[test]
fn day09p1_hybrid() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HybridMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(1);
	assert_eq!(3906448201, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p2_hybrid() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HybridMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(2);
	assert_eq!(59785, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p1_vec() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<VecMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(1);
	assert_eq!(3906448201, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p2_vec() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<VecMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(2);
	assert_eq!(59785, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p1_hash() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HashMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(1);
	assert_eq!(3906448201, computer.next().unwrap()?);
	Ok(())
}
#[test]
fn day09p2_hash() -> Result<(), Box<dyn std::error::Error>> {
	let mut computer: IntcodeProgram<HashMemory> =
		IntcodeProgram::from_file("../inputs/day09.txt")?.with_input(2);
	assert_eq!(59785, computer.next().unwrap()?);
	Ok(())
}
