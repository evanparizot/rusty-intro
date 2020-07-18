## Chapter 4: Ownership
- Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time


### 4.1 What is Ownership?
#### The Stack and the Heap
- All items stored on the stack must have a known, fixed size. Otherwise the heap must be used
- When _allocating_ on the heap, a pointer is generated to the reserved location and said pointer is kept on the stack
- Pushing onto the stack is always going to be faster. Accessing data on the stack is going to be faster as well (less hoops to just through)

#### Ownership Rules
1. Each value in Rust has a variable that's called it's owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

#### Variable Scope
> A scope is the range within a program for which an item is valid
```rust
{   // s not yet valid
    let s = "hello"; // s valid past this point
    // s still valid
}   // s not valid after this point
```

#### The string type
- Other than string literals (which are immutable), there exists a second type `String`
- This type is allocated on the heap
```rust
let s = String::from("hello");  // s => hello
s.push_str(", world");          // s => hello, world
```

#### Memory and Allocation
- In order to support mutable, growable data structures, need to allocate memory on the heap
1. The memory must be requested from the memory allocator at runtime
2. We need a way of returning this memory to the allocator when we're done with our `String`

- When a variable goes out of scope, Rust calls a special function `drop` where code can be put that returns memory

#### Ways Variables and Data Interact: Move
```rust
let x = 5;
let y = x;
```
- `x` is initialized and pushed onto the stack
- `y` is a copy of `x` and it's value is also pushed onto the stack

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
- `s1` has a pointer created pointing to the location in the heap the string is stored
- When `s2` is created, instead of having another pointer pointing to the same heap location get created, `s1`'s pointer is actually moved to `s2` and `s1` is invalidated
- This means that only `s2` needs to be freeded later
- Rust will never automatically create a deep copy of data

#### Ways Variables and Data Interact: Clone
- Can use `clone` to deep copy heap and stack data
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
- The call to clone may be expensive, but it's an explicit indicator that something is happening

#### Stack-Only Data: Copy
- Rust has a special annotation `Copy` that can be placed on types like integers that are stored on the stack
- Rust will not allow a `Copy` annotation on a type if it or it's parts implement the `Drop` trait
`Copy` Types:
- All interger types (`u32`, `i64`)
- The Boolean type (`bool`)
- All the floating point types (`f64`)
- The character type (`char`)
- Tuples if they contain types that are also `Copy` (eg. (`i32`, `i32`) not (`i32`, `String`))

#### Ownership and Functions
- Passing a variable to a function will move or copy, just as assignment does
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

#### Return Values and Scope
- Returning values can also transfer ownership. Assigning a value to another variable moves it.
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable

### 4.2 References and Borrowing
An example of using a _reference_ as a parameter instead of taking ownership:
```rust
fn function1() {
    let s1 = String::from("hello");
    let len = do_something(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn do_something(s: &String) -> usize {
    s.len()
}
```

- When having references as function parameters it's called _borrowing_
- Attempting to modify a reference is not allowed; they're immutable by default

#### Mutable References
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

- Can only have one mutable reference to a particular piece of data in a particular scope. This prevents data races
Data Races occur when:
1. Two or more pointers access the same data at the same time
2. At least one of the pointers is being used to write to the data
3. There's no mechanism being used to synchronize access to the data

Example of an issue:
```rust
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

Example of something that is ok:
```rust
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

#### Dangling References
- The Rust compiler guarantees that references will never be dangling references; if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does

```rust
fn main() {
    let nothing = dangle();
}

fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // return reference to string, BUT
}       // s goes out of scope here and is dropped. We have an issue with the return
// alternative would be to just return s directly
```

### 4.3 The Slice Type
- Allow the access of contiguous sequences in a collection rather than the whole collection

#### String Slices
- Rather than a reference to the entire string, can slice to take a reference to a portion of the string
```rust
let world = &s[6..11]; // world is a slice that contains a pointer to the 7th byte of s with a length value of 5

let s = String::from("hello");

// These are the same
let slice = &s[0..2];
let slice = &s[..2];

// These are the same
let len = s.len();
let slice = &s[3..len];
let slice = &s[3..];

// These are the same
let slice = &s[0..len];
let slice = &s[..];
```

#### String Literals Are Slices
```rust
let s = "hello, world";
// type of s is &str (immutable)
```

##### String Slices as Parameters
- Can make function parameters more generic for string types:
```rust

fn first_word(s: &str) -> &str {}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

##### Other Slices
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```