use add_one;

fn main() {
    println!("Hello, world!");
    let num = 10;
    println!("Hello, world! {num} plus one is {}", add_one::add_one(num));
}
