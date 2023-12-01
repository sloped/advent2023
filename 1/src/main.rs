use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let number: String = line.chars().filter(|c: &char| c.is_digit(10)).collect();
        result.push(number.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("input.txt");
    let mut sum: u32 = 0;
    for line in lines {
        let mut value = String::from("");
        value.push(line.chars().next().unwrap());
        value.push(line.chars().rev().next().unwrap());
        sum = sum + value.parse::<u32>().unwrap();
    }
    print!("{}", sum );
}

