pub fn working() {
	let string_heap = String::from("sita.ram");
	// Error - the size for values of type `str` cannot be known at compilation time
	// let this_is_str = string_heap[5..]; // this is str
	let string_slice = &string_heap[5..];
	println!("string_slice from String --> {}", string_slice);
	let entire_string_slice = &string_heap[..];
	println!(
		"entire_string_slice from String --> {}",
		entire_string_slice
	);
	// This is like entire_string_slice but for hard-coded and have known size thus stored on Stack
	let string_literal = "ram";
	println!("string_literal --> {}", string_literal);

	let string_heap_pointer = &string_heap;
	println!("string_heap_pointer --> {}", string_heap_pointer);

	// BORROWING
	/*
	1. Assign String to new variable
	let borrow_String = string_heap;
	let a = string_heap.split('.'); // ERROR - as it is moved
	*/
	/*
	2. Assign &str to new variable
	*/
	// since &str is a pointer, new pointer is created to same portion of String
	let _borrow_slice = string_slice;
	// calling split is also ok, as it do not change original String
	let _a = string_slice.split('.'); // ERROR - as it is moved

	/*
	3. add &str and &str
	let string_literal = "ram" + "sita"; // ERROR
	Error: cannot add `&str` to `&str`
	string concatenation requires an owned `String` on the left
	*/

	/*
	4. add String and &str = String works
	*/
	let x = "ram".to_owned(); // Creates owned data from borrowed data, usually by cloning
	let string_literal = x + "d"; // WORKS

	// let y = x.split("ff"); // Borrow of moved value

	// METHODS
	/*
	1. clone a &str
			let a = string_slice.clone();
			- cloning a reference/pointer is of no use
			- using `clone` on a double-reference; this will copy the reference of type `&str`
			instead of cloning the inner type
	*/
	/*
	2. spliting a String
	*/
	let a = string_heap.split('.');
	for s in a {
		// after splitting a String - each part is &str
		// because: splitting into part with create reference at various index in original String
		println!("{}", s);
	}
	/*
	3. spliting a &str
	*/
	let a = string_literal.split('.');
	for s in a {
		// after splitting a &str - each part is &str
		// because: splitting a &str with create reference at different indices comapred to earlier
		// if &str = 5..10
		// &str.split => 5..7 and 8..10
		println!("{}", s);
	}
}
