fn main() {
	let name = "akin";
	let mut age = 35;
	age += 1;
	if name.len() > 0 {
		println!(
			"Hello, world!, my name is {}, and I am {} years old",
			name, age
		);
	}
}
