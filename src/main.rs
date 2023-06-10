#![allow(dead_code)]
#![feature(array_chunks)]

mod format;

use std::fs;

use format::*;

fn main() {
    let path = "rom/packfile.dat";
    let bytes = fs::read(path).expect("Could not read the data file");

    let packfile = PackFile::load(&bytes).unwrap();

    println!("{:#?}", packfile);

    /*
    match PackFile::new_read(&buffer, &mut 0) {
        Ok(file) => {
            // Set up file structure
            let path_output = PathBuf::from("output");
            let path_output_raw = path_output.join("raw");
            let path_output_deflated = path_output.join("deflated");
            let path_output_parsed = path_output.join("parsed");
            let _ = fs::remove_dir_all(&path_output);
            fs::create_dir_all(&path_output).expect("Cannot create the output directory");
            fs::create_dir_all(&path_output_raw).unwrap();
            fs::create_dir_all(&path_output_deflated).unwrap();
            fs::create_dir_all(&path_output_parsed).unwrap();

            // Raw and deflated
            for chunk in file.entries {
                // Raw
                let path_output_raw = path_output_raw.join(
                    format!("{:X}.{}",
                        chunk.offset,
                        if chunk.is_zlib() { "zlib" } else { "dat" }
                    )
                );
                fs::write(
                    &path_output_raw,
                    &chunk.data
                ).expect("Could not write a raw data file");

                // Deflated
                let path_output_deflated = path_output_deflated.join(format!("{:X}.dat", chunk.offset));
                if chunk.is_zlib() {
                    fs::write(
                        &path_output_deflated,
                        chunk.zlib_data().expect("Invalid ZLIB archive")
                    ).expect("Could not write a deflated data file");
                }
                else {
                    fs::copy(
                        &path_output_raw,
                        &path_output_deflated
                    ).expect("Could not copy a deflated data file");
                }
            }
        }
        Err(e) => eprintln!("{e:?}"),
    }
    */
}
