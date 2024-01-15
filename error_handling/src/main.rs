use std::fs::{self, File};
use std::io::{ErrorKind, self, Read}; // let us match type of error

fn a() {
    b();
}

fn b() {
    c(70);
}

fn c(num: i32) {
    if num == 7 {
        // panic will throw the error for code
        panic!("૭ માં આસમાને ગુસ્સો છે ");
    }
    println!("હું તો ટાઢો છું.");
}

fn main() {
    // uncomment below function to understand panic
    // a();
    // setting up RUST_BACKTRACE=1 will lead us to error prone code line


    // Recoverable Errors

    // similar to option enum, Ok or Error
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");
    // using match operation to handle error
    // handling error gracefully
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error while creating the file: {:?}", e),
            },
            other_error => {
                panic!("Error while opening the file: {:?}", other_error)
            }
        }
    };

    let _read_username_from_file = read_username_from_file();
}

// Error Propagation example, returns Result type
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    //  ? will work as if condition, if below code fails, it will return the error
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // below line will run as above 3 line
    fs::read_to_string("hello.txt")
}

// understood when to use panic vs Result
// Mostly use Result type until and unless you can't avoid error
// Alternate approach would be the custom type to handle the error