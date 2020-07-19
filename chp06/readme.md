## Chapter 6: Enums and Pattern Matching

### 6.1 Defining an Enum
- Can define methods on enums just like structs
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

#### The Option Enum and It's Advantages over Null Values
- The _Option_ encodes the common scenario in which a value could be something or it could be nothing
- Rust **does not** have _null_ but has an enum that can encode the concept of a value being present or absent (`Option<T>`)

- By having the `Option<T>` type, you have to explicitly convert to the `<T>` before operating
- In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`
- Can use methods provided by the [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html) enum to pull values out of `Option<T>`

### 6.2 The Match Control Flow Operator
```rust
enum Coin {
    Penny, Nickel, Dime, Quarter
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10, 
        Coin::Quarter => 25
    }
}
```

- Can use `_` as a wildcard catch all within match blocks
```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

### 6.3 Concise Control Flow with if let
Instead of having the following:

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

can use the `if let` syntax to reduce boilerplate: 
```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```




