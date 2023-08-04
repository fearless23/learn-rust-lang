# Enums

## Chapter 1
- [Link](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

- Enums allow you to define a type by enumerating its possible variants.
- Enum is a like a shorter version of Struct and all the values represent some variant of a type.
- Struct values are created when a new struct instance is initiated.
- Enum values(variants) are defined in the definition of enum and are constant
- Methods can be defined on Enum, but it is available on a Enum instance
- Struct values are accessible by `.` notation and can be changed by methods
- Enum values(variants) are accessible by `::` notation and can`t be changed by methods
- Enum values are not values, or struct or types instead they are like scoped names
- All values of Enum are not different type instead they are variants of same type
 and their type is Enum(`Message`) itself.
```rs
enum WrongMessage {
  Quit, // if Quit is a struct defined outside it won`t work
  i32, // Rust type cannot be used as Enum value
  Move { x: i32, y: i32 }, // if Move is a struct defined outside it won`t work
  // but Move here is ok as it is defined inside enum
  Write(String), // Similar to tuple of single value
  ChangeColor(i32, i32, i32), // Similar to tuple of three values
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
// type of Message::Quit, Message::Move is `Message`
let e = MyEnum::Quit;
let f = MyEnum::Move { x: 1, y: 2 };
```
Above enum has four variants with different types:
- Quit has no data associated with it at all.
- Move has named fields, like a struct does.
- Write includes a single String.
- ChangeColor includes three i32 values.

So, We acheived to group multiple types into variations of one type with ENUM.
We could have done the same thing by defining multiple Structs like as follows
```rs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```


## Methods on ENUM
- Method on enum are similar to method on structs
- Static method for enum do not exist because `::` is for accessing variants

```rs
impl Message {
  fn call(&self) {
    // method body would be defined here
  }
}

let m = Message::Quit;
m.call();
```

So, far we have variants of an Enum can be any different kind. In practical scenarios, enums are useful
for group multiple values or types like
- IPAddr: IpV4, IpV6
- Error: Error1, Error2
- Response: Result, Error
- Optional: Value, None

## Standard Enum
- Rust standard lib has some useful enums like Option
```rs
// Rust internal Option enum
enum Option<T> {
  None,
  Some(T),
}

let e: Option<u32> = Some(5); // explicit type
let e = Some(5); // type is inferred
let e: Option<u32> = None; // type has to be mentioned for None
```