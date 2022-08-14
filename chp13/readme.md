https://doc.rust-lang.org/book/ch13-00-functional-features.html

# Functional Language Features: Iterators and Closures
## 13.1 Closures: Anonymous Functions that Capture their environment

### Closure Type Inference and Annotation
- Closures don't typically require type annoatations
- Typically short and have a narrow context of relevance
- We can, however, annotate with types if we want to:

```rust
let verbose = |num: u32| -> u32 {
    println!("Doing something");
    thread::sleep(Duration::from_secs(2));
    num
}
```

- Since we do not need to be explicit with types for a closure, the compiler 
    will throw an error if we attempt to use a closure with multiple types

```rust
let closure = |x| x;
let s = closure(String::from("hello"));
let n = closure(5); //error raised here
```


### Capturing References or Moving Ownership
- Closures can capture values in 3 different ways. Which way that gets used is determined by the compiler within the closure body
1. Borrow immutably
2. Borrow mutably
3. Take ownership

```rust
// Borrow Immutably
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// Borrow Mutably
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// Take ownership
// use the move keyword
```

### Moving captured values out of closures and the Fn traits
- Once a closure has captured a reference or captured ownership, the body of the
closure determines what happens to those references or values
- A closure body can do the following:
1. Move a captured value out of the closure
2. Mutate the captured value
3. Neither move nor mutate
4. Capture nothing from the environment in the first place

- Traits determine how a closure is handling values. Closures automatically implement one to three `Fn` traits:
1. `FnOnce` applies to closures that can be called at least once. All closures implement this trait. Closures that
moves captured values out of its body will only implement `FnOnce` 
2. `FnMut` applies to closures that don't move captured values out of their body. Can be called more than once.
3. `Fn` applies to closures that don't move captured values out of their body and don't mutate captured values, 
as well as closures that capture nothing from their environment


## 13.2 Processing a series of items with iterators
- Iterators in rust are lazy, meaning that methods need to consume an iterator for it to do something

```rust
let v1 = vec![1, 2, 3];
let v1_iterator = v1.iter();
for v in v1_iterator {
    println!("Got: {}", v);
}
```

### The iterator trait and the next method
- All iterators implement the `Iterator` trait
- Calling `next()` on an iterator uses it up, requiring mutability or ownership

### Methods that Consume the Iterator
- Methods that call `next` are _consuming adaptors_ (calling them uses up the iterator)

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
```

### Methods that Produce other Iterators
- _Iterator Adaptors_ are methods defined on the `Iterator` trait that don't consume the iterator; they produce different ierators by changing aspects of the original iterator

```rust
let v1: Vec<i32> = vec![1,2,3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2,3,4]);
```

### Using closures that Capture their environment
- Many iterator adapters take closures as arguments
- The `filter` method is one such example

```rust
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

## 13.3 Improving our I/O Project