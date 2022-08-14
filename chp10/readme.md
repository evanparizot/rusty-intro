# Generic Types, Traits, and Lifetimes
## 10.1 Generic Data Types

Can use a type parameter (eg `T`) to parametize function signature

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

### In struct definitions
- Can define structs to take a generic type parameter for one or more fields

```rust
struct Point<T> {
    x: T,
    y: T
}

struct Point2<T, U> {
    x: T, y: U
}
```

### In enum definitions

```rust
enum Option<T> {
    Some(T), 
    None,
}
```

### In method definitions

```rust
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};
}

```

- Methods within a `impl` that declares a generic type will be defined on any instance of the type
- Can also specify on a specific type

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- Generic type parameters in a struct definition don't need to be the same as the struct's method signatures
```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}
```

## 10.2 Trails: Defining Shared Behavior
- A _trait_ defines functionality a particular type has and can share with other types. 
- Traits can be used to define shared behavior in an abstract way
- Similar to interfaces in other languages

### Defining a trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing a trait on a type

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.lcoation)
    }
}
```

- Cannot implement external traits on external types

### Default implementations
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

impl Summary for NewsArticle {} // Means we use the default implementation
```

- Default implementations can call other methods in the same trait

```rust
pub trait Summary {
    fn summarize_author(&self) -> String {}

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}
```

### Traits as parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is the same as directly above
pub fn notify<T: Summary>(item: &T) {

}
```

We can also specify multiple trait bounds:  

```rust
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}
```

There is an alternate syntax when using many trait bounds

```rust
fn something<T, U>(t: &T, u: &U) -> i32
    where T:Display + Clone, U: Clone + Debug {}
```

### Returning types that implement traits

- Can return a value of some type specifying a trait
- Can only use this if returning a single type
```rust
fn returns_summary() -> impl Summary {
    Tweet {

    }
}
```

## 10.3 Validating references with lifetimes

- Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
- They start with a ' (eg `'a`)
```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}
```

- When annotating lifetimes in functions, annotations go in the function signature, not in the function body
- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters (otherwise it needs to be created new in the function and ownership given to the calling function (no creating and returning a reference))


### Lifetime annotations in struct definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Lifetime elision
- **Lifetime Elision Rules** - Rules that the compiler evaluates your code to for lifetime annotations. If your code fits the rules, then lifetime annotations don't need to be specified

Putting it all together:

```rust
use std::fmt::Display;
fn longest_with_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T) -> &'a str where T:Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x 
    } else {y}
}
```