use std::{
    fs,
    path::Path,
};
use format::BinaryChunk;
use crate::format::PmanFile;


mod format;


fn main() {
    let path = "packfile.dat";
    let buffer = fs::read(path).expect("Could not read the data file");

    match PmanFile::new_read(&buffer, &mut 0) {
        Ok(file) => {
            let path = Path::new("output");
            fs::create_dir_all(path).expect("Cannot create an output directory");
            for (declaration, file) in file.file_declarations.iter().zip(file.files) {
                let path = path.join(format!("{:X}.dat", declaration.offset));
                if file.is_zlib() {
                    println!("Found ZLIB file at {:X}", declaration.offset);
                }
                fs::write(path, file.data).expect("Could not write a data file");
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
