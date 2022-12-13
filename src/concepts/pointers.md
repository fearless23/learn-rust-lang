# Pointers in Rust

### Pointers while assignment

- `a` is the original value and type.
- `x` is the what a is assigned to.

#### 1. Primitive Types Assignment

Types on which this applies:

- int
- float
- &str

For any primitive type, `x = a` assignment creates a new copy of a and assigns it to x.

##### Mutability:

Mutability of each `x` and `a` is represented by

`let mut a = 3;` a is mutable or

`let a = 3;` a is immutable

`let x = a;` x is immutable; mutability of a is whatever is chosen

`let mut x = a;` x is mutable; mutability of a is whatever is chosen

Value of `x` is whatever value of a is at the time of assignment.

### 2. Non Primitive Types Assignment

- Strings
- vectors
- Other non-primitive types.

For non primitive types assignment `x = a` assignment. Value moves from a to x or in rust, its called x borrowed value from a.

- Now, x is the owner of value. `Only one owner at a time`
- What happened to a?

  - It no longer has value or ownership. It`s dead.

- Option 1: Create a pointer to a.
  - Do `x = &a`; this creates x as pointer of a.
  - `a` is still the owner of the value.
  - Print pointer with `{:p}`
  - Get value with `*x`; also called dereference.
  - Using `{}` or `{:?}` in print, will print value stored
    at pointer i.e original value a.
- Option 2: Create copy of a, before assigning to x.
  - Each types has its way to create copy for non-primitive types.

#### Borrowing

Above when we create pointers, there are 2 type of pointers. Read only, Writeable pointer.

- `let x = &a` --> x is read-only pointer.
  - \*x can read value of a, cant change it.
  - a itself can be mut or immut
- `let x = &mut a` --> x is writeable pointer.

  - \*x can write values to memory where a is, given a itself is mutable.

#### Mutability

- `let a = vec![1,2,3];` implies a is im-mutable.

  - `let x = &a;` will make `*x` im-mutable
  - Above operation is called immutable borrow.
  - `let x = &mut a;` will throw error as a is not mutable.
  - `let mut x = a;` will throw error; as x is only a pointer.

- `let mut a = vec![1,2,3];` implies a is mutable.
  - `let x = &a` will borrow a as im-mutable, but ownership is of a. Now, a,x both cannot be changed.

### Important Note 1:

Mutable borrow is possible from `mut a`; and immutable borrow is possible from not mut `a`.

- `let x = &a;` is a im-mutable borrow
- `let x = &mut a;` is mutable borrow
- However `let mut x` means we can change `x` to point to another vec of same type and
- `let x` means `x` itself is immutable.

### Important Note 2:

After new vector is pointed; all operations must occur on new. Because if we assign to original `a` even mutable a i.e `mut a`; after assignment we get `cannot assign to a because it is borrowed` error. To mutate value at `a`, use `*x`.

| `a`    | Borrowing | `x`   | Example                                              | Operations                                                             |
| ------ | --------- | ----- | ---------------------------------------------------- | ---------------------------------------------------------------------- |
| mut    | mut       | mut   | `let mut a = vec![1,2,3];`</br>`let mut x = &mut a;` | mut                                                                    |
| mut    | mut       | immut | `let mut a = vec![1,2,3];`</br>`let x = &mut a;`     | immut                                                                  |
| mut    | immut     | mut   | `let mut a = vec![1,2,3];`</br>`let mut x = &a;`     | mut                                                                    |
| mut    | immut     | immut | `let mut a = vec![1,2,3];`</br> `let x = &a;`        | immut                                                                  |
| im-mut | immut     | mut   | `let a = vec![1,2,3];`</br>`let mut x = &a;`         | x can be &b i,e point to other similar vectors.                        |
| im-mut | immut     | immut | `let a = vec![1,2,3];`</br> `let x = &a;`            | x can only point to a.                                                 |
| im-mut | mut       | mut   | `let a = vec![1,2,3];`</br> `let mut x = &mut a;`    | Error - cannot borrow `a` as mutable, as it is not declared as mutable |
| im-mut | mut       | immut | `let a = vec![1,2,3];`</br> `let x = &mut a;`        | Error - cannot borrow `a` as mutable, as it is not declared as mutable |
