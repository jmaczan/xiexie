use std::path::Path;
use std::{
    fs::{read_dir, remove_dir_all, DirBuilder, File},
    io,
    io::Write,
    path::PathBuf,
};

pub fn get_files_list(source_path: String) -> io::Result<Vec<PathBuf>> {
    let mut entries = read_dir(source_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
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
