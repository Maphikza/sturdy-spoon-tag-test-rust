#![feature(test)]

extern crate test;
use test::Bencher;
use std::collections::HashMap;
use tag_finder::find_user_tags;

// Assuming `find_user_tags` and other necessary code is visible here, either by including it directly or by using a module

#[bench]
fn bench_find_user_tags(b: &mut Bencher) {
    // Setup
    let users: HashMap<String, String> = [
        ("john".to_string(), "@johnny".to_string()),
        ("testuser".to_string(), "@testtag".to_string()),
        // Add more users as needed
    ].iter().cloned().collect();

    let input = "This is a test string with john, and TestUser mentioned.";

    b.iter(|| {
        // The code to benchmark
        find_user_tags(input, &users);
    });
}
