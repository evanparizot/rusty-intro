## Chapter 5: Using Structs to Structure Related Data

### 5.1 Defining and Instantiating Structs
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
```

#### Using Tuple Structs without Named Fields to Create Different Types
- Can define structs using _tuple structs_
- Useful for when you want to give a tuple a name and be different from other tuples, but a full blown struct would be excessive

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```

#### Unit-Like Structs Without Any Fields
- Can define _unit-like structs_ that behave similar to the `()` unit type

### 5.2 An Example Program Using Structs

#### Adding Useful Functionality with Derived Traits
- Can add trait annotations for something like display purposes
- Need to add a `#[derive(Debug)]` annotation

### 5.3 Method Syntax
- _Methods_ are similar to functions, but are instead defined within a struct

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

```
- Need to use `&self` due to that methods can take ownership of `self`, borrow `self` immutably (this case) or borrow `self` mutably

#### Methods with More Parameters