pub fn run() {
  let s = String::from("Hello");
  let mut v = s + "a";

  v.push('a');
  v.push_str("world");

  println!("{}", v);
  
  let a = v.contains("world");
  let b = v.capacity();
  // creates new strin, but only applicable on
  // v not s
  let c = v.replace("world", "hello");

  println!("a: {}", a);
  println!("b: {}", b);
  println!("c: {}, v: {}", c, v);

  // Loop over string
  for word in v.split("") {
    println!("{}", word);
  }

  // assert_eq!(a, false)
}
