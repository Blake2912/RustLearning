use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);


    // Accessing for a single entry
    let team_name = String::from("yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of blue team: {score}");

    // Iterating over a list of entries
    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    // Overwriting a value
    scores.insert(String::from("blue"), 30);
    println!("{:?}", scores);

    // Inserting a value if not present
    scores.entry(String::from("red")).or_insert(25);

    println!("{:?}", scores);
    
    // Updating a Value Based on the old value
    println!("\n");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // Entry gives the key and value for inplace manipulation.
        *count += 1;
    }

    println!("{:?}", map);


}
