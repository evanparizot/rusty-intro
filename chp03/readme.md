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
- All elements in an array must be of same type
- Fixed length

```rust
let a = [1, 2, 3, 4, 5];
```

- Useful for when wanting data allocated on the stack rather than the heap
- Useful for ensuring fixed number of elements (alternative would be a vector (array list or dynamic array or list))

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// i32 is type of each element
// ;5 indicates array contains 5 elements

let b = [3; 5];
let b = [3, 3, 3, 3, 3];
// These are the same

println!("First element in array: {}", b[0]);

```
- Rust will _panic_ when attempting to access an invalid index


### 3.3 Functions
- Functions are defined with `snake_casing` by convention
- Start with the keyword `fn`

```rust
fn main() {
    println!("Hello world");
    another_function();
}

fn another_function() {
    println!("Hello world, again");
}
```

#### Function Parameters
```rust
fn main() {
    function(4);
}

fn function(x: i32) {
    println!("The value of x is : {}", x);
}
```

#### Function Bodies Contain Statements and Expressions
- Creating a variable and assigning a value to it with the `let` keyword is a **statement**
```rust
fn main() {
    let y = 5;
}
```
- Function definitions are also statements. **Statements** do not return values; can't assign a `let` statement to another variable
- **Expressions** evaluate to something. 
    - Can be part of statements
    - Expressions do not include ending semicolons (adding one turns it into a statement)

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // expression
    };

    println!("The value of y is: {}", y); // 4
}
```

#### Functions with Return Values
- Most functions return the last expression implicitly
```rust
fn five() -> i32 {
    5 // this is an expression (returns something)
}

fn invalidfive() -> i32 {
    5; // this is a statement (does not return something)
}

fn altfive() -> i32 {
    return 5;
}

fn main() {
    let x = five();

    println!("The vale of x is : {}", x);
}
```

### 3.4 Comments
- Use double slashes
- No block comments

### 3.5 Control Flow
#### if Expressions
- `if`'s are expressions

```rust
fn main() {
    let num = 3;

    if num < 5 {
        println!("Was less that 5");
    } else {
        println!("Was greater than 5 or was 5");
    }
}
```
- With ifs, conditionals must evalutate to a bool. It won't convert non-boolean types

#### Handling Multiple Conditions with else if
```rust
fn main() {
    let num = 5;
    if num % 4 == 0 {
        println!("Divisible by 4");
    } else if num % 3 == 0 {
        println!("Divisible by 3");
    } else if num % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("What are you doing?");
    }
}
```

#### Using if in a let Statement
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6] };

    println!("The value of number is : {}", number); // 5
}
```

#### Repetition with Loops
##### Repeating code with loop
- Can terminate program with either `break` keyword or external interrupt
```rust
fn main() {
    loop {
        println!("Infinite loop");
    }
}
```
##### Returning values from loops
```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
}
```

##### Conditional Loops with while
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("done");
}
```

##### Looping through a collection with for
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for e in a.iter() {
        println!("Currently on {}", e);
    }
}

fn alt() {
    for n in (1..4).rev() {
        println!("{}", n);
    }
    println!("Done");
}

```
