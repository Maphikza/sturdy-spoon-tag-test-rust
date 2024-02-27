use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use tag_finder::find_user_tags;

fn main() {
    let start_time = Instant::now();

    let data = fs::read_to_string("users.json").expect("Error reading file");

    let users: HashMap<String, String> = serde_json::from_str(&data).expect("Error unmarshalling JSON");

    // Convert usernames to lowercase right when accessing them instead of creating a new HashMap
    let input = "This is a test string with john, and TestUser mentioned.";
    let found_results = find_user_tags(input, &users);

    println!("Exact tags and corresponding usernames found in the string:");
    for (tag, username) in found_results.iter() {
        println!("Tag: {}, Username: {}", tag, username);
    }

    println!("Time taken for the entire process: {:?}", start_time.elapsed());
}


