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
