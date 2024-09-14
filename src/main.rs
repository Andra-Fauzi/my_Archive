use archive::get_name_from_path;
use structure::{AFiles, AHeader};

mod archive;
mod structure;
mod extract;

fn main() {
    let path = String::from("./andra.txt");
    let file = AFiles::new(path);
    for content in &file.contents {
        let str_content = char::from(*content);
        print!("{}",str_content);
    }
    let mut header = AHeader::new();
    header.insert(file);
    let path_header = String::from("./archive.waw");
    header.write(path_header);

}
