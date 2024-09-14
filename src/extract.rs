use std::fs;
use std::fs::File;

use crate::structure::{self, AHeader};

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
}