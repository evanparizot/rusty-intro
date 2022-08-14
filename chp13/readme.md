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
