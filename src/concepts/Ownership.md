# Ownership

- [Link](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/ownership.html)

The ownership system has a few distinct concepts: 
- ownership,
- borrowing, 
- and lifetimes.

Rust do not have a garbage collector, so to manage resources like memory is uses concepts of ownership. 
- Any variable can have maximum of 1 owner at a given time.
- When a variable has no owner it is automatically cleaned/removed.

## Passing to function
- When an variable(argument/value) `v` is passed to a `function` as argument, it gets ownership of that value thus original value can`t be used anymore. We call this "moving" a variable.

Rust provides a couple of different ways to mitigate this issue:
1. Make another copy/clone of `v` and pass that to `function` instead.
2. Make `function` borrow its argument instead of taking ownership of it, and then make a copy/clone in order to return an new value.
3. Or, you could make `function` *mutably* borrow a reference to its argument (which will need to be
   mutable), modify it directly, then not return anything. This means that `v` will be modified in-place.


In above, passing to function section, we see there is one owner at a time
1. new copy/clone of `v` is owned by `function` now, as we pass it to function
2. We pass & of `v` to function, thus we pass reference of `v`, this is called Borrowing. The owner of `v` is still outside the function. Note: Here, function borrows `v`, but cannot modify it since it is not mutable.
3. We pass mutable reference of `v` to function i.e &mut`v` instead of &`v`, thus function now borrows the mutable reference and can modify the `v` directly now.

4. When passing variable to a function, we can pass it as mut like so
```rs
let a = vec![1,2,3];
fn run(mut x:Vec<i32>){
  //
}
run(a);
```
- Here `a` is passed to `run` function, thus `run` function now owns `a`
- Value of `a` is moved to `x` (argument)
- since, x is declared with `mut`, `x` is now mutable version of `a`


lets, summarize these 4 methods so far
```rs
// #4
let a = vec![1,2,3];
fn run(mut x:Vec<i32>){} 
run(a) // a is owned by run function now and can`t be used anymore

// #1
let a = vec![1,2,3];
let new_a = a.clone();
fn run(x:Vec<i32>){}
run(new_a)
// #2
let a = vec![1,2,3];
fn run(x: &Vec<i32>){
  let y = x.clone(); // newly owned vec which can be modified
  y.push(4);
  y
}
let b = run(&a); // a is not owned by run function now and can be used any-where
// #3
let mut a = vec![1,2,3]; // will  be modified in-place
fn run(x: &mut Vec<i32>){
  // modify x in-place
}
run(&mut a) // a is not owned by run function now and can be used any-where, but modified in-place
a.push(3)

```

## Reference
- Using `&` before a variable `x`, we can use it as a reference(aka pointer) to `x`.
- By default, pointer allows reading the `x` value but not modify it, even if `x` is mutable.
- To make changes to the `x` value use `&mut` before variable, which gives you an mutable reference.
- Note: for `&mut` to work the original variable should be mutable as well
- The ownership of value is now with new variable using `&mut`
- Changing values
```rs
let mut x = 100;
let y = &x; // Reference
*y += 100; // ERROR
let y = &mut x; // Mutuable reference
*y += 100; // WORKS
```