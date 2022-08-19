use crate::{Args, ALLOWED_ASSETS_EXTENSIONS, HTML_EXTENSION};
use clap::Parser;
use std::fs;

pub fn generate_assets(files_list: Vec<String>) {
    files_list
        .into_iter()
        .filter(|file| {
            !file.to_lowercase().ends_with(HTML_EXTENSION)
                && ALLOWED_ASSETS_EXTENSIONS
                    .into_iter()
                    .any(|extension| file.to_lowercase().ends_with(extension))
        })
        .for_each(|file| {
            fs::copy(
                &file,
                Args::parse().target.to_owned()
                    + "/"
                    + file.get(Args::parse().source.len()..).unwrap(),
            )
            .unwrap();
        });
}
