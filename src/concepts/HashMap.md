# HashMap

- Being Hashmap from `use std::collections::HashMap;`

## Creating HashMap
- `HashMap::new()`: no type, type will be set when first key-value pair is added
- `HashMap::<K, V>new()`: type of key is K, type of value is V
- HashMap type is `HashMap<K,V>`

## HashMap methods
- insert: insert or updates Key and Value, needs HashMap to be mutable
- len: length of HashMap
- values: iter of values
- values().collect(): collect the iter of values into some vector
- iter of values can be methods as any iter
- keys: iter of keys
- contains_key: checks if a certain key exists

## Accessing property
- hashmap.get(k: &K) tries to get value of key `k` and returns `Option<&V>`