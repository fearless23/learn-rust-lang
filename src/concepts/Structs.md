# Struct

## chapter 1
- [Link](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- Struct is a data-structure similar to class in Javascript, i.e it holds multiple related values and methods.
- It holds multiple related values together and then optionally can have methods to change those values.

```rs
struct Color { red: u8, green: u8, blue: u8 };
struct RGBColor(u8, u8, u8);
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

let color1 = Color {
  red: 200,
  green: 15,
  blue: 123,
};

let color2 = RGBColor(200,15,123);

let user = User {
  active: true,
  username: String::from("someusername123"),
  email: String::from("someone@example.com"),
  sign_in_count: 1,
};
```

## Accessing Property
- Access any field from Struct using Dot notation like Javascript object
- Field Init Shorthand: Can use field:value -> field if field and value name are same (same as Javascript)
  - { age: age, name: "Jassi", email: email } -> { age, name:"Jassi", email }
- Struct Update Syntax: Structs can be de-structured (like Javascript object) into another struct, using double-dot `..` notation
  - Note: `..` comes at the end of struct and will use fields which are not declared, see example
  - This behaviour is different from Javascript.

```rs
let user1 // consider user1 from previous examples
let user2 = User {
  email: String::from("another@example.com"),
  ..user1 // email field is ignored from user1
};
```

### Unit Struct
```rs
#[derive(Debug)]
struct UnitLikeStruct;

fn main() {
  let unit_like_struct = UnitLikeStruct;
  println!("{:?}s are fun!", unit_like_struct);
}
```

### Print
- The primitive types have Display and Debug Trait implemented
- Other types have Debug Trait, thus can be printed using `{:?}` syntax
- But, Struct do not have any trait implemented by default, so add `#[derive(Debug)]` at top to print Structs in some default manner using `{:?}` or `{:#?}`


## Ownership
- for now let go with struct owns their data, if we pass reference we need lifetime then, with it beyond scope for now.


## Implementing Methods
- Methods can be created for strucut using impl Struct synatax as shown below.
- Methods are accessed using dot notation just like other languages.
- `&self` is reference to instance itself (like this in Javascript)
- `&self` is shorthand for `self: &Self`, whereas `Self` is alias for current `Struct`
- first parameter to every method is `&self`, but when calling method we dont need to pass anything for first parameter.
- `&self` is reference to struct, thus only reading is done
- `&mut self` is mutable reference to struct, by this we can change the struct values
- When passing struct as argument to function or method of another struct, we use type of argument `&Rectangle`
  - Thus, `&self` is only valid inside and as a first argument

```rs
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// &self is shorthand for `self: &Self`,
// whereas `Self` is alias for `Rectangle`
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    // &Rectangle is reference to other struct (can be of same type or any other type)
    self.width > other.width && self.height > other.height
  }
}
```

## Associated Functions
- Associated functions are methods on `impl` block of a struct which do not need `&self` reference
- These are similar to static methods of a class in other languages.
- These methods are accessible using `::` notation

```rs
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }
  fn current_time() -> u8 {
    234
  }
}

let time = Rectangle::current_time() // works

let rect = Rectangle { width:1, height:2 };
let time = rect.current_time(); // Error, as current_time is associated function not a method on rect
```

## Multiple impl blocks
- The multiple impl blocks can be used to implement methods, the usefullness of this way is explored later
```rs
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
impl Rectangle {
  fn perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }
}
impl Rectangle {
  fn current_time() -> u8 {
    234
  }
}
```

## Access Struct method in other method
- use `Self.method()`
```rs
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn print_area(&self) -> u32 {
    let area = Self.area(self);
    println!("{}",area);
  }
}

```