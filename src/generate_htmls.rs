use std::{collections::HashMap, path::Path};

use clap::Parser;

use crate::{
    io, Args, AGGREGATOR_PURPOSE, BODY_TAG, CSS_EXTENSION, CSS_TAG, HTML_EXTENSION, JSON,
    JSON_EXTENSION, TAG_PREFIX, TEMPLATE_PURPOSE,
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

    let mut subpage_content = io::read_from_file(file_path.as_str());

    if configuration.purpose == AGGREGATOR_PURPOSE.to_owned() {
        configuration
            .aggregations
            .unwrap()
            .iter()
            .flat_map(|field| field.iter())
            .into_iter()
            .for_each(|(aggregation_name, aggregation)| {
                println!("{} {:?}", aggregation_name, aggregation);
                let _: &String = aggregation_name;
                let _: &Vec<HashMap<std::string::String, std::string::String>> = aggregation;

                let aggregation_template_start_tag =
                    "<xiexie::aggregation::".to_owned() + aggregation_name + ">";
                let aggregation_template_end_tag =
                    "</xiexie::aggregation::".to_owned() + aggregation_name + ">";

                let aggregation_template_start: usize;
                match subpage_content.find(&(aggregation_template_start_tag)) {
                    Some(value) => aggregation_template_start = value,
                    None => {
                        return;
                    }
                };

                let aggregation_template_end: usize;
                match subpage_content.find(&(aggregation_template_end_tag)) {
                    Some(value) => aggregation_template_end = value,
                    None => {
                        return;
                    }
                };

                let aggregation_template: &str;
                match subpage_content.get(
                    aggregation_template_start + aggregation_template_start_tag.len()
                        ..aggregation_template_end,
                ) {
                    Some(value) => aggregation_template = value,
                    None => {
                        return;
                    }
                }

                println!("{}", aggregation_template);
                // find position of opening and closing tag - <xiexie::aggregation::{aggregation_name}> and </xiexie::aggregation::{aggregation_name}>
                // if not found, omit further mapping for this aggregation_name
                // if found:
                //  make a copy of content between start of opening and end of closing tag, call it aggregation_template
                //  make an empty string, call it aggregation_html
                //  iterate over all aggregation_items in an aggregation and for each:
                //  copy an aggregation_template as aggregation_item_html
                //  open {aggregation_item_name}.json
                //  iterate over fields and for each field:
                //  replace each xiexie::aggregation::{aggregation_name}::{field} with field's value in aggregation_item_html
                //  append aggregation_item_html to the end of aggregation_html
                // replace content in tag with aggregation_html
                // after everything is finished, write_to_target_file(subpage_file_name, subpage_content)
                aggregation
                    .into_iter()
                    .flat_map(|field| field.iter())
                    .for_each(|(aggregation_item_name, aggregation_item)| {
                        println!(
                            "{:?} {:?} {:?}",
                            aggregation_name, aggregation_item_name, aggregation_item
                        );
                        // *subpage_content = subpage_content.replace((TAG_PREFIX.to_owned() + aggregation).as_str(), content);
                        // let file_configuration = read_configuration(source_directory_path,aggregation_item_name);
                        // file_configuration.fields.unwrap()
                    });
                // for (aggregation_item, content) in aggregation.iter().flat_map(|field| field.iter()) {}
                // *subpage_content =
                //     subpage_content.replace((TAG_PREFIX.to_owned() + aggregation).as_str(), content);
            });
        write_to_target_file(subpage_file_name, subpage_content);
        return;
    }

    append_css_link(
        &mut subpage_content,
        template_html_content,
        source_directory_path,
        subpage_name,
    );

    replace_tags(configuration, &mut subpage_content);

    write_to_target_file(subpage_file_name, subpage_content);
}

fn write_to_target_file(subpage_file_name: &str, subpage_content: String) {
    io::write_to_file(
        Args::parse().target.as_str(),
        subpage_file_name,
        subpage_content,
    );
}

fn replace_tags(configuration: JSON, subpage_content: &mut String) {
    for (tag, content) in configuration
        .fields
        .unwrap()
        .iter()
        .flat_map(|field| field.iter())
    {
        *subpage_content = subpage_content.replace((TAG_PREFIX.to_owned() + tag).as_str(), content);
    }
}

fn append_css_link(
    subpage_content: &mut String,
    template_html_content: String,
    source_directory_path: &str,
    subpage_name: &str,
) {
    let css_file_path = source_directory_path.to_owned() + subpage_name + CSS_EXTENSION;
    let has_css_file = Path::new(&css_file_path).exists();
    let css_file_html_link = build_css_link(subpage_name);

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
        + Path::new(subpage_name)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
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
