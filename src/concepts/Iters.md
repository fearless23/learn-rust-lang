# Iter

TODO:
- Iter, map and collect
- iter_mut

## Iter, map and collect
- All three are different methods and have their own independent existence, but usually used in this manner
- Lets take `a` which can be array/vec/slice/string?
- leet b = a.iter().map(func).collect();
- iter creates iterator which can be used in for-loop
- iter always gives &T reference to item of a, for mutating use iter_mut
- map is similar to Javascript map allows to map `a` using arrow function
- collect is a powerful methods which convert the mapped to final data structure