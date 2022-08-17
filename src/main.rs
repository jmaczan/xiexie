use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
pub mod get_pages_list;
pub mod read_from_file;
pub mod write_to_file;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JSON {
    xiexie: Vec<HashMap<String, String>>,
}

fn main() {
    println!("Starting xiexie 谢谢!");

    let source_directory = "./src/".to_owned() + "sample-source-folder/";
    let target_directory = "./dist";
    let skeleton_file_name = "skeleton.html";
    let source_directory_name_length = source_directory.len() as usize;
    let skeleton_file_path = source_directory.to_owned() + "/" + skeleton_file_name;
    let html_extension = ".html";
    let css_extension = ".css";
    let ttf_extension = ".ttf";
    let woff_extension = ".woff";
    let json_extension = ".json";
    let allowed_assetss_extensions = [css_extension, ttf_extension, woff_extension];

    println!("Reading a website skeleton... 💀🦴");
    let skeleton_html_content = read_from_file::read_from_file(skeleton_file_path.as_str());
    println!("Done 💮");

    write_to_file::set_up_target_directory(target_directory);

    let files_list = match get_pages_list::get_pages_list(source_directory) {
        Ok(pages_list) => pages_list
            .into_iter()
            .map(|page| page.to_str().unwrap().to_owned())
            .collect::<Vec<String>>(),
        Err(_) => {
            println!("I couldn't find source files to generate the website.");
            return;
        }
    };

    files_list
        .clone()
        .into_iter()
        .filter(|file| {
            file.to_lowercase().ends_with(html_extension) && !file.ends_with(skeleton_file_name)
        })
        .for_each(|subpage_path| {
            let mut subpage_content = read_from_file::read_from_file(subpage_path.as_str());
            let source_directory_path = subpage_path.get(..source_directory_name_length).unwrap();
            let subpage_file_name = subpage_path.get(source_directory_name_length..).unwrap();
            let subpage_name = subpage_file_name
                .get(..subpage_file_name.len() - html_extension.len())
                .unwrap();
            let css_file_path = source_directory_path.to_owned() + subpage_name + css_extension;
            let has_css_file = Path::new(&css_file_path).exists();
            let css_file_html_link = "<link rel=\"stylesheet\" href=\"".to_owned()
                + subpage_name
                + css_extension
                + "\" />";

            let json_file_path = source_directory_path.to_owned() + subpage_name + json_extension;
            let has_json_file = Path::new(&json_file_path).exists();

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

            if has_json_file {
                let raw_json_file_content = read_from_file::read_from_file(json_file_path.as_str());
                println!("{:?}", raw_json_file_content);
                let json_file_content =
                    serde_json::from_str::<JSON>(&raw_json_file_content).unwrap();
                println!("{:#?}", json_file_content);
                for (key, value) in json_file_content.xiexie.iter().flat_map(|d| d.iter()) {
                    println!("{} {}", key, value);
                    println!("----------------------");

                    subpage_content =
                        subpage_content.replace(("xiexie::".to_owned() + key).as_str(), value);
                }
            }

            write_to_file::write_to_file(target_directory, subpage_file_name, subpage_content);
        });

    files_list
        .into_iter()
        .filter(|file| {
            !file.to_lowercase().ends_with(html_extension)
                && allowed_assetss_extensions
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
}
