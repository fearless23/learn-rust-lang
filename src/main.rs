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

	let mut sm: u64 = groups[0];
	let mut md: u64 = groups[1];
	let mut lg: u64 = groups[2];

	for num in groups {
		if num < sm {
			continue;
		}
		if num <= md {
			sm = num;
		} else {
			sm = md;
			if num <= lg {
				md = num;
			} else {
				md = lg;
				lg = num;
			}
		}
	}
	sm + md + lg
}
