fn main() {
    /*
      CHAPTER 1:
      EXPRESSIONS --> without a ;
      3
      x+1

      STATEMENTS --> LEFT = variable, RIGHT = expression, ends with ;
      let y = 3;
    */

    // CHAPTER 2: Copy, clone and move
    // Simple scalar values are automagically cloned
    let a = 4;
    let _b = a;
    // Primitive values - u32 (integer), bool, f64, char, tuples ()

    let s1 = String::from("hello");
    let s2 = s1; // value of s1 is moved to s2, s1 is not valid anymore
    let s3 = s2.clone(); // here we created a clone of s2, so both s2 & s3 are valid after this line
    println!("{} {}, world!", s2, s3);

    // 3 concepts
    // 1: Simple scalar values are automagically cloned/copied
    // 2: For complex values, assigning s2 = s1, execute move, s1 not valud
    // 3: use .clone to make copy of s1

    // CHAPTER 3: Passing values to a function

    // s's value moves into the function and so is no longer valid here
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s);
    // println!("{}, world!", s); // -> Error: borrow of moved value: `s`, value borrowed here after move

    let s7 = String::from("hello"); // s comes into scope
    let _s8 = takes_and_gives_back(s7);

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    // Same as earlier, if Simple Scalar Values are passed to a function, they are still valid
    // But if Complex valuse like String are passed, they are not valid anymore.
    // Note: Any value passed to function becomes valid inside the function

    /*
    The ownership of a variable follows the same pattern every time:
    assigning a value to another variable moves it.
    When a variable that includes data on the heap goes out of scope,
    the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

    i.e
    simple scalar values -- as in javascript
    complex values are moved b/w variables or passed to function
    previous ones are cleared/dropped and are not valid anymore, unless ownership moved
    */

    // So, any complex wil be invalid as soon as passed to function, we have to return same value from each function
    // and assign to new variables.
    // UFFF!!!

    // So, Rust has a concept where we can pass a value around without changing ownership
    // Its called Reference and Borrowing

    // CHAPTER 4: Reference and Borrowing

    let s9 = String::from("hello");
    // instead of passing s9 and losing ownership, we have reference to function
    // representated by & sign
    // Action of creating Ownership is called Borrowing
    let _len = calculate_length(&s9);

    // to pass a reference, that can be modified
    // Create variable with mut, and pass reference with &mut
    let mut j1 = String::from("j1 hello");
    push_string_to_end(&mut j1);
    println!("{}", j1);

    // NOTE: you can create only one mutable reference for a particular piece of data
    // which makes sense, because one piece of data cannot have 2 owners of the reference at same time

    {
        let _r1 = &mut j1;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut j1;
    // r2 is created, after r1 is no longer valid, so at a time there is only one owner

    // no problem -> Immutable reference owner is j1
    let _re1 = &j1;

    // no problem -> Immutable reference owner is j1
    // create as many as Imutable references
    let _re2 = &j1;

    // BIG PROBLEM
    // cannot borrow `j1` as mutable because it is also borrowed as immutable, mutable borrow occurs here
    let _re3 = &mut j1;

    // println!("{}, {}, and {}", _re1, _re2, _re3);

    let reference_to_nothing = dangle(); // see dangle function

    /*
    CONCEPTS
    Immutable reference by default, do not change ownership
    `let s = String::from("Hello") and fn send(a: &String)
    For mutable reference
    `let mut s = String::from("Hello") and fn send(a: &mut String)

    - Can create any number of immutable references
    - Cannot create 2 mutable reference at same time
    - Cannot combine immutable and mutable references
    - No dangling references (done automatically)

    Let’s recap what we’ve discussed about references:
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
    */
}

fn dangle() -> String /*&String */ {
    /*
      missing lifetime specifier
    this function's return type contains a borrowed value, but there is no value for it to be borrowed from
      */
    let s = String::from("hello");

    // &s
    s
}

fn push_string_to_end(some_string: &mut String) {
    // push_str modifiees original value
    some_string.push_str(", world")
}

fn calculate_length(s: &String) -> usize {
    // s is reference, so function do not own this string
    // function cant modify reference, as it do not own it
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// JAVASCRIPT
/*
## EXAMPLE 1
const a = 3;
const b = 3;

## EXAMPLE 2
const y = { name:'jassi', age: 32, address: { line1:'aa', line2: 'bb' } }
const z = y; {name:'jassi', age:32, address: y.address }

y.name = 'sushil'
z.name => jassi
y.address = null;
z.address => null;

## EXAMPLE 3
const z = [1,2,3, {a:1,b:2}]
const y = z;
const xx = [...z]; // Independent copy or clone
*/

// ## GO
/*
## EXAMPLE 1
const a = 3; // "Hello", [1,2,3]
const b = 3; // "hello", [1,2,3]

## EXAMPLE 2
const y = { name:'jassi', age: 32, address: { line1:'aa', line2: 'bb' } }
const z = *y; or &y {name:'jassi', age:32, address: y.address }
*/

// ## RUST
/*
## EXAMPLE 1
const a = 3; // "Hello", [1,2,3]
const b = 3; // "hello", [1,2,3]

## EXAMPLE 2
const y = { name:'jassi', age: 32, address: { line1:'aa', line2: 'bb' } }
const z = y; {name:'jassi', age:32, address: y.address }
// Neither copy nor clone i.e no value copy, no reference
// It is called move
*/

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
    // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
    // Here, some_integer goes out of scope. Nothing special happens.
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string
    // a_string is returned and moves out to the calling function
}
