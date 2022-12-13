/// Red, Green, Blue Color
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

/// Tuple Struct for color
struct TupleColor(u8, u8, u8);

/// Person Struct
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  /// Returns Full Name of Person
  fn get_full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  /// Set New last name of Person
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
}

pub fn run() {
  let c = Color {
    red: 255,
    green: 4,
    blue: 0,
  };

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let ct = TupleColor(255, 4, 0);

  println!("Color: {} {} {}", ct.0, ct.1, ct.2);

  let mut p = Person {
    first_name: "Jaspreet".to_string(),
    last_name: "Singh".to_string(),
  };

  println!("F: {} L: {}", p.first_name, p.last_name);
  println!("FN {}", p.get_full_name());
  p.set_last_name("Fearless");
  println!("FN {}", p.get_full_name());
}
