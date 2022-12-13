// Arrays - Fixed length list of same type items

use std::mem::size_of_val as size;

pub fn run() {
	let numbers: [i32; 5] = [1, 2, 3, 4, 5];
	numbers[3] = 42; // use mut to do this...
	println!("{:?}", numbers);
	println!("{}", numbers[0]);
	// Cant add as arrays are fixed length...
	println!("Length: {}", numbers.len());
	println!("Size: {} bytes", size(&numbers));
	println!("Slice: {:?}", &numbers[..2])
}
