#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

const DAY: &str = "inputs/day22.txt";
use std::collections::HashMap;
use std::fs::read_to_string;
use std::mem::swap;

fn main() {
	let process: Vec<Techniques> = read_to_string(DAY)
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			let words: Vec<&str> = l.trim().split_ascii_whitespace().collect();
			match words[0] {
				"deal" => match words[1] {
					"into" => DealIntoNewStack,
					"with" => DealWithIncrement(words[3].parse().unwrap()),
					_ => unreachable!(),
				},
				"cut" => Cut(words[1].parse().unwrap()),
				_ => unreachable!(),
			}
		})
		.collect();
	// let mut cards = factory_order(CARD_COUNT);
	let mut table = vec![0; CARD_COUNT];

	// for p in process.iter() {
	// 	p.shuffle(&mut cards, &mut table);
	// }
	// println!("{}", cards.iter().position(|&n| n == 2019).unwrap());
	let mut seen = HashMap::new();

	for num_cards in CARD_COUNT.. {
		let mut cards = factory_order(num_cards);
		for p in process.iter() {
			p.shuffle(&mut cards, &mut table);
		}
		// println!("{}", cards[2020]);
		if let Some(n) = seen.insert(cards[0..21].to_vec(), num_cards) {
			println!(
				"Repeat at {} and {}, difference {}, 2020 = {}",
				n,
				num_cards,
				num_cards - n,
				cards[20]
			);
		}
	}
}

const CARD_COUNT: usize = 10007;
const BIG_CARD_COUNT: usize = 119_315_717_514_047;
const BIG_SHUFFLE_COUNT: usize = 101_741_582_076_661;

type Cards = Vec<usize>;

#[derive(Debug, Clone, Copy)]
enum Techniques {
	DealIntoNewStack,
	Cut(isize),
	DealWithIncrement(usize),
}
use Techniques::*;

impl Techniques {
	fn shuffle(&self, cards: &mut Cards, table: &mut Cards) {
		match *self {
			DealIntoNewStack => {
				cards.reverse();
			}
			Cut(n) => {
				let n = (n + CARD_COUNT as isize) as usize % CARD_COUNT;
				table[(CARD_COUNT - n)..CARD_COUNT].copy_from_slice(&cards[0..n]);
				table[0..(CARD_COUNT - n)].copy_from_slice(&cards[n..CARD_COUNT]);
				swap(cards, table);
			}
			DealWithIncrement(n) => {
				let mut current_pos = 0;
				for &card in cards.iter() {
					table[current_pos] = card;
					current_pos = (current_pos + n) % CARD_COUNT;
				}
				swap(cards, table);
			}
		}
	}
}

fn factory_order(count: usize) -> Cards {
	(0..count).collect()
}
