use std::{
    fs::{DirBuilder, File},
    io::Write,
};

pub fn write_to_file(path_to_directory: &str, file_name: &str, file_content: String) {
    // Create a temporary file.
    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    DirBuilder::new()
        .recursive(true)
        .create(path_to_directory)
        .unwrap();
    let mut file = File::create(path_to_directory.to_owned() + "/" + file_name).unwrap();
    file.write_all(file_content.as_bytes())
        .expect("Failed to write to a file");
}
