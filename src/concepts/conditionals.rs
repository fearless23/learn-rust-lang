// Conditionals - If, else

pub fn run() {
	let height: u8 = 202;

	if height >= 183 {
		println!("Hey!, You are above 6 feet tall.")
	} else {
		println!("Hey!, You are below 6 feet tall.")
	}

	// If, else, else if
	// >, <, ==, <=, >=
	// &&, ||

	// Shortand If
	let income = 20000;
	let status = if income >= 200000 { "Rich" } else { "Poor" };
	println!("Your financial status is {}", status);

	// Infinite loop
	let mut c = 0;

	loop {
		c += 1;
		println!("c: {}", c);
		if c == 7 {
			break;
		}
	}

	println!("{}", c);

	// While loop (FizzBuzz)
	let mut i = 1;
	while i <= 100 {
		if i % 15 == 0 {
			println!("{} FIZZ BUZZ", i);
		} else if i % 3 == 0 {
			println!("{} BUZZ", i)
		} else if i % 5 == 0 {
			println!("{} FIZZ", i);
		} else {
			println!("{} ....", i);
		}
		i += 1;
	}

	// Range
	for x in 0..100 {
		println!("{}", x)
	}
}
