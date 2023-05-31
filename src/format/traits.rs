use std::{
    error,
    fmt::{Debug, Display},
    io,
};

use super::asset_table::AssetType;

pub trait Printable: Debug + Display {}
impl<T> Printable for T where T: Debug + Display {}

/// Expectation of some data.
#[derive(Debug)]
pub enum ExpectedData {
    /// Expected to be equal.
    Equal { value: Box<dyn Printable> },
    /// Expected to be in bound.
    Bound {
        min: Option<Box<dyn Printable>>,
        max: Option<Box<dyn Printable>>,
    },
    /// Expected to something else.
    Other { description: String },
}

impl Display for ExpectedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedData::Equal { value } => {
                write!(f, "equal to {}", value)
            }
            ExpectedData::Bound { min, max } => match (min, max) {
                (None, None) => unreachable!("Invalid bound"),
                (None, Some(max)) => write!(f, "> {max}"),
                (Some(min), None) => write!(f, "<= {min}"),
                (Some(min), Some(max)) => write!(f, "Between {min} and {max}"),
            },
            ExpectedData::Other { description } => write!(f, "{description}"),
        }
    }
}

/// An error that produces while reading data files.
#[derive(Debug)]
pub struct DataError {
    /// Type of the data file.
    /// TODO change to an enum if possible.
    pub file_type: Option<AssetType>,
    /// Section of the data file.
    pub section: Option<String>,
    /// Offset in bytes since the start of the file.
    pub offset: Option<usize>,
    /// Data found.
    pub actual: Box<dyn Printable>,
    /// Data expected.
    pub expected: ExpectedData,
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error encountered")?;
        if let Some(ref file_type) = self.file_type {
            write!(f, " in {file_type}")?;
        }
        if let Some(ref section) = self.section {
            write!(f, " at {section}")?;
        }
        if let Some(ref offset) = self.section {
            write!(f, " (offset {offset})")?;
        }
        writeln!(f, "")?;
        writeln!(f, "- Expected: {}", self.expected)?;
        writeln!(f, "- Actual: {}", self.actual)?;
        Ok(())
    }
}

impl error::Error for DataError {}

impl Into<io::Error> for DataError {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self.to_string())
    }
}

impl From<io::Error> for DataError {
    fn from(error: io::Error) -> Self {
        DataError {
            file_type: None,
            section: None,
            offset: None,
            actual: Box::new(error.to_string()),
            expected: ExpectedData::Other {
                description: "".to_string(),
            },
        }
    }
}

/// Data file after conversion is done.
#[derive(Debug)]
pub struct ConvertedFile {
    /// Data in a modern format.
    data: Vec<u8>,
    /// Path of the output file.
    extension: &'static str,
}

impl ConvertedFile {
    pub fn new(data: Vec<u8>, extension: &'static str) -> Self {
        Self { data, extension }
    }
}

/// File from the that can be loaded.
pub trait AssetLoad: Debug {
    /// Load bytes for the game asset file.
    ///
    /// # Arguments
    ///
    /// - `bytes` - Bytes directly from the asset file.
    ///
    /// # Returns
    ///
    /// - Loaded data.
    /// - Number of bytes read to load the data.
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized;

    fn file_type() -> AssetType
    where
        Self: Sized;
}

/// File from the game that can be converted to modern file format.
pub trait AssetConvert: Debug {
    /// Any additional info necessary for converting a game file.
    ///
    /// For example, a palette for a texture file.
    type Extra;

    /// Convert an asset file to their more-modern format.
    ///
    /// One asset file can be converted to multiple files.
    /// For example, a model can have embedded texture and animation assets.
    fn convert(&self, extra: &Self::Extra) -> Vec<ConvertedFile>;
}

/// Read a part of a buffer and move the offset.
///
/// # Arguments
///
/// - `buffer` - Bytes from the asset file.
/// - `offset` - Starting offset from where to start reading.
/// - `SIZE` - How many bytes to read.
///
/// # Returns
///
/// - Data from the file that was read.
/// - Offset before reading.
pub(super) fn read_part<'a, const SIZE: usize>(
    bytes: &'a [u8],
    offset: &mut usize,
) -> Result<(&'a [u8; SIZE], usize), io::Error> {
    let offset_clone = offset.clone();
    let start = *offset;
    let end = start + SIZE;

    if end > bytes.len() {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!(
                "Tried reading bytes from {start} to {end} when length is {}",
                bytes.len()
            ),
        ))
    } else {
        *offset += SIZE;
        let slice = &bytes[start..end];
        Ok((slice.try_into().unwrap(), offset_clone))
    }
}
