# Modules

- [link](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- Modules in rust are like modules in any other language, modules export functions or variables.


### In Javascript (for reference only)
- every export from a file makes that module `m`, and to access exports from `m`, we import `m` from relative path of file.
- We can combine exports from multiple path and export them again to make a larger module.

## Define: Create Module named garden
In the following snippet we create a simple module called `garden` in some file
```rs
// Define a module garden
mod garden {
	pub fn sum() -> u32 {
		3
	}
}
```

## Declare: Importing or using modules
- Declaring a module
```rs
mod garden; // thats how we declare module
use garden::sum; // thats how we import functions from a module
// or use crate::garden::sum; // thats how we import functions from a module
```

## Steps for searching declared module
Following are the steps to use or import modules inside a rust project
- If we define a module in any file, it can be exported
- If we declare a module in any file, rust will look in this order (module=garden)
  1. Search in current path `pwd`, if defined inline then done, else next
  2. Search in `pwd/garden.rs`, if defined inline then done, else next
  3. Search in `pwd/garden/mod.rs`,if defined inline then done, else error

Note: We can define multiple modules in a file
Note: We can also not define any module in a file, but add `pub` in front of function/variable, these exported variables are part of an implicit module, name of which is fileName itself. It behaves same as other modules.


```rs
// src/a/b/c/garden.rs
pub fn sum() -> u32 {
	3
}
pub const X: u32 = 3;

pub mod house {
	pub fn add() -> u32 {
		5
	}
}
```

```rs
// src/a/b/c/some-name.rs
mod garden; // declared here, 
// 1. searches inline not found
// 2. searches in src/a/b/c/garden.rs found 2 things, sum function and X Values and another mod definition house

let a = garden::sum();
let b = garden::X;
let c = garden::house::add();
println!("{} {} {}", a, b, c);
```

## Linked to entrypoint
We have defined/declared/modules in our project, but eventually they have to link to main function
1. We start at entrypoint i.e `src/lib.rs` or `src/main.rs`
2. Declaring Module in `src/main.rs`  
  - Rust check a file for either module declaration or definiition
  So, rust will search garden module in following order
  1. Search in `src/main.rs`
  2. Search in `src/garden.rs`
  3. Search in `src/garden/mod.rs`
So, not all module in our project will be linked to main, those modules are un-reachable and will be ignored by compiler and language-server.


## mod.rs
- `mod.rs` inside a folder is a special lookup file
- Think of it as `index.js` for CommonJS module system in Javascript
- if `garden.rs` is not sufficient or garden needs to a folder, then `garden/mod.rs` is suitable


### Important note 1
We can use a module named `house` which is declared in 
- our current file (`pwd`)
- or `pwd/house.rs` 
- or `pwd/house/mod.rs`

So, if we want to use some deeply nested module, we cannot directly use that.
- Rust import/use of a module do not depend on path directly, instead we have to declare the module somewhere
- If we have our `deep` module is deeply nested somewhere like `src/a1/a2/a3/deep.rs`
- To use this module `deep`, it has to declared, if we declare it anywhere it may not be searched at `pwd` or `pwd/deep.rs` or `pwd/deep/mod.rs`. So, it has to be declared in `a3` only to be correctly searched.
- Since, we declared `use deep` in `a3` folder, `a3` has to link to main and so-on
- so, in main.rs it will look like 
```rs
mod a1;
fn main() {
  let a = a1::a2::a3::deep::some_func();
  // or crate::a1::a2::a3::deep::some_func();
}
```

### Important note 2
`a1` module can only be declared in main.rs
- After loading/linking/declaring `a1` into main, `a2` -> `a3` -> `deep` are also linked/loaded
- If we declare `mod a1` in `b1` (b1 is at same level as a1), it will search for
  - `src/b1.rs`: not found
  - `src/b1/a1.rs`: not found
  - `src/b1/a1/mod.rs`: not found
  So, it is never found, thus `a1` module can only be declared in main.rs
- In other files, we have to use `crate::a1::a2::a3::deep::some_func()`
- Thus, `a1::a2::a3::deep` represent the path to module
- and `crate::a1::a2::` represent absolute path to module

### Path
Given `a1::a2::a3::deep`, 
- path = `pwd/a1.rs` a1 can be file or folder (if file a2/a3/deep are defined inline)
- path = `pwd/a1/a2.rs` a2 can be file or folder (if file a3/deep are defined inline)
- path = `pwd/a1/a2/a3.rs` a3 can be file or folder (if file deep are defined inline)
- path = `pwd/a1/a2/a3/deep.rs` deep can be file or folder (if file some_func are defined inline)

Given `crate::a1::a2::a3::deep`, same as above, just that pwd=src  
So, module chain do not exactly map to path and it is better that way, because path string cannot be statically analyzed, whereas module chain can be.

## Crate::
- [link](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
- `crate::` is a special keyword which can be used with `use` or inside functions to load already declared modules.
- Using `crate::` is like using absolute path, since we use it use modules declared starting in `main.rs`
- Without using `crate::`, we are using modules as a relative path

## Nesting modules
- module definition can be nested inside each other
```rs
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}
```

## use keyword
- use keyword creates shortcuts to items to reduce repetition of long paths (within a scope)
- use keyword is used for de-structuring / renaming modules (within a scope)

Instead of,
```rs
mod a1;
fn main() {
  let a = a1::a2::a3::deep::some_func();
}
```
we can do
```rs
mod a1;
use a1::a2::a3::deep::some_func;
// or use crate::a1::a2::a3::deep::some_func;
fn main() {
  let a = some_func();
}
```

or we can do
```rs
mod a1;
use a1::a2::a3::deep::{some_func, some_other_func}; // using 2 functions from deep
fn main() {
  let a = some_func();
}
```

or we can do
```rs
mod a1;
use a1::a2::a3::deep; // stop at deep, now we can use any function from deep
fn main() {
  let a = deep::some_func();
}
```

## pub use / re-export
- pub means export the function/variable/module
- using `pub use a1::a2::a3::deep` in `a1` we can shorten the path for export of deep
  - i.e deep is re-exported from a1
- now, anyone using `crate::a1::a2::a3::deep` can use `crate::a1::deep`
- It works similar as combining imports from other module and then exporting it
- Inside `a1`, we can combine any path say `pub use a1::a2::a3::deep` and `pub use a1::a2::b3::sum`
  - i.e deep and sum are re-exported from a1
- Now, deep and sum can be exported as `crate::a1::deep` and `crate::a1::sum`


## self
- self keyword can be used to reference nested modules in a module in a file

## super
- super keyword can be used to reference parent module
- super can be chained like `super::super::` to keep going upwards until main
- super acts as `../` in path
- super::super will acts as `../../`
- super can be useful in test module to refer in same file

## use ::*
- self::*
- super::*
- a1::a2:a3::*

- `*` denotes and export all possible exports, but this might lead to readability hit, like a3::* have sum and minus functions, which can now be used directly but from does sum and minus comes here is less clear.  
- So, better to avoid this
- It can be useful in test modules, because it helps avoid re-writing same functions again

## Inbuild modules / crate
- Inbuilt module do not require declaration like String::from()
- Standard lib modules needs to be exported and can be such at `use std::time`
- Crate installed  (like NPM packages) can be imported same as StandardLib

## How to structure your code into files and modules
- In some sense, always use absolute path to module i.e use `crate::`
- Technically, follow a guideline/pattern to define/declare/use modules in an consistent manner