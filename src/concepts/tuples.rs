pub fn run() {
  let person: (&str, &str, i32) = ("jassi", "fearless", 23);

  println!(
    "person tuple vals: {}, {}, {}",
    person.0, person.1, person.2
  );
}
