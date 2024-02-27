// src/lib.rs
use std::collections::HashMap;

pub fn find_user_tags(input: &str, users: &HashMap<String, String>) -> HashMap<String, String> {
    let mut results = HashMap::new();
    let input_lower = input.to_lowercase();

    for word in input_lower.split_whitespace() {
        let trimmed_word = word.trim_matches(|c: char| !c.is_alphanumeric());
        
        // Iterate through users and compare directly, avoiding duplicate lowercase conversions
        for (username, tag) in users {
            if username.to_lowercase() == trimmed_word {
                results.insert(tag.clone(), trimmed_word.to_string());
                break; // Avoid unnecessary iterations after a match is found
            }
        }
    }

    results
}
