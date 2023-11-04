//! This only works with the last version of Ashen :).

mod nom;

#[allow(clippy::wildcard_imports)]
use nom::*;

#[derive(Debug, PartialEq)]
enum EntryKind {
    // TODO(nenikitov): Add more kinds
    Unknown,
}

#[derive(Debug, PartialEq)]
struct EntryHeader {
    offset: u32,
    size: u32,
}

#[derive(Debug, PartialEq)]
pub struct EntryData {
    bytes: Vec<u8>,
    kind: EntryKind,
}

#[derive(Debug, PartialEq)]
pub struct PackFile {
    copyright: String,
    entries: Vec<EntryData>,
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

    fn entry_headers(input: &[u8], total_entries: u32) -> Result<Vec<EntryHeader>> {
        fn entry_header(input: &[u8]) -> Result<EntryHeader> {
            // TODO(nenikitov): add check for `asset_kind == 0`
            let (input, asset_kind) = number::le_u32(input)?;

            let (input, offset) = number::le_u32(input)?;

            let (input, size) = number::le_u32(input)?;

            // TODO(nenikitov): add check for `reserved == 0`
            let (input, reserved) = number::le_u32(input)?;

            Ok((input, EntryHeader { offset, size }))
        }

        multi::count(entry_header, total_entries as usize)(input)
    }

    fn entries<'a>(
        input: &'a [u8],
        entry_headers: &'_ [EntryHeader],
    ) -> Result<'a, Vec<EntryData>> {
        fn entry<'a>(input: &'a [u8], entry_header: &'_ EntryHeader) -> EntryData {
            EntryData {
                bytes: input[entry_header.offset as usize..][..entry_header.size as usize].to_vec(),
                kind: EntryKind::Unknown,
            }
        }

        let entries = entry_headers.iter().map(|h| entry(input, h)).collect();

        Ok((&[], entries))
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

    #[test]
    fn packfile_entries_works() -> eyre::Result<()> {
        #[rustfmt::skip]
        let (_, entries) = PackFile::entry_headers(
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

        assert_eq!(
            entries,
            [
                EntryHeader {
                    offset: 0x0A20,
                    size: 0x6500,
                },
                EntryHeader {
                    offset: 0x6F20,
                    size: 0x8000,
                },
            ]
        );

        Ok(())
    }
}
