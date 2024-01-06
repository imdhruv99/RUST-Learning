fn main() {

    // Loop
    let mut counter: i32 = 0;
    loop {
        println!("Normal Loop Example");
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let result: i32 = loop {
        println!("Loop example that returns the counter");
        counter += 1;
        if counter == 20 {
            break counter;
        }
    };
    println!("The result is {}", result);

    // While Loop
    let mut number: i32 = 10;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // For Loop
    let a: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    // For loop with Range
    for number in 1..10 {
        println!("The value is {}", number);
    }
}
