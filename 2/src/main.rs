use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let parts = split_by_number(line);
        let numbers = extract_numbers(parts);

        let first_last = first_last(&numbers);
        result.push(first_last);
    }

    result
}

fn main() {
    let lines = read_lines("input.txt");
    let mut sum: u32 = 0;
    for line in lines {
        sum = sum + line.parse::<u32>().unwrap();
    }
    print!("{}", sum);
}

fn first_last(line: &String) -> String {
    
    let mut value = String::from("");
    value.push(line.chars().next().unwrap());
    value.push(line.chars().rev().next().unwrap());

    value
}

fn split_by_number(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            if !current_part.is_empty() {
                parts.push(current_part.clone());
                current_part.clear();
            }
            parts.push(ch.to_string());
        } else {
            current_part.push(ch);
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }

    parts
}
fn extract_numbers(strings: Vec<String>) -> String {
    let number_words = [("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), 
                        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')];

    let mut result = String::new();

    for s in strings {
        // print!("{}   ---   ", s);
        if s.chars().all(|c| c.is_digit(10)) {
            result.push_str(&s);
        } else {
            let mut i = 0;
            while i < s.len() {
                let mut matched = false;
                for (word, num) in number_words.iter() {
                    if s[i..].starts_with(word) {
                        result.push(*num);
                        i += word.len() - 1; 
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    i += 1;
                }
            }
        }
    }

    result
}



