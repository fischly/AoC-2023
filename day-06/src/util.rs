use std::env;
use std::fs::read_to_string;

pub fn read_input() -> String {
    let file_path: String = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("You have to provide the file path as argument"));

    read_to_string(file_path).unwrap_or_else(|_| panic!("Error opening file"))
}
