use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret Number: {}", secret_number);

    // keep guessing until and unless it is correct number.
    loop {

        println!("Please input your number.");
        // variables by default are immutable, need to add `mut` if we want to make it mutable
        let mut guess = String::new(); //  `new`` is the function, it returns the empty string, associative function.

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // here match will keep asking us to enter valid number
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num, // if number is valid number, and not string
            Err(_) => continue, // if entered value is anything except number
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Won!".green());
                break;
            }
        }
    }
   
}
