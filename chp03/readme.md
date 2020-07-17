## Chapter 3 Notes

### 3.1 Variables and Mutability
- Rust variables by default are immutable

#### Differences between variables and constants
- Not allowed to use `mut` with constants (constants are always immutable, will never change)
- Declare constants with the `const` keyword (must annotate type of value)
- Constants may be set to only a constant expression (not the result of a function call or runtime computed value)
- Constant declaration example:

```rust
const MAX_POINTS: u32 = 100_000;
```

#### Shadowing
> Declaring a new variable with the same name as a previous variable

```rust
fn main() {
    let x = 5;
    // x => 5
    let x = x + 1;
    // x => 6
    let x = x * 2;
    // x => 12
}
```
- Since we are creating a new variable when using `let` again, can also change the type but reuse the name
```rust
let spaces = "   "; // str
let spaces = spaces.len(); // int
```



### 3.2 Data Types
- Rust is a statically typed language

#### Scalar Types
- Represents a single value. Four primary types: `integer`, `floating-point`, `boolean`, `characters`

##### Integer Types
| Length  | Signed  | Unsigned  |
|:-:|---|---|
| 8-bit  | `i8`  | `u8`  |
| 16-bit  | i16  | `u16` |
| 32-bit  | i64  | `u32` |
| 128-bit  | i128  | `u128` |
| arch  | isize  | `usize` |

- `Unsigned`: Always positive
- `Signed`: Positive or Negative
- `arch` depends on OS architecture (64-bit vs 32-bit)

| Number Literals  | Example  |
|:-:|---|
| Decimal  | `98_222`  |   
| Hex  | `0xff`  |   
| Octal  | `0o77`  |   
| Binary  | `0b1111_0000`  |   
| Byte (`u8` only)  | `b'A'`  |

- When compiling in debug, Rust will check for **Integer Overflows**. Rust does not check when compiling in release mode. Need to be aware that _two's compliment wrapping_ will happen during runtime to avoid overflow errors.


##### Floating-Point Types
- `f32`
- `f64` (default)

```rust
fn main() {
    let x = 2.0;      // f64
    let y: f32 = 3.0; // f32
}
```

##### Numeric Operations
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

##### Boolean Type
```rust
fn main() {
    let t = true;
    let f: bool: false;
}
```

##### Character Type
- Specified with a single quote
- Unicode Scalar Value characters (`U+0000` to `U+E000` & `U+E000` to `U+10FFFF`)

```rust
fn main() {
    let c = 'z';
    let z = 'Z';

}
```

#### Compound Types
> Group multiple values into one type

##### Tuple Type
- General way of grouping together a number of values with a variety of types into one compound type
- Fixed length

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 5.2, 1);

    println!("First item in tuple: {}", tup.0);

    // Tuple Destructuring
    let (x, y, z) = tup;
}
```

##### Array Type
