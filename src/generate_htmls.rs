use std::path::Path;

use clap::Parser;

use crate::{
    io, Args, BODY_TAG, CSS_EXTENSION, CSS_TAG, HTML_EXTENSION, JSON, JSON_EXTENSION, TAG_PREFIX,
    TEMPLATE_PURPOSE,
};

pub fn generate_htmls(files_list: &Vec<String>) {
    files_list
        .clone()
        .into_iter()
        .filter(|file| file.to_lowercase().ends_with(HTML_EXTENSION))
        .for_each(|file_path| generate_html_file(file_path));
}

fn generate_html_file(file_path: String) {
    let source_directory = Args::parse().source;
    let source_directory_name_length = source_directory.len() as usize;
    let source_directory_path = file_path.get(..source_directory_name_length).unwrap();
    let subpage_file_name = file_path.get(source_directory_name_length..).unwrap();
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

    let template_file_name = json_file_content.template.as_str();
    let template_file_path = source_directory + "/" + template_file_name;

    let template_html_content = io::read_from_file(template_file_path.as_str());

    let css_file_path = source_directory_path.to_owned() + subpage_name + CSS_EXTENSION;
    let has_css_file = Path::new(&css_file_path).exists();
    let css_file_html_link = "<link rel=\"stylesheet\" href=\"".to_owned()
        + subpage_name.get(1..).unwrap()
        + CSS_EXTENSION
        + "\" />";

    let mut subpage_content = io::read_from_file(file_path.as_str());

    subpage_content = template_html_content
        .as_str()
        .replace(BODY_TAG, subpage_content.as_str())
        .replace(
            CSS_TAG,
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
        subpage_content = subpage_content.replace((TAG_PREFIX.to_owned() + key).as_str(), value);
    }

    io::write_to_file(
        Args::parse().target.as_str(),
        subpage_file_name,
        subpage_content,
    );
}
