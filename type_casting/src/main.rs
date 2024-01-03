use std::io;

fn main() {
    let x: u8 = 5; // range for u8 = 0 to 255
    let y: i8 = 10; // range for i8 = -128 to -127
                    // let z = 25i8; <<- another way to define variables
                    // let z = 25_i8 <<- another way to define variables
                    // let z = 25 as i8 <<- another way to define variables, type casting

    // we can not perform addition on different types like `z = x + y` will through error
    // so we need to implement type conversion
    let z = x + (y as u8); // type casting
    println!("z is {}", z);

    let a: f32 = 22.0;
    let b: f32 = 7.0;
    // let c = a / b;
    let c = (a as f64) / (b as f64);
    println!("c is {}", c);

    // overflow condition that runs
    let d = (i32::MAX as i64) + 1; // i32's max value + 1
    let e = 10_i32;
    let f = d as i32 / e; // cz we are converting it into i32 from i64, it will overflow and give us negative number
    println!("f is {}", f);

    // string type conversion example
    let mut input = String::new();
    println!("Please enter new string: ");
    io::stdin().read_line(&mut input).expect("read line function did not work");
    println!("input before type conversion is {}", input);

    // parse will convert string to integer,
    // unwrap is like expect, but it will take valid data type (in our case i64) and unwrap it in actual integer and return that
    // so if you insert `String` it will give us the runtime error
    println!("input after type conversion:");
    let int_input : i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 5);
}
