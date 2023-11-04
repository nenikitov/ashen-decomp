//! This only works with the last version of Ashen :).

mod nom;

#[allow(clippy::wildcard_imports)]
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

struct EntryHeader {
    offset: u32,
    size: u32,
}

impl PackFile {
    const HEADER: &'static str = "PMAN";
    const COPYRIGHT_LENGTH: usize = 56;

    pub fn new(bytes: &[u8]) -> Result<Self> {
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

    fn entries(input: &[u8], total_entries: u32) -> Result<Vec<EntryHeader>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [u8] = include_bytes!("../../res/packfile.dat");
    const FILE_COUNT: u32 = 158;

    #[test]
    fn packfile_header_works() -> eyre::Result<()> {
        let (_, (copyright, file_count)) = PackFile::header(INPUT)?;

        assert_eq!(copyright, "Copyright (c) 2004 Torus Games Pty. Ltd.");
        assert_eq!(file_count, FILE_COUNT);

        Ok(())
    }

    #[test]
    fn packfile_entries_works() -> eyre::Result<()> {
        #[rustfmt::skip]
        let (_, entries) = PackFile::entries(
            &[
                // File 1
                0x00, 0x00, 0x00, 0x00,
                0x20, 0x0A, 0x00, 0x00,
                0x00, 0x65, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                // File 2
                0x00, 0x00, 0x00, 0x00,
                0x20, 0x6F, 0x00, 0x00,
                0x00, 0x80, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            2
        )?;

        Ok(())
    }
}
