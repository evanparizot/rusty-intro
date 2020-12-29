## 8.1 Storing lists of values with vectors
- `Vec<T>` allow you to store more than one value in a single data structure
- Can only store values of the same type

```rust
let v: Vec<i32> = Vec::new();
```

- In above case, need to specify what type the vector will hold
- Can use the `vec!` macro to do this inline

```rust
let v = vec![1 ,2 ,3];

let third: &i32 = &v[2];
```

- Can either use direct accessing (& with []) or using optionals with `.get()`

```rust
let v = vec![100,32,20];
for i in &v {
    println!("{}", i);
}

let mut x = vec![100,32,20];
for i in &mut x {
    *i += 50;
}
```

## 8.2 Strings

## 8.3 Storing keys with associated values in hash maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
```