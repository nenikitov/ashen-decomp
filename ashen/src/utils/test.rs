use std::{
    cell::LazyCell,
    env, fs, io,
    path::{Path, PathBuf},
};

pub const WORKSPACE_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."));

pub const DEFLATED_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| WORKSPACE_PATH.join("output").join("deflated"));

pub const PARSED_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| WORKSPACE_PATH.join("output").join("parsed"));

/// Gets the bytes from a file on the "output/deflated" folder.
macro_rules! deflated_file {
    ($file:expr_2021) => {
        std::fs::read(DEFLATED_PATH.join($file))
            .expect("deflated test ran.\nRun `cargo test -- --ignored parse_rom_packfile` before.")
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
pub fn output_file<P>(path: P) -> io::Result<Box<dyn io::Write>>
where
    P: AsRef<Path>,
{
    fn inner(path: &Path) -> io::Result<Box<dyn io::Write>> {
        if !should_skip_write() {
            let parent = path.parent().ok_or(io::ErrorKind::InvalidFilename)?;
            fs::create_dir_all(parent)?;
            let file = fs::File::create(path)?;
            Ok(Box::new(file))
        } else {
            Ok(Box::new(io::sink()))
        }
    }

    inner(path.as_ref())
}

pub(crate) use deflated_file;
