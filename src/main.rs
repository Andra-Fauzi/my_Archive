use std::fs;

use structure::{AFiles, AHeader};

mod archive;
mod compression;
mod extract;
mod structure;

fn main() {
    // for writing file into archive
    /*
    let path = String::from("./archive-file");
    let path2 = String::from("./an.txt");
    let file = AFiles::new(path);
    let file2 = AFiles::new(path2);
    let mut header = AHeader::new();
    header.insert_file(file);
    header.insert_file(file2);
    let path_header = String::from("./archive.waw");
    header.write_archive_to(path_header);
    */

    // for writing extracted files
    let mut header_extract = AHeader::new();
    let file_header = fs::read("./archive.waw").expect("cannot read the file");
    header_extract.parse_header(file_header);
    header_extract.write_extract_files();
}
