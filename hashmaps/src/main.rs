use std::collections::HashMap;

fn main() {
    // hashmap
    // uses hashing function to store the key and value in memory

    let green_team = String::from("Green");
    let blue_team = String::from("Blue");
    
    let mut scores = HashMap::new();

    // adding value into hashmaps
    scores.insert(green_team, 88);
    scores.insert(blue_team, 38);
    
    // fetching value using get method
    let team_name = String::from("green_team");
    let score = scores.get(&team_name);
    
    // iterating over hashmap's all elements
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating the hashmap

    // this will overwrite existing value
    scores.insert(String::from("Green"), 20);
    scores.insert(String::from("Green"), 210);
    
    // this will not overwrite existing value
    // below code means, if entry for blue is not exist, insert new one. if exist do not perform any operation
    scores.entry(String::from("Blue")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(15);


    // updating value of hashmap based on old value
    let text = "Hello Dhruv Prajapati Hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
