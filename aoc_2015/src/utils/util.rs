use std::fs;

pub fn parse_input(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("should read file input without errors")
}
