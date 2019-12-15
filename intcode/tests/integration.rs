mod official {
	use intcode::{HashMemory, HybridMemory, IntcodeProgram, VecMemory};

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
}

mod individual {
	use intcode::{Data, Indexer, IntcodeProgram as IP};

	fn check_mem<M>(program: &IP<M>, check: &[Data])
	where
		M: std::ops::IndexMut<Indexer, Output = Data>,
	{
		for (i, &v) in check.iter().enumerate() {
			assert_eq!(program[i], v);
		}
	}

	#[test]
	fn halt() {
		let mut program = IP::from_vec(vec![99]);
		assert!(program.next().is_none());
		assert!(program.halted());
	}

	#[test]
	fn explode() {
		let mut program = IP::from_vec(vec![]);
		program.explode();
		assert!(program.exploded());
	}

	#[test]
	fn add() {
		//                                  0  1  2  3  4   5
		let mut program = IP::from_vec(vec![1, 4, 5, 3, 99, 5]);
		program.next();
		check_mem(&program, &[1, 4, 5, 104, 99, 5]);
	}

	#[test]
	fn multiply() {
		//                                  0  1  2  3  4   5
		let mut program = IP::from_vec(vec![2, 4, 5, 3, 99, 5]);
		program.next();
		check_mem(&program, &[2, 4, 5, 495, 99, 5]);
	}

	#[test]
	fn relative() {
		let mut program = IP::from_vec(vec![109, 3, 21101, 1, 1, -2, 99]);
		assert!(program.next().is_none());
		check_mem(&program, &[109, 2, 21101, 1, 1, -2, 99]);
		assert_eq!(program.rel(), 3);
	}
}
