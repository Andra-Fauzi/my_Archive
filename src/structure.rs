#[derive(Debug,Clone)]
pub struct AFiles {
    pub name: String,
    pub contents: Vec<u8>,
}

pub struct AHeader {
    pub signature: [u8;4],
    pub files: Vec<AFiles>
}