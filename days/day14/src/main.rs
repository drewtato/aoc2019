const DAY: u8 = 14;
use std::fs::read_to_string;
use std::collections::HashMap;

type ReactionMap<'a> = HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>;

fn main() {
	let input = read_to_string(format!("inputs/day{:02}.txt", DAY)).unwrap();
	let reactions: ReactionMap = input
		.trim()
		.lines()
		.map(|l| {
			let leftright: Vec<_> = l.split(" => ").collect();
			let left = leftright[0].split(',').map(|pair| {
				let pair: Vec<_> = pair.split_whitespace().collect();
				let num = pair[0].parse().unwrap();
				let ident = pair[1];
				(num, ident)
			}).collect();
			let right: Vec<_> = leftright[1].split_whitespace().collect();
			let num = right[0].parse().unwrap();
			let ident = right[1];
			(ident, (num, left))
		}).collect();

	// println!("{:?}", input);
	
	let min_ore = find_ore("FUEL", 1, &mut HashMap::new(), &reactions);
	
	println!("{}", min_ore);
	
	let max_ore = find_ore("FUEL", 1_000_000_000, &mut HashMap::new(), &reactions);
	let ratio = max_ore as f64 / 1_000_000_000.0;
	
	let mut guess = (1e12 / ratio) as usize + 1;
	
	while find_ore("FUEL", guess, &mut HashMap::new(), &reactions) <= 10_usize.pow(12) {
		eprintln!("Added");
		guess += 2;
	}
	while find_ore("FUEL", guess, &mut HashMap::new(), &reactions) > 10_usize.pow(12) {
		eprintln!("Subbed");
		guess -= 1;
	}
	println!("{}", guess);
}

// fn get_input() -> String {
// 	let mut s = String::new();
// 	std::io::stdin().read_line(&mut s).unwrap();
// 	s
// }

fn find_ore<'a>(item: &'a str, mut count: usize, extras: &mut HashMap<&'a str, usize>, reactions: &'a ReactionMap) -> usize {
	if item == "ORE" {
		return count;
	}
	if let Some(n) = extras.remove(item) {
		use std::cmp::Ordering::{Greater, Equal, Less};
		match count.cmp(&n) {
			Greater => count -= n,
			Equal => return 0,
			Less => { extras.insert(item, n - count); return 0},
		}
	}
	let (reaction_count, reactors) = &reactions[item];
	let needed_reactions = match count % reaction_count {
		0 => count / reaction_count,
		x => {
			*extras.entry(item).or_default() += reaction_count - x;
			count / reaction_count + 1
		}
	};
	reactors.iter().map(|(reactor_count, reactor)| {
		find_ore(reactor, needed_reactions * reactor_count, extras, reactions)
	}).sum::<usize>()
}
