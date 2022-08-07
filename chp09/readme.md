# Chapter 9 : Error Handling

## 9.1 Unrecoverable Errors with panic!
- the `panic!` macro causes your program to exit

## 9.2 Recoverable Errors with Result
- Can handle issues within a program with the `Result` enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

This means we need to handle results appropriately:

```rust
fn main() {
    let f = File::open("Hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening file: {:?}", e),
    }
}
```

### Matching on different errors

```rust
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating a file: {:?}", e),
            },
            other_error => {
                panic!("Other error was thrown: {:?}", other_error)
            },
        }
    }
}
```

```rust
// This is semantically the same as the code block above
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: ", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### Shortcuts for panic on Error (unwrap and expect)

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap()
}
```

### Propogating Errors

```rust
fn read() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

```rust
// This is the same as the above
fn read() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- `?` after a Result functions similarly to a match statement
- If the result is Ok, then the program continues as normal
- If the result is Err, then the function exits with the Error
- `?` will cause errors to go through the from clause, being converted to the Error listed in the return definition

```rust
fn read() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

```rust
fn read() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

- `?` can only be used in functions whose return type is compatible with the value `?` is used on
- This basically means that functions using `?` need to return a Result, Option, or type implementing `FromResidual`

```rust
fn last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- You cannot mix and match using `?` on options and results in a function


## 9.3 To panic! or Not to panic!