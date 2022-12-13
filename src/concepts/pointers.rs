/*
  Reference Pointers - Point to a value stored in memory.
  Pointer itself is the memory address like 0x7ffc44f563c
  Printing a pointer with {:p} gives memoery address.
  Printing a pointer with {:?} or {}, printing value at that address.
*/

pub fn run() {
  println!("Example 1: Array assignment");
  let arr1 = [1, 2, 3];
  println!("arr1 pointer is {:p}", &arr1);
  let mut arr2 = arr1;
  // above assignment creates a copy...
  arr2[2] = 30;
  println!("Values {:?} {:?}", arr1, arr2);

  println!("\nExample 2: Integer assignment");
  let mut a: i32 = 1;
  let x = a;
  println!("a is {}", a);
  println!("x = a; x is {}", x);
  a = 3;
  println!("a=3; a is {}", a);
  println!("a=3; x is {}", x);

  println!("\nExample 3: &str assignment");
  let mut s = "hello";
  let v = s;
  println!("s is {}", s);
  println!("v = s; v is {}", v);
  s = "World";
  println!("s changed; s is {}", s);
  println!("s changed; v is {}", v);

  /*
  println!("\nExample 4: ImMutable String assignment");
  let m = String::from("Hello");
  println!("m value {}", m);
  println!("n is pointer of m; n = &m");
  let n = &mut m;
  *n = "roll".to_ascii_uppercase();
  println!("n is a memory address = {:p}", n);
  println!("value stored at n is *n = {}", *n);

  println!("\nExample 4: Mutable String assignment");
  let mut r = String::from("Hello");
  println!("r mutable value {}", r);

  println!("n is pointer of m; n = &m");
  let mut mut_p_r = &r;
  let immut_p_R = &r;

  // *n.push_str(" world");
  // n.push_str()
  t = "roll".to_ascii_lowercase();
  println!("m changed {}", *t);
  println!("n is a memory address = {:p}", t);
  println!("value stored at n is *n = {}", *t);

  println!("\nExample 5: Immutable Vector assignment");
  // Im-mutable vector vec1
  let vec1 = vec![10000000, 2, 3, 4, 5];
  println!("vec1 vallue {:?}", vec1);
  println!("vec1 address {:p}", &vec1);

  println!("--- vec2 = &vec1; assignment");
  let vec2 = &vec1;
  println!("vec2 address: {:p}", vec2);
  println!("vec2 value *vec2 {:?}", *vec2);

  println!("Example 6: Mutable Vector assignment");
  // Im-mutable vector vec1
  let mut mut_vec1 = vec![10000000, 2, 3, 4, 5];
  println!("mut_vec1 vallue {:?}", mut_vec1);
  println!("mut_vec1 address {:p}", &mut_vec1);

  println!("--- mut_vec2 = &mut_vec1; assignment");
  let vec2 = &mut_vec1;
  mut_vec1[1] = 42;
  println!("vec2 address: {:p}", vec2);
  println!("vec2 value *vec2 {:?}", *vec2);
  */

  // Each case below::
  /*
  // a - I, R - I
  let b = vec![40, 80, 60];
  let a = vec![1, 2, 3];

  let x = &a;
  println!("x is pointer to {:p}", x);

  let mut mut_x = &b;
  println!("mut_x is pointing to b -> {:p}, value: {:?}", mut_x, *mut_x);
  mut_x = &a;
  println!("mut_x now points to a -> {:p}, value: {:?}", mut_x, *mut_x);
  */
  /*
  // a - M, R - I
  let b = vec![40, 80, 60];
  let mut a = vec![1, 2, 3];


  let x = &a;
  println!("x points to b -> {:p}, value: {:?}", x, *x);
  // Since, a is mutable
  println!("x points to b -> {:p}, value: {:?}", x, *x);

  let mut mut_x = &b;
  println!("mut_x is pointing to b -> {:p}, value: {:?}", mut_x, *mut_x);
  mut_x = &a;
  println!("mut_x now points to a -> {:p}, value: {:?}", mut_x, *mut_x);
  */
}
