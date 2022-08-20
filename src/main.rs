use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::ExitCode;
pub mod generate_assets;
pub mod generate_htmls;
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
const TAG_PREFIX: &str = "xiexie::";
const BODY_TAG: &str = "xiexie::body";
const CSS_TAG: &str = "xiexie::css";

fn main() -> ExitCode {
    let args = Args::parse();
    let source_directory = args.source;
    let target_directory = String::from(args.target);

    io::set_up_target_directory(&target_directory);

    let files_list = match io::get_files_list(source_directory) {
        Ok(files_list) => files_list,
        Err(_) => {
            println!("I couldn't find source files to generate the website.");
            return ExitCode::FAILURE;
        }
    };

    generate_htmls::generate_htmls(&files_list);

    generate_assets::generate_assets(files_list);

    println!(
        "Success. Your website is in {} directory.",
        target_directory
    );
    ExitCode::SUCCESS
}
