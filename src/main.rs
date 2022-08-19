use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
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

    files_list
        .clone()
        .into_iter()
        .filter(|file| file.to_lowercase().ends_with(HTML_EXTENSION))
        .for_each(|subpage_path| {
            let source_directory_path = subpage_path.get(..source_directory_name_length).unwrap();
            let subpage_file_name = subpage_path.get(source_directory_name_length..).unwrap();
            let subpage_name = subpage_file_name
                .get(..subpage_file_name.len() - HTML_EXTENSION.len())
                .unwrap();

            let raw_json_file_content = io::read_from_file(
                (source_directory_path.to_owned() + subpage_name + JSON_EXTENSION).as_str(),
            );
            let json_file_content = serde_json::from_str::<JSON>(&raw_json_file_content).unwrap();

            if json_file_content.purpose == TEMPLATE_PURPOSE.to_owned() {
                return;
            }

            let skeleton_file_name = json_file_content.template.as_str();
            let source_directory = Args::parse().source;
            let skeleton_file_path = source_directory + "/" + skeleton_file_name;

            let skeleton_html_content = io::read_from_file(skeleton_file_path.as_str());

            let css_file_path = source_directory_path.to_owned() + subpage_name + CSS_EXTENSION;
            let has_css_file = Path::new(&css_file_path).exists();
            let css_file_html_link = "<link rel=\"stylesheet\" href=\"".to_owned()
                + subpage_name.get(1..).unwrap()
                + CSS_EXTENSION
                + "\" />";

            let mut subpage_content = io::read_from_file(subpage_path.as_str());

            subpage_content = skeleton_html_content
                .as_str()
                .replace("xiexie::body", subpage_content.as_str())
                .replace(
                    "xiexie::css",
                    match has_css_file {
                        true => css_file_html_link.as_str(),
                        false => "",
                    },
                );

            for (key, value) in json_file_content
                .fields
                .iter()
                .flat_map(|field| field.iter())
            {
                subpage_content =
                    subpage_content.replace(("xiexie::".to_owned() + key).as_str(), value);
            }

            io::write_to_file(&target_directory, subpage_file_name, subpage_content);
        });

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
