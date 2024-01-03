use std::io; // user input

fn main() {
    let mut input = String::new();

    // read_line function expect a string as standard input, to pass other than string, we need to do type casting
    println!("Enter the value: ");
    io::stdin()
        .read_line(&mut input) // mutable reference to the input variable
        .expect("Failed to read line");

    println!("Input: {}", input);
}
