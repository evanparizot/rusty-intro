## 7.1

- A _crate_ is a binary or library
- A _package_ is one or more crates that provide a set of functionalities
    - Must contain zero or one library crates (no more)
    
- Can create packages with `cargo new`

## 7.2/7.3 - Defining modules

- Define modules with the `mod` keyword
- Can use relative or absolute paths for calling modules
- All items in modules (funcs, methods, structs, enums, modules) are private by default
- Can make items public using the `pub` keyword
