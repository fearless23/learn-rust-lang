// Vector - Flexible length list of same type items

use std::mem::size_of_val as size;

pub fn run() {
  let mut vec_nums: Vec<i32> = vec![10000000, 2, 3, 4, 5];
  vec_nums[3] = 42;
  // Add items to vector
  vec_nums.push(21);
  println!("{:?}", vec_nums);
  println!("{}", vec_nums[3]);
  println!("Length: {}", vec_nums.len());
  println!("Size: {} bytes", size(&vec_nums));
  println!("Slice: {:?}", &vec_nums[3..]);

  // Vector/Array Loops
  for x in vec_nums.iter() {
    println!("{}", x)
  }

  // Runs over vector/array allowing vals to change.
  for x in vec_nums.iter_mut() {
    // *x = *x + 1;
    //    Shorthand: 
    *x += 1
  }
  println!("{:?}", vec_nums);
}
