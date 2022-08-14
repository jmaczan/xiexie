use std::{
    fs::{DirBuilder, File},
    io::Write,
};

pub fn write_to_file(path_to_directory: &str, file_name: &str, file_content: String) {
    create_directory_if_not_exist(path_to_directory);
    write_content_to_file(path_to_directory, file_name, file_content);
}

fn create_directory_if_not_exist(path_to_directory: &str) {
    DirBuilder::new()
        .recursive(true)
        .create(path_to_directory)
        .unwrap();
}

fn write_content_to_file(path_to_directory: &str, file_name: &str, file_content: String) {
    File::create(path_to_directory.to_owned() + "/" + file_name)
        .unwrap()
        .write_all(file_content.as_bytes())
        .expect("Failed to write to a file");
}
