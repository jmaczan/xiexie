use std::fs;

pub fn read_from_file(path_to_file: &str) -> String {
    fs::read_to_string(path_to_file).expect("Failed to read from file.")
}
