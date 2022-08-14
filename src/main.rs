pub mod read_from_file;
pub mod write_to_file;

fn main() {
    println!("Starting xiexie 谢谢!");

    let source_directory = "./src/".to_owned() + "sample-source-folder" + "/" + "skeleton.html";

    println!("Reading a website skeleton... 💀🦴");
    let skeleton_html_content = read_from_file::read_from_file(source_directory.as_str());
    println!("With text:\n{skeleton_html_content}");
    println!("Done 💮");

    let mut replaced_file = skeleton_html_content
        .as_str()
        .replace("xiexie::title", "New title");
    replaced_file = replaced_file
        .as_str()
        .replace("xiexie::body", "<p>Body is here</p>");
    println!("You're website is ready to use! All generated files are inside the xiexie-build directory. xiexie! 谢谢!");

    write_to_file::write_to_file("./dist", "skeleton-filled.html", replaced_file);
}
