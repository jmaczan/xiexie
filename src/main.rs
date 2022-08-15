pub mod get_pages_list;
pub mod read_from_file;
pub mod write_to_file;

fn main() {
    println!("Starting xiexie 谢谢!");

    let source_directory = "./src/".to_owned() + "sample-source-folder";
    let target_directory = "./dist";
    let skeleton_file_name = "skeleton.html";
    let source_directory_name_length = source_directory.len() as usize;
    let skeleton_file_path = source_directory.to_owned() + "/" + skeleton_file_name;
    let html_extension = ".html";

    println!("Reading a website skeleton... 💀🦴");
    let skeleton_html_content = read_from_file::read_from_file(skeleton_file_path.as_str());
    println!("Done 💮");

    let raw_files_list = match get_pages_list::get_pages_list(source_directory) {
        Ok(pages_list) => pages_list,
        Err(_) => {
            println!("I couldn't find source files to generate the website.");
            return;
        }
    };

    let files_list = raw_files_list
        .into_iter()
        .map(|file| String::from(file.to_str().unwrap()));
    println!("{:?}", files_list);
    let pages_list = files_list
        .filter(|file| {
            file.to_lowercase().ends_with(html_extension) && !file.ends_with(skeleton_file_name)
        })
        .collect::<Vec<String>>();

    pages_list.into_iter().for_each(|subpage_path| {
        let mut subpage_content = read_from_file::read_from_file(subpage_path.as_str());
        let subpage_file_name = subpage_path.get(source_directory_name_length..).unwrap();
        let subpage_name = subpage_file_name
            .get(1..subpage_file_name.len() - html_extension.len())
            .unwrap();
        subpage_content = skeleton_html_content
            .as_str()
            .replace("xiexie::title", "New title")
            .replace("xiexie::body", subpage_content.as_str())
            .replace(
                "xiexie::css",
                ("<link rel=\"stylesheet\" href=\"".to_owned() + subpage_name + ".css\" />")
                    .as_str(),
            );

        write_to_file::write_to_file(target_directory, subpage_file_name, subpage_content);
    });

    println!("Your website is ready to use! All generated files are inside the {} directory. xiexie! 谢谢!", target_directory);
}
