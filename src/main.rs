fn main() {
	let numbers = get_numbers();
	let sum = top_three_groups(numbers);
	println!("Top three total is {:?}", sum);
}

fn get_numbers() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().map(|i| i.parse::<u64>().unwrap_or(0));
	input.collect::<Vec<u64>>()
}

fn top_three_groups(numbers: Vec<u64>) -> u64 {
	let mut groups: Vec<u64> = Vec::new();
	let mut sum: u64 = 0;
	for num in numbers {
		if num == 0 {
			groups.push(sum);
			sum = 0;
		} else {
			sum += num;
		}
	}

	let mut s: u64 = groups[0];
	let mut m: u64 = groups[1];
	let mut l: u64 = groups[2];

	for num in groups {
		if num > s {
			if num <= m {
				s = num;
			} else {
				s = m;
				if num <= l {
					m = num;
				} else {
					m = l;
					l = num;
				}
			}
		}
	}
	s + m + l
}
