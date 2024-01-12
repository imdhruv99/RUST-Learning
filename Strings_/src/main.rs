use unicode_segmentation::UnicodeSegmentation as us;

fn main() {
    // Strings are stored as collection of UTF-8 encoded bytes
    let mut s1 = String::new();
    let s2 = "This is string as well";
    let mut s3 = s2.to_string();
    let s4 = String::from("This is also a String");

    // appeding to string
    s1.push_str("hello world");
    s3.push('!');
    let final_string = s1 + &s3;

    // printing
    println!("{}", final_string);
    println!("{}", s2);
    println!("{}", s4);

    // format macro
    let s5 = "Dhruv";
    let s6 = " Prajapati";
    println!("{}{}", s5, s6); // format macro does not take ownership of the string, so we can still use the string in next line

    // indexing
    let s7: String = String::from("Dhruv");
    // below code will throw error, string can't be index by int, cz Strings are UTF-8 encoded and UTF-8 has different size for different char.
    // let c: char = s7[0];  
    // strings can be represented as 3 diff ways
    // 1. Collection of Bytes
    for b in "કેમ છો!".bytes() {
        println!("This is bytes of કેમ છો! :{}", b);
    }
    // 2. Scaler Values
    for c in "કેમ છો!".chars() {
        println!("This is scaler value of કેમ છો! :{}", c);
    }
    // 3. Grapheme Cluster --> for this we need to import crate named unicode-segmentation 
    for d in "કેમ છો!".graphemes(true) {
        println!("This is Grapheme cluster of કેમ છો! :{}", d);
    }
}
