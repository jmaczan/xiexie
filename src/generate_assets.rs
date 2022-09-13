use crate::{Args, DISALLOWED_ASSETS_EXTENSIONS};
use clap::Parser;
use std::fs;

pub fn generate_assets(files_list: Vec<String>) {
    files_list
        .into_iter()
        .filter(|file| !only_allowed_assets(file))
        .for_each(|file| {
            copy_to_target_directory(file);
        });
}

fn copy_to_target_directory(file: String) {
    fs::copy(
        &file,
        Args::parse().target.to_owned() + "/" + file.get(Args::parse().source.len()..).unwrap(),
    )
    .unwrap();
}

fn only_allowed_assets(file: &String) -> bool {
    DISALLOWED_ASSETS_EXTENSIONS
        .into_iter()
        .any(|extension| file.to_lowercase().ends_with(extension))
}
