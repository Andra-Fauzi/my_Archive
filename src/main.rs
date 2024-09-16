use std::fs;

use structure::{AFiles, AHeader};

mod archive;
mod extract;
mod structure;

fn main() {
    let path = String::from("./andra.txt");
    let file = AFiles::new(path);
    for content in &file.contents {
        let str_content = char::from(*content);
        print!("{}", str_content);
    }
    let mut header = AHeader::new();
    header.insert(file);
    let path_header = String::from("./archive.waw");
    header.write(path_header);

    let file_header = fs::read("./archive.waw").expect("cannot read the file");
    AHeader::parse_header(file_header);
}
