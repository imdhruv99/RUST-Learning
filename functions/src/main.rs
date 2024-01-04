use std::io;

fn get_user_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn sub_numbers(x: i32, y: i32) -> i32 {
    x - y
}

fn mul_numbers(x: i32, y: i32) -> i32 {
    x * y
}

fn div_numbers(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let number_one = get_user_input("Enter First number:");
    let number_two = get_user_input("Enter Second number:");

    let addition_result = add_numbers(number_one, number_two);
    println!("Addition Result: {}", addition_result);

    let subtraction_result = sub_numbers(number_one, number_two);
    println!("Subtraction Result: {}", subtraction_result);

    let multiplication_result = mul_numbers(number_one, number_two);
    println!("Multiplication Result: {}", multiplication_result);

    match div_numbers(number_one, number_two) {
        Ok(division_result) => println!("Division Result: {}", division_result),
        Err(err) => println!("Error: {}", err),
    }
}
