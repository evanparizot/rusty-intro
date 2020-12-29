use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // `rand::thread_rng()`
    // gives particular numb gen; local to the current thread of execution and seeded by OS

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        
        // Variables are immutable by default
        // Need to have mut keyword to make mutable
    
        // ::new
        // associated function (static method)

        io::stdin()
            .read_line(&mut guess) // & -> reference (immutable by default (see chp 4))
            .expect("Failed to read line");
        
        let guess = match guess.trim().parse::<u32>() {
        // ::<> is called a turbofish?
        //let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => {
                println!("Just Right");
                break;
            }
        }
    }
}