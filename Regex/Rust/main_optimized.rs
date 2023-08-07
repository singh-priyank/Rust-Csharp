use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct Pattern {
    pattern: String,
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Patterns {
    patterns: Vec<Pattern>,
}

fn regex_test() {
    // Read the JSON file containing the regex patterns and replacements
    let patterns_json = fs::read_to_string("patterns.json").expect("Failed to read patterns.json");
    let patterns: Patterns = serde_json::from_str(&patterns_json).expect("Failed to parse JSON");

    // Read the input text from the TXT file
    let input_text = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut replacements = HashMap::new();

    for pattern in patterns.patterns {
        replacements.insert(pattern.pattern, pattern.label);
    }

    let mut result_text = input_text.to_string();

    // Compile the regex patterns outside the loop
    // OPTIMIZATION
    let regexes: Vec<Regex> = replacements
        .keys()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();

    // Measure the time taken to perform the regex replacements
    let start_time = Instant::now();

    // Iterate over the regex patterns and replace the matches in the result_text
    for (regex, replacement) in regexes.iter().zip(replacements.values()) {
        result_text = regex.replace_all(&result_text, replacement.to_string()).to_string();
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;

    // println!("Original text: {}", input_text);
    // println!("Replaced text: {}", result_text);
    println!("Time taken: {:?} and size of text file: {}", duration, (input_text.len() as f64)/1024.0);
}

fn main() {
    regex_test()
}


// Get dummy PII data -> https://pii-tools.com/pii-examples/