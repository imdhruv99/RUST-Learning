use std::fs::File;
use std::io::ErrorKind; // let us match type of error

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
}
