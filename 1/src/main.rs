// Import the module for reading files.
use std::fs::read_to_string;

// Function to read lines from a file and return a vector of strings.
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    // Iterate over each line in the file.
    for line in read_to_string(filename).unwrap().lines() {
        // Filter out non-digit characters and collect the remaining digits into a string.
        let number: String = line.chars().filter(|c: &char| c.is_digit(10)).collect();
        // Add the collected number string to the result vector.
        result.push(number.to_string())
    }

    // Return the vector containing the processed lines.
    result
}

// Main function to execute the program.
fn main() {
    // Read lines from "input.txt" and process them.
    let lines = read_lines("input.txt");
    let mut sum: u32 = 0;

    // Iterate over each processed line.
    for line in lines {
        // Create a new string to hold the value.
        let mut value = String::from("");
        // Add the first character of the line to the value string.
        value.push(line.chars().next().unwrap());
        // Add the last character of the line (in reverse) to the value string.
        value.push(line.chars().rev().next().unwrap());
        // Convert the value string to a number and add it to the sum.
        sum += value.parse::<u32>().unwrap();
    }
    // Print the total sum of the values.
    println!("{}", sum);
}
