# Vectors

- Vectors in rust are variable length list of same type items, type = Vec<T>
- Unlike, arrays vector length can change
- [Link](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/arrays-vectors-and-slices.html)

## Create an Vector
There are few method to create a vector
1. let a = vec![0; 10]; // create vector of length 10 and fill 0 -> type = Vec<i32>
2. let a = vec![1, 2, 3, 4, 5]; // declare all values, -> type = Vec<i32>
3. let a = Vec::new(); // empty vector of type Vec<unknown>, once we push first value, it will get type
4. let a = vec![]; // same as 3

## Common Methods
- `.len()`: length of vector -> usize

## Slice
- Slice is view into underlying type, thus it is a reference
- Slice of array is of type &[T]
- Slice of any type, (i.e slice of array, slice of vec or slice of String) is of type &[T], because slice is stored as an array either indices of underlying string or values.
- Slice of vector is done using -> &a[start..end]
- If Slice start,end out of bounds rust-server will not complain, but compiler will complain

## Index Access
- simply as a[index], same as other languages

## Iter
- Iterator type for suitable types like Array, Vector, slice and others.
- `.iter()` creates an iterator of type `Iter<`\``_, T>`, which is loop-able with for-loop, 
  - for y in a.iter() {}, y will be of type `&T`