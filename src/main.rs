use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::ExitCode;
pub mod generate_html;
pub mod io;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JSON {
    purpose: String,
    template: String,
    fields: Vec<HashMap<String, String>>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    source: String,

    #[clap(short, long, value_parser, default_value_t = String::from("./dist"))]
    target: String,
}

const HTML_EXTENSION: &str = ".html";
const CSS_EXTENSION: &str = ".css";
const TTF_EXTENSION: &str = ".ttf";
const WOFF_EXTENSION: &str = ".woff";
const JSON_EXTENSION: &str = ".json";
const TEMPLATE_PURPOSE: &str = "template";
const ALLOWED_ASSETS_EXTENSIONS: [&str; 3] = [CSS_EXTENSION, TTF_EXTENSION, WOFF_EXTENSION];

fn main() -> ExitCode {
    println!("Starting xiexie 谢谢!");

    let args = Args::parse();
    let source_directory = args.source;
    let target_directory = String::from(args.target);

    let source_directory_name_length = source_directory.len() as usize;

    io::set_up_target_directory(&target_directory);

    let files_list = match io::get_files_list(source_directory) {
        Ok(files_list) => files_list,
        Err(_) => {
            println!("I couldn't find source files to generate the website.");
            return ExitCode::FAILURE;
        }
    };

    generate_html::generate_html(&files_list);

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
                target_directory.to_owned()
                    + "/"
                    + file.get(source_directory_name_length..).unwrap(),
            )
            .unwrap();
        });

    println!("Your website is ready to use! All generated files are inside the {} directory. xiexie 谢谢!", target_directory);
    ExitCode::SUCCESS
}
