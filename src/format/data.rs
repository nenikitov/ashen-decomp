use std::fmt::Debug;
use super::packfile_entry::*;

#[derive(Debug)]
pub enum ExpectedData {
    Equal {
        value: Box<dyn Debug>
    },
    Bound {
        min: Option<Box<dyn Debug>>,
        max: Option<Box<dyn Debug>>,
    },
    Other {
        description: String
    }
}


/// Description of an error encountered when reading a file.
///
/// # Examples:
/// - Magic string wasn't correct.
/// - Unexpected value.
#[derive(Debug)]
pub struct DataError {
    /// File type that data was tried to be parsed in.
    pub file_type: String,
    /// Chunk offset from the beginning of the file.
    pub offset: usize,
    /// Name of the section of the data file.
    pub section: String,
    /// Description of what expected data should look like.
    pub exepcted: ExpectedData,
    /// Actual data of the chunk.
    pub actual: Box<dyn Debug>,
}

pub trait DataFile {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, DataError>
    where Self: Sized;
}

pub trait Asset : Into<Vec<u8>> {
    fn extension() -> &'static str;
}

pub(super) fn read_part<'a> (buffer: &'a [u8], offset: &mut usize, size: usize) -> &'a [u8] {
    let start = *offset;
    let end = start + size;

    *offset += size;
    &buffer[start .. end]
}

