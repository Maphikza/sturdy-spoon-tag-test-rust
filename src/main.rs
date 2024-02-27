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

// fn find_user_tags(input: &str, users: &HashMap<String, String>) -> HashMap<String, String> {
//     let mut results = HashMap::new();
//     let input_lower = input.to_lowercase();

//     for word in input_lower.split_whitespace() {
//         let trimmed_word = word.trim_matches(|c: char| !c.is_alphanumeric());
        
//         // Iterate through users and compare directly, avoiding duplicate lowercase conversions
//         for (username, tag) in users {
//             if username.to_lowercase() == trimmed_word {
//                 results.insert(tag.clone(), trimmed_word.to_string());
//                 break; // Avoid unnecessary iterations after a match is found
//             }
//         }
//     }

//     results
// }


