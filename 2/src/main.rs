// Importing the required module to read files.
use std::fs::read_to_string;

// Function to read lines from a file and process each line.
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    // Read each line from the file.
    for line in read_to_string(filename).unwrap().lines() {
        // Split the line into parts where digits and letters are separated.
        let parts = split_by_number(line);
        // Extract numbers from the split parts.
        let numbers = extract_numbers(parts);

        // Process the extracted numbers to get the first and last digit.
        let first_last = first_last(&numbers);
        // Add the processed result to the final vector.
        result.push(first_last);
    }

    // Return the vector containing processed results for each line.
    result
}

// Main function to execute the program.
fn main() {
    // Read and process lines from "input.txt".
    let lines = read_lines("input.txt");
    let mut sum: u32 = 0;

    // Sum up the processed results of each line.
    for line in lines {
        sum += line.parse::<u32>().unwrap();
    }
    // Print the total sum.
    println!("{}", sum);
}

// Function to get the first and last character of a string.
fn first_last(line: &String) -> String {
    // Initialize a new String.
    let mut value = String::from("");
    // Push the first character of the line to the new string.
    value.push(line.chars().next().unwrap());
    // Push the last character of the line to the new string.
    value.push(line.chars().rev().next().unwrap());

    // Return the string containing the first and last character.
    value
}

// Function to split a string into a vector of strings based on digits and non-digit characters.
fn split_by_number(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();

    // Iterate through each character in the input string.
    for ch in input.chars() {
        // Check if the character is a digit.
        if ch.is_digit(10) {
            // If the current part is not empty, add it to parts and reset current_part.
            if !current_part.is_empty() {
                parts.push(current_part.clone());
                current_part.clear();
            }
            // Add the digit as a separate part.
            parts.push(ch.to_string());
        } else {
            // If the character is not a digit, keep adding it to the current part.
            current_part.push(ch);
        }
    }

    // After iterating, if there's a remaining current part, add it to parts.
    if !current_part.is_empty() {
        parts.push(current_part);
    }

    // Return the vector of split parts.
    parts
}

// Function to extract numbers from a vector of strings.
fn extract_numbers(strings: Vec<String>) -> String {
    // Define the mapping of words to their respective digit.
    let number_words = [("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), 
                        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')];

    let mut result = String::new();

    // Iterate through each string in the vector.
    for s in strings {
        // If the string is a digit, add it directly to the result.
        if s.chars().all(|c| c.is_digit(10)) {
            result.push_str(&s);
        } else {
            // If the string is not a digit, process it to extract the number.
            let mut i = 0;
            while i < s.len() {
                let mut matched = false;
                // Iterate through the number words to find a match.
                for (word, num) in number_words.iter() {
                    // Check if the current substring matches any number word.
                    if s[i..].starts_with(word) {
                        // If a match is found, add the corresponding digit to the result.
                        result.push(*num);
                        // Move the index to the end of the matched word minus 1.
                        i += word.len() - 1; 
                        matched = true;
                        break;
                    }
                }
                // If no match is found, move to the next character.
                if !matched {
                    i += 1;
                }
            }
        }
    }

    // Return the final string of extracted numbers.
    result
}