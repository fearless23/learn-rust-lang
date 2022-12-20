fn main() {
	let numbers = get_numbers_empty_as_zero();
	// println!("numbers => {:?}", numbers);
	let sum = sum_numbers_in_groups(numbers);
	println!("Max is {:?}", sum);
}

fn get_numbers() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().filter_map(|i| match i.parse::<u64>() {
		Ok(i) => Some(i),
		Err(_) => None,
	});
	input.collect::<Vec<u64>>()
}

fn get_numbers_empty_as_zero() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().map(|i| i.parse::<u64>().unwrap_or(0));
	input.collect::<Vec<u64>>()
}

fn sum_numbers(numbers: Vec<u64>) -> u64 {
	let mut sum: u64 = 0;
	for num in numbers {
		sum += num;
	}
	sum
}

fn sum_numbers_in_groups(numbers: Vec<u64>) -> u64 {
	// let groups: Vec<u64> = Vec::new();
	let mut max_group_sum = 0;
	let mut sum: u64 = 0;
	for num in numbers {
		if num == 0 {
			// set max_group_score
			if sum > max_group_sum {
				max_group_sum = sum;
			}

			// groups.push(sum);
			sum = 0;
		} else {
			sum += num;
		}
	}
	max_group_sum
}

fn top_three_groups(numbers: Vec<u64>) -> u64 {
	// let groups: Vec<u64> = Vec::new();
	let mut smallest_group: u64 = 0;
	let mut middle_group: u64 = 0;
	let mut largest_group: u64 = 0;

	let mut sum: u64 = 0;
	for num in numbers {
		if num == 0 {
			// summed the group
			if sum > smallest_group {
				// accept
			}
		} else {
			sum += num;
		}
	}
	smallest_group + middle_group + largest_group
}
