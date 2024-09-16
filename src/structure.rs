#[derive(Debug, Clone)]
pub struct AFiles {
    pub name: String,
    pub contents: Vec<u8>,
}

pub struct AHeader {
    pub signature: u32,
    pub files: Vec<AFiles>,
}

pub mod Token {
    pub const SIGNATURE: u32 = 0x542120;
    pub const LOCAL_FILE_HEADER: u32 = 0x542125;
    pub const CENTRAL_DIRECTORY_HEADER: u32 = 0x542130;
    pub const END_OF_CENTRAL_DIRECTORY_HEADER: u32 = 0x542135;
}
