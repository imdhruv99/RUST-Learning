fn main() {
    // Vector

    let mut v: Vec<i32> = Vec::new(); // creating empty vector with signed 32

    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector v is: {:?}", v);

    {
        // vec macro to initialize Vector with value
        let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // accessing element in vector using index, &v specifies reference to our vector v2
        let third: &i32 = &v[2];
        println!("The third element is: {}", third);

        // get method returns Option Enum,
        // Vector will give RunTime error for Index out of Bound, because it's stored on Heap.
        // Array will give us the Compile time error for Index out of Bound.
        // but Get method will return None, instead of throwing the error. Kind of Error Handling
        match v.get(200) {
            Some(third) => println!("The third element is: {}", third),
            None => println!("There is no third element."),
        }
    }

    // iterating over the vector element with immutable reference
    for i in &v {
        println!("{}", i);
    }
    // iterating over the vector element with mutable reference
    for i in &mut v {
        println!("{}", i);
    }

    // storing enum inside a vector
    enum SpreadSheetShell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetShell::Int(10),
        SpreadSheetShell::Float(100.00),
        SpreadSheetShell::Text(String::from("Dhruv")),
    ];

    match &row[1] {
        SpreadSheetShell::Int(i) => println!("{}", i),
        _ => println!("Not a Integer")
    };
}
