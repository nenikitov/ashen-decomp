//! This only works with the last version of Ashen :).

mod nom;

use nom::*;

enum EntryKind {}

pub struct EntryData {
    bytes: Vec<u8>,
    kind: EntryKind,
}

pub struct PackFile {
    copyright: String,
    entries: Vec<EntryData>,
}

impl PackFile {
    const HEADER: &'static str = "PMAN";
    const COPYRIGHT_LENGTH: usize = 56;

    fn new(bytes: &[u8]) -> Result<()> {
        todo!()
    }

    fn header(input: &[u8]) -> Result<(String, u32)> {
        let (input, _) = bytes::tag(Self::HEADER)(input)?;

        let (input, total_entries) = number::le_u32(input)?;

        let (input, copyright) = bytes::take(Self::COPYRIGHT_LENGTH)(input)?;
        let copyright = String::from_utf8_lossy(copyright)
            .trim_end_matches('\0')
            .to_string();

        Ok((input, (copyright, total_entries)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packfile_header_works() -> eyre::Result<()> {
        let (_, (copyright, file_count)) = PackFile::header(b"PMAN\x64\x00\x00\x00Copyright string goes here...\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")?;

        assert_eq!(copyright, "Copyright string goes here...");
        assert_eq!(file_count, 100);

        Ok(())
    }
}
