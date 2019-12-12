const DAY: u8 = 12;
use std::fs::read_to_string;

fn main() {
	let mut positions: Vec<Vec<isize>> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.chars()
				.filter(|&c| c.is_numeric() || c == '-' || c == ',')
				.collect::<String>()
				.split(',')
				.map(|n| n.parse().unwrap())
				.collect()
		})
		.collect();

	// println!("{:?}", positions);
	let mut velocities = vec![vec![0; 3]; 4];
	let mut periods = vec![None; 3];
	let (pos0, vel0) = (positions.clone(), velocities.clone());

	for i in 0..1000 {
		step(&mut positions, &mut velocities);
		check_periods(i, &positions, &velocities, &pos0, &vel0, &mut periods);
	}

	let total = positions
		.iter()
		.zip(velocities.iter())
		.map(|(ps, vs)| {
			ps.iter().cloned().map(isize::abs).sum::<isize>()
				* vs.iter().cloned().map(isize::abs).sum::<isize>()
		})
		.sum::<isize>();
	println!("{}", total);

	if !periods.iter().all(|p| p.is_some()) {
		for i in 1000.. {
			step(&mut positions, &mut velocities);
			if check_periods(i, &positions, &velocities, &pos0, &vel0, &mut periods) {
				let periods: Vec<_> = periods.iter().map(|d| d.unwrap() + 1).collect();
				let ans = periods.into_iter().fold(1, lcm);
				println!("{}", ans);
				break;
			}
		}
	}
}

fn lcm(a: usize, b: usize) -> usize {
	use gcd::Gcd;
	(a * b) / a.gcd(b)
}

fn check_periods(
	i: usize,
	p1: &[Vec<isize>],
	v1: &[Vec<isize>],
	p2: &[Vec<isize>],
	v2: &[Vec<isize>],
	periods: &mut Vec<Option<usize>>,
) -> bool {
	let mut done = false;
	for d in 0..3 {
		if periods[d].is_none()
			&& p1
				.iter()
				.zip(v1)
				.zip(p2.iter().zip(v2))
				.all(|((ms1, vs1), (ms2, vs2))| ms1[d] == ms2[d] && vs1[d] == vs2[d])
		{
			periods[d] = Some(i);
			done = periods.iter().all(|p| p.is_some());
		}
	}
	done
}

use std::cmp::Ordering::{Equal, Greater, Less};
fn step(moons: &mut Vec<Vec<isize>>, velocities: &mut Vec<Vec<isize>>) {
	for (moon1, vel) in moons.iter().zip(velocities.iter_mut()) {
		for moon2 in moons.iter() {
			for ((pos2, vel), pos) in moon2.iter().zip(vel.iter_mut()).zip(moon1.iter()) {
				*vel += match pos.cmp(pos2) {
					Equal => 0,
					Greater => -1,
					Less => 1,
				};
			}
		}
	}

	for (moon, vel) in moons.iter_mut().zip(velocities.iter()) {
		for (pos, v) in moon.iter_mut().zip(vel.iter()) {
			*pos += v;
		}
	}
}
