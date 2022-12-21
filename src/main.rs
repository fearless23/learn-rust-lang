fn main() {
	let moves = get_moves();
	println!("MOVES {:?}", moves);
  let score = calculate_score(moves);
	println!("SCORE {:?}", score);
}

fn get_moves() -> Vec<Vec<String>> {
	let input = std::fs::read_to_string("src/aoc2022/day2/demo.txt").unwrap();
	let input = input
		.lines()
		.map(|i| i.split(' ').map(|j| j.to_string()).collect());
	input.collect::<Vec<Vec<String>>>()
}

fn calculate_score(moves:Vec<Vec<String>>) -> u32 {
}
