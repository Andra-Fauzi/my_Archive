use crate::structure::Token::{
    self, CENTRAL_DIRECTORY_HEADER, END_OF_CENTRAL_DIRECTORY_HEADER, SIGNATURE,
};
use crate::structure::{AFiles, AHeader};
use std::fs;
use std::fs::File;
use std::io::{self, Write};

pub fn get_file_content(path: &String) -> io::Result<Vec<u8>> {
    let contents = fs::read(path)?;
    return Ok(contents);
}

pub fn get_name_from_path(path: &String) -> String {
    let length = path.len();
    let mut name: String = String::new();
    for i in (0..=length - 1).rev() {
        if path.chars().nth(length - 1).unwrap() == '/' {
            continue;
        }
        if path.chars().nth(i).unwrap() == '/' {
            break;
        }
        name = format!("{}{}", path.chars().nth(i).unwrap(), name);
    }
    return name;
}

impl AFiles {
    pub fn new(path: String) -> AFiles {
        let contents = get_file_content(&path).expect("cant get content of file");
        let name = get_name_from_path(&path);
        AFiles { name, contents }
    }
}

impl AHeader {
    pub fn new() -> AHeader {
        AHeader {
            signature: SIGNATURE,
            files: Vec::new(),
        }
    }
    pub fn insert(&mut self, file: AFiles) {
        self.files.push(file);
    }
    pub fn write(&mut self, path: String) {
        let mut w_file = File::create(path).expect("cannot creating file");
        w_file
            .write_all(&self.signature.to_be_bytes())
            .expect("cannot write signature");
        for file in &self.files {
            w_file
                .write_all(&Token::LOCAL_FILE_HEADER.to_be_bytes())
                .expect("cannot write content");
            w_file
                .write_all(&Token::CENTRAL_DIRECTORY_HEADER.to_be_bytes())
                .expect("cannot write content");
            w_file
                .write_all(&file.contents)
                .expect("cannot write content");
            w_file
                .write_all(&Token::END_OF_CENTRAL_DIRECTORY_HEADER.to_be_bytes())
                .expect("cannot write content");
        }
    }
}
