use std::{
    fs::{self, File},
    io::Read,
};

use format::BinaryChunk;

use crate::format::PmanFile;

mod format;

fn main() {
    let file = "packfile.dat";

    let mut f = File::open(&file).expect("File not found");
    let metadata = fs::metadata(&file).expect("Unable to read file metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");

    let header = PmanFile::new_read(&buffer, &mut 0);

    println!("{:#?}", header);
}
