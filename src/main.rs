pub mod get_pages_list;
pub mod read_from_file;
pub mod write_to_file;

fn main() {
    println!("Starting xiexie 谢谢!");

    let source_directory = "./src/".to_owned() + "sample-source-folder";
    let skeleton_file_name = "skeleton.html";
    let source_directory_name_length = source_directory.len() as usize;
    let skeleton_file_path = source_directory.to_owned() + "/" + skeleton_file_name;

    println!("Reading a website skeleton... 💀🦴");
    let skeleton_html_content = read_from_file::read_from_file(skeleton_file_path.as_str());
    // println!("With text:\n{skeleton_html_content}");
    println!("Done 💮");

    let raw_pages_list = match get_pages_list::get_pages_list(source_directory) {
        Ok(pages_list) => pages_list,
        Err(_) => {
            println!("I couldn't find source files to generate the website.");
            return;
        }
    };

    let pages_list = raw_pages_list
        .into_iter()
        .map(|file| String::from(file.to_str().unwrap()))
        .filter(|file| {
            file.to_lowercase().ends_with(".html") && !file.ends_with(skeleton_file_name)
        })
        .collect::<Vec<String>>();

    println!("{:?}", pages_list);

    pages_list.into_iter().for_each(|subpage_path| {
        let mut subpage_content = read_from_file::read_from_file(subpage_path.as_str());

        subpage_content = skeleton_html_content
            .as_str()
            .replace("xiexie::title", "New title")
            .replace("xiexie::body", subpage_content.as_str());

        write_to_file::write_to_file(
            "./dist",
            subpage_path.get(source_directory_name_length..).unwrap(),
            subpage_content,
        );
    });

    println!("Your website is ready to use! All generated files are inside the dist directory. xiexie! 谢谢!");
}
