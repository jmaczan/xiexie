use std::path::Path;
use std::{
    fs::{remove_dir_all, DirBuilder, File},
    io::Write,
};

pub fn write_to_file(directory_path: &str, file_name: &str, file_content: String) {
    write_content_to_file(directory_path, file_name, file_content);
}

pub fn set_up_target_directory(directory_path: &str) {
    remove_directory_with_content(directory_path);
    create_directory(directory_path);
}

fn remove_directory_with_content(directory_path: &str) {
    if !Path::new(directory_path).exists() {
        return;
    }

    remove_dir_all(directory_path).expect("Couldn't remove directory.");
}

fn create_directory(directory_path: &str) {
    DirBuilder::new()
        .recursive(true)
        .create(directory_path)
        .unwrap();
}

fn write_content_to_file(directory_path: &str, file_name: &str, file_content: String) {
    File::create(directory_path.to_owned() + "/" + file_name)
        .unwrap()
        .write_all(file_content.as_bytes())
        .expect("Failed to write to a file");
}
