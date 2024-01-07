fn main() {
    let s = String::from("Dhruv");
    ownership(s);

    let x: i32 = 23;
    makes_copy(x);
    println!("x's value is: {}", x)
}

fn ownership(demo_string: String) {
    println!("{}", demo_string);
}

fn makes_copy(y: i32) {
    println!("y's value is: {}", y);
}
