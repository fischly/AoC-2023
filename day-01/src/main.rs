use std::env;
use std::fs::read_to_string;

const MAPPING: [(&str, &str); 9] = [
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

fn main() {
    let file_path: String = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("You have to provide the file path as argument"));

    let mut input: String = read_to_string(file_path).unwrap_or_else(|_| panic!("Error opening file"));
    // comment out this for loop to get solution for exercise (1)
    for m in MAPPING {
        input = input.replace(m.0, m.1);
    }
    let lines: std::str::Lines<'_> = input.lines();

    let mut sum: u32 = 0;
    for line in lines {
        let digits_only: String = line.chars().filter(|c| c.is_digit(10)).collect();

        let first_digit: u32 = digits_only.chars().nth(0).unwrap().to_digit(10).unwrap();
        let last_digit: u32 = digits_only.chars().last().unwrap().to_digit(10).unwrap();

        sum += first_digit * 10 + last_digit;
    }

    println!("{sum:?}")
}
