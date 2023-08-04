# Arrays 

- Array in rust are fixed length list of same type items, type = [T]
- [Link](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/arrays-vectors-and-slices.html)
- Cannot add to an array, as they are fixed length

## Create an array
There are few method to create an array
1. let a = [0; 10]; // create array of length 10 and fill 0 -> type = [i32;10]
2. let a = [1, 2, 3, 4, 5]; // declare all values, -> type = [i32;5]

## Common Methods
- `.len()`: length of array -> usize

## Slice
- Slice is view into underlying type, thus it is a reference
- Slice of array is of type &[T]
- Slice of any type, (i.e slice of array, slice of vec or slice of String) is of type &[T], because slice is stored as an array either indices of underlying string or values.
- Slice of array is done using -> &a[start..end]
- If Slice start,end out of bounds rust-server and compiler will complain


## Index Access
- simply as a[index], same as other languages

## Iter
- Iterator type for suitable types like Array, Vector, slice and others.
- `.iter()` creates an iterator of type `Iter<`\``_, T>`, which is loop-able with for-loop, 
  - for y in a.iter() {}, y will be of type `&T`