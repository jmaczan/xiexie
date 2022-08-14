use std::{fs, io, path::PathBuf};

pub fn get_pages_list(path_to_directory: String) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path_to_directory)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}
