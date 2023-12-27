pub mod nom;

pub mod fs {
    /// Gets a path relative to the workspace directory.
    macro_rules! workspace_file {
        ($file:literal) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../", $file)
        };
    }

    /// Writes to a file creating the directory automatically.
    pub fn output_file<P, C>(path: P, contents: C) -> io::Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        fn inner(path: &Path, contents: &[u8]) -> io::Result<()> {
            let parent = path.parent().ok_or(io::ErrorKind::InvalidFilename)?;
            fs::create_dir_all(parent)?;
            fs::write(path, contents)
        }

        inner(path.as_ref(), contents.as_ref())
    }

    use std::{fs, io, path::Path};

    pub(crate) use workspace_file;
}
