use std::collections::HashSet;
use std::fs::read_to_string;
fn main() {
	let rucksacks = get_lines();
	let score = get_score(rucksacks);
	println!("score {:?}", score);
}

fn get_lines() -> Vec<String> {
	let input = read_to_string("src/aoc2022/day3/input.txt").unwrap();
	input.lines().map(|i| i.to_string()).collect()
}

fn get_score(rucksacks: Vec<String>) -> u32 {
	let mut sum = 0;
	for rucksack in rucksacks {
		let (first, last) = rucksack.split_at(rucksack.len() / 2);
		let common_char = find_common_char(first.to_string(), last.to_string());
		let priority = match common_char {
			Some(i) => get_char_priority(i),
			None => 0,
		};
		sum += priority;
	}
	sum
}

fn find_common_char(a: String, b: String) -> Option<String> {
	let mut seta = HashSet::new();
	let mut setb = HashSet::new();
	for ca in a.chars() {
		seta.insert(ca);
	}
	for cb in b.chars() {
		setb.insert(cb);
	}
	let axx = seta.intersection(&setb).collect::<Vec<&char>>();
	if axx.is_empty() {
		return None;
	}
	Some(axx[0].to_string())
}

fn get_char_priority(i: String) -> u32 {
	match i.as_str() {
		"a" => 1,
		"b" => 2,
		"c" => 3,
		"d" => 4,
		"e" => 5,
		"f" => 6,
		"g" => 7,
		"h" => 8,
		"i" => 9,
		"j" => 10,
		"k" => 11,
		"l" => 12,
		"m" => 13,
		"n" => 14,
		"o" => 15,
		"p" => 16,
		"q" => 17,
		"r" => 18,
		"s" => 19,
		"t" => 20,
		"u" => 21,
		"v" => 22,
		"w" => 23,
		"x" => 24,
		"y" => 25,
		"z" => 26,
		"A" => 27,
		"B" => 28,
		"C" => 29,
		"D" => 30,
		"E" => 31,
		"F" => 32,
		"G" => 33,
		"H" => 34,
		"I" => 35,
		"J" => 36,
		"K" => 37,
		"L" => 38,
		"M" => 39,
		"N" => 40,
		"O" => 41,
		"P" => 42,
		"Q" => 43,
		"R" => 44,
		"S" => 45,
		"T" => 46,
		"U" => 47,
		"V" => 48,
		"W" => 49,
		"X" => 50,
		"Y" => 51,
		"Z" => 52,
		_ => 0,
	}
}

// collect is very good, can collect to any type
fn get_lines_c() {
	let input = read_to_string("src/aoc2022/day3/input.txt").unwrap();
	let _out = input
		.lines()
		.map(|i| i.to_string().chars().collect::<HashSet<char>>())
		.collect::<Vec<HashSet<char>>>();
}
