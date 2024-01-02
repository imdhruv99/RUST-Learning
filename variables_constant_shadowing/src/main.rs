fn main() {
    // immutable variable
    let x: u32 = 10;
    println!("x is : {}", x);

    // mutable variable
    let mut y: u32 = 105;
    println!("y is : {}", y);
    y = 110;
    println!("y is after changing the value: {}", y);

    // Name Shadowing,
    // x is 2 in this scope only, outside scope will change.
    {
        let x = 2;
        println!("x is : {}", x);
    }

    // but we can use variable of exterior scope into interior scope.
    {
        let x = x - 2;
        println!("x is : {}", x);
    }

    let x = 15;
    println!("x is : {}", x);

    // we can change the type of variable when redefining it, Now RUST will consider x as new variable
    // by using let we are redefining the variable
    let x = "Dhruv Prajapati";
    println!("x is : {}", x);

    // Below block will through error because we are not redefining the variable yet changing it's type
    // Error: expected `u32`, found `&str`
    /*
    y = "Dhruv";
    println!("y is : {}", y);
    */

    // use `const` keyword to create constant
    // we can cont redefine or reassign the constant.
    const PI: f64 = 3.14;
    println!("PI's value is: {}", PI);
}
