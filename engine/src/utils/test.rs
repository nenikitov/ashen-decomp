use std::{env, fs, io, path::Path};

pub const PARSED_PATH: &'static str = "output/parsed/";
pub const DEFLATED_PATH: &'static str = "output/deflated/";

/// Gets a path relative to the workspace directory.
macro_rules! workspace_file_path {
    ($file:expr) => {
        const_format::concatcp!(env!("CARGO_MANIFEST_DIR"), "/../", $file)
    };
}

macro_rules! parsed_file_path {
    ($file:expr) => {
        workspace_file_path!(const_format::concatcp!(PARSED_PATH, $file))
    };
}

/// Gets the bytes from a file on the "output/deflated" folder.
macro_rules! deflated_file {
    ($file:expr) => {
        std::cell::LazyCell::new(|| {
            std::fs::read(workspace_file_path!(const_format::concatcp!(
                DEFLATED_PATH,
                $file
            )))
            .expect("deflated test ran.\nRun `cargo test -- --ingored parse_rom_packfile` before")
        })
    };
}

pub fn should_skip_write() -> bool {
    match env::var("SKIP_TEST_WRITE")
        .map(|value| value.to_lowercase())
        .as_deref()
    {
        Ok("true") | Ok("1") => true,
        _ => false,
    }
}

/// Writes to a file creating the directory automatically.
pub fn output_file<P, C>(path: P, contents: C) -> io::Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fn inner(path: &Path, contents: &[u8]) -> io::Result<()> {
        if !should_skip_write() {
            let parent = path.parent().ok_or(io::ErrorKind::InvalidFilename)?;
            fs::create_dir_all(parent)?;
            fs::write(path, contents)
        } else {
            Ok(())
        }
    }

    inner(path.as_ref(), contents.as_ref())
}

pub(crate) use {deflated_file, parsed_file_path, workspace_file_path};
