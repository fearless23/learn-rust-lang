pub fn run() {
  // void return function...
  greeting("hey", "fearless");

  // Save result of a function
  let sum = add(2, 3);
  println!("Sum: {}", sum);

  // closure
  let c = 16;
  let closure = |a: i32, b: i32| add(a + b, c);
  println!("{}", closure(2, 3));
}

// Functions
fn greeting(greet: &str, name: &str) {
  println!("{}, {} nice to meet you!.", greet, name)
}

/// Add Function
fn add(a: i32, b: i32) -> i32 {
  a + b
}
