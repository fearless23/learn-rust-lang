# Strings

- String in rust is little different from string in other languages
- For our understanding, lets define few of our terms
  - String: `String`
  - String Reference: `&String`
  - String slice: `str`
  - String Reference or String Literal: `&str`

- Commonly used are `String` and `&str`

## String (String Builder)
The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type
- Growable (append, delete)
- Mutable (with mut keyword)
- Owned (creator owns the variable)

## str (str)
- Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. 
- `str` is a reference to some UTF-8 encoded string data stored elsewhere.
- Since, `String` is growable it is stored somewhere on a heap (or similar DS), whereas `str` represent a slice of that `String`, thus it stored on stack for easy and fast access.
- It is also called String Literal because defining `let a = "Hello World!"` is literal representation of a string.
  - Type of `a` is `&str`
- since, str is slice of String, thus a reference by default, it is usually reprensented as `&str`

## Notes:
- `&String` and `&str` are coersed in many scenarios i.e can be changed to one another without explicitly doing so
- `&str` is a reference thus it is not owned, to own is use `.to_string()` method on `&str`
```rs
let a = "Hello"; // a is &str
let b = a.to_string() // b is String and Owned
```

### String and &str
|Expression| Type of a|
|--|--|
| `let a = "red";` | `&str` |
| `let a = "  red ".trim();` | `&str` |
| `let a = String::from("red");` | `String` |
| `let a = &String::from("red")[0..1];` | `&str` |
| `let a = "red".to_string();` | `String` |
| `let a = "RED".to_lowercase();` | `String` |
| `let a = "red".replace("r", "b");` | `String` |
| `let a = "Happy Monday!".to_string().replace("Mon", "Tues");` | `String` |
| `let a = "rust is fun!".to_owned();` | `String` |
| `let a: String = "nice weather".into();` | `String` |
| `let a = format!("Interpolation {}", "Station");` | `String` |

So, to convert `&str` to owned `String`, we can use many options
- .to_string() and .to_owned are simple ones
- .to_string() is a generic method which can convert other types to String
- So, `.to_owned` is the recommended method to convert `&str` to owned `String`

### Creation of String (String Builder)
- Internally, String is a collection of bytes i.e vec of bytes
- So, all Vector methods are available to String as well like `new`, `from` etc.
```rs
let s = String::new();
let mut s = String::from("Hello World");
s.push_str("df") // here "df" is &str
let k = s + "df" // s is moved here, `+` operator uses add method on String, thus it ownes s now.
```

### + operator
- let k = s + "df" from previous example
- `+` operator is shorthand for `add(self, s: &str) {}` method on String
- Usually methods use `&self` as first paramters, but `add` method uses `self` thus taking ownership of `self`.
- Calling s + "df" means `add` will take ownership of self i.e `s`
- since `add` takes self i.e String and a `&str` as parameters, thus `+` operator will merge `String`(left) and `&str`(right)
- So, two Strings or two &str cannot be concated
- Also, `&str` + `String` won\`t work as left variable on `+` operator should be `String`



## &str (string literal / string reference)
```rs
let input = "Hello cars";
let outut = input.replace("cars", "balloons"); // output is String
```
