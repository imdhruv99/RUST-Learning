fn main() {
    // // for more details: https://doc.rust-lang.org/book/ch03-02-data-types.html

    // Primitive DataTypes

    // Scaler DataTypes

    // Signed integer --> supported i8, i16, i32(Default), i64, i128, isize (based on arch size)
    // one bit is used for size, so can store length up to 2^n - 1 - 1. so i8 == -(2^7) to (2^7) == -128 to 127, 1 less for sign

    let a: i32 = -209897;
    println!("a is: {}", a);

    // Unsigned integer --> supported i8, i16, i32(Default), i64, i128, isize (based on arch size)
    // same as signed except we can not use negative value, so can store length up to 2^(n) - 1. so u8 == (2^8) - 1 == 0 to 255
    let b: u32 = 12345;
    println!("b is: {}", b);

    // Float --> supported f32, f64(Default)
    let c: f32 = 12.34;
    println!("c is: {}", c);

    // Boolean --> supported true, false
    let d = true;
    println!("d is: {}", d);
    let e: bool = false;
    println!("e is: {}", e);

    // Character --> Use Single inverted comma ` 'a' `, contain single char
    let f = 'A';
    println!("f is: {}", f);
    let g: char = 'B';
    println!("g is: {}", g);

    // Compound DataTypes

    // Tuple
    // Fixed-length heterogeneous items.
    // Mutable only with mut keyword, but cannot change type
    // Indexing via . syntax e.g myTuple.0 for first element
    let h: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = h.0;
    let six_point_four = h.1;
    let one = h.2;
    println!("h.0 is: {}", five_hundred);
    println!("h.1 is: {}", six_point_four);
    println!("h.2 is: {}", one);

    // or
    println!("h is {:?}", h);

    let mut i = (500, 6.4, 1);
    let (x, y, z) = i;
    println!("The value of x is: {x} and     The value of y is: {y} and The value of z is: {z}");
    i = (600, 56.65, 1);
    println!("i is {:?}", i);

    // Array
    // Fixed-length homogeneous items.
    // Mutable only with mut keyword
    // Indexing via [] syntax e.g myArr.[0] for first element
    let mut j: [i32; 3] = [1, 2, 3];
    let k: [bool; 4] = [false, true, false, true];
    println!("j[1] is: {}", j[1]);
    println!("k is {:?}", k);

    j = [5, 6, 7];
    println!("j is {:?}", j);
}
