fn main() {
    // conditions
    // `<`, `>`, `<=`, `>=`, `!=`, `==`
    // We can't compare two different datatype
    // let condition = 2 <= 2.2
    // Above line will throw us error
    let condition_1 = (2 as f32) <= 2.2;
    println!("{}", condition_1);

    // compound condition
    // Logical Operators `&&`, `||`, `!`
    let condition_2 = true && condition_1;
    println!("{}", condition_2);

    // control flow
    let number: u32 = 88;
    if number == 56 {
        println!("This is true, Number is 56");
    } else if number == 88 {
        println!("This is true, Number is 88")
    }else {
        println!("This is not true");
    }
}
