// using & we pass as references and borrowing
// We can not have mutable reference if we already have immutable
// References must be valid
fn main() {
    let s1 = String::from("Dhruv");
    let cal_len_obj = cal_length(&s1);
    println!("Length of {} is {}.", s1, cal_len_obj);

    let mut s2 = String::from("Dhruv");
    change(&mut s2);
    println!("s2 is {}", s2);
}

// immutable reference
fn cal_length(ss: &String) -> usize {
    let length: usize = ss.len();
    length
}

// mutable reference
fn change(string: &mut String) {
    string.push_str(" Prajapati");
}
