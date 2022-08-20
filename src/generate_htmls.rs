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

    let configuration = read_configuration(source_directory_path, subpage_name);

    if configuration.purpose == TEMPLATE_PURPOSE.to_owned() {
        return;
    }

    let template_html_content = read_template(&configuration, source_directory);

    let css_file_path = source_directory_path.to_owned() + subpage_name + CSS_EXTENSION;
    let has_css_file = Path::new(&css_file_path).exists();
    let css_file_html_link = build_css_link(subpage_name);

    let mut subpage_content = io::read_from_file(file_path.as_str());

    append_css_link(
        &mut subpage_content,
        template_html_content,
        has_css_file,
        css_file_html_link,
    );

    for (key, value) in configuration.fields.iter().flat_map(|field| field.iter()) {
        subpage_content = subpage_content.replace((TAG_PREFIX.to_owned() + key).as_str(), value);
    }

    io::write_to_file(
        Args::parse().target.as_str(),
        subpage_file_name,
        subpage_content,
    );
}

fn append_css_link(
    subpage_content: &mut String,
    template_html_content: String,
    has_css_file: bool,
    css_file_html_link: String,
) {
    *subpage_content = template_html_content
        .as_str()
        .replace(BODY_TAG, subpage_content.as_str())
        .replace(
            CSS_TAG,
            match has_css_file {
                true => css_file_html_link.as_str(),
                false => "",
            },
        );
}

fn build_css_link(subpage_name: &str) -> String {
    "<link rel=\"stylesheet\" href=\"".to_owned()
        + subpage_name.get(1..).unwrap()
        + CSS_EXTENSION
        + "\" />"
}

fn read_template(configuration: &JSON, source_directory: String) -> String {
    let template_file_name = configuration.template.as_str();
    let template_file_path = source_directory + "/" + template_file_name;
    let template_html_content = io::read_from_file(template_file_path.as_str());
    template_html_content
}

fn read_configuration(source_directory_path: &str, subpage_name: &str) -> JSON {
    let raw_json_file_content = io::read_from_file(
        (source_directory_path.to_owned() + subpage_name + JSON_EXTENSION).as_str(),
    );

    serde_json::from_str::<JSON>(&raw_json_file_content).unwrap()
}
