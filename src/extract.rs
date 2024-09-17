use std::char;
use std::fmt::write;
use std::fs::File;
use std::io::Write;
use std::{fs, str::Chars};

use crate::structure::{self, AFiles, AHeader, Token};

impl AHeader {
    /*
    TODO: Make the parse function for this function
    fn newFromFile(path: String) -> AHeader{
        let file = fs::read(path).expect("cannot reading the file");
        let mut signature: [u8;4] = [0;4];
        for i in 0..=4 {
            signature[i] = file[i];
        }
        assert_eq!(signature,[0x2F, 0x41, 0x43, 0x21]);
    }
    */
    /*
     * type         offset      byte
     * signature    0           4
     * FILE HEADER  4           4
     * START FILE   8           4
     * BUFFER FILE  12          n
     * END OF FILE  n           4
     */
    pub fn parse_header(&mut self, datas: Vec<u8>) {
        let signature = &datas[0..4];
        assert_eq!(*signature, Token::SIGNATURE.to_be_bytes());
        println!("signature correct");
        let mut token_header = 0;
        let mut token_start = 0;
        let mut token_end = 0;
        for i in 0..datas.len() {
            if i + 3 < datas.len() {
                if datas[i..i + 4] == Token::SIGNATURE.to_be_bytes() {
                    println!("found signature");
                } else if datas[i..i + 4] == Token::CENTRAL_DIRECTORY_HEADER.to_be_bytes() {
                    token_start = i;
                    println!("found start");
                } else if datas[i..i + 4] == Token::LOCAL_FILE_HEADER.to_be_bytes() {
                    token_header = i;
                    println!("found file header");
                } else if datas[i..i + 4] == Token::END_OF_CENTRAL_DIRECTORY_HEADER.to_be_bytes() {
                    token_end = i;
                    let file = Self::parse_file(&datas, token_header, token_start, token_end);
                    self.files.push(file);
                    println!("end of file");
                }
            } else {
                println!("out of index");
            }
        }
    }
    // offset of file
    fn parse_file(datas: &[u8], header: usize, start: usize, end: usize) -> AFiles {
        let mut buffer: Vec<u8> = Vec::new();
        let mut name: String = String::new();
        for i in header + 4..start {
            print!("{}", char::from(datas[i]));
            name.push(char::from(datas[i]));
        }
        println!();
        for i in start + 4..end {
            print!("{}", char::from(datas[i]));
            buffer.push(datas[i]);
        }
        println!();
        return AFiles::from(name, buffer);
    }
    pub fn write_extract_files(self) {
        for file in self.files {
            let mut f = File::create(file.name).expect("cant create file");
            f.write_all(&file.contents);
        }
    }
}
