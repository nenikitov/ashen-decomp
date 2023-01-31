use std::fmt::Debug;

type Char = u8;

pub enum BinaryError {
    MissingSection { section: &'static str },
    InvalidFormat { section: &'static str, expected: &'static str, actual: Box<dyn Debug> }
}

pub trait BinaryChunk {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError> where Self: Sized;
}




#[derive(Debug)]
pub struct PmanHeader {
    pub pman: [Char; 4],
    pub num_files: u32,
    pub copyright: [Char; 56]
}
impl BinaryChunk for PmanHeader {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError> where Self: Sized {
        todo!()
    }
}


#[derive(Debug)]
pub struct PmanFileDeclaration {
    pub start: u32,
    pub offset: u32,
    pub size: u32,
    pub end: u32
}

#[derive(Debug)]
pub struct PmanFile {
    pub header: PmanHeader,
    pub file_declarations: Vec<PmanFileDeclaration>
}

