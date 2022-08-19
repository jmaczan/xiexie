use std::path::Path;
use std::{
    fs::{read_dir, remove_dir_all, DirBuilder, File},
    io,
    io::Write,
};

pub fn get_files_list(source_path: String) -> io::Result<Vec<String>> {
    let mut files = read_dir(source_path)?
        .map(|file| file.map(|e| e.path()))
        .map(|file| file.unwrap().to_str().unwrap().to_owned())
        .collect::<Vec<String>>();

    files.sort();

    Ok(files)
}

pub fn write_to_file(target_path: &str, file_name: &str, file_content: String) {
    File::create(target_path.to_owned() + "/" + file_name)
        .unwrap()
        .write_all(file_content.as_bytes())
        .expect("Failed to write to a file");
}

pub fn set_up_target_directory(target_path: &str) {
    remove_directory_with_content(target_path);
    create_directory(target_path);
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
