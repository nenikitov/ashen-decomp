//! This only works with the last version of Ashen :).

use engine::utils::nom::*;
use flate2::read::ZlibDecoder;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct EntryHeader {
    offset: u32,
    size: u32,
}

#[derive(Debug, PartialEq)]
pub struct EntryData {
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct PackFile {
    copyright: String,
    entries: Vec<EntryData>,
}

impl PackFile {
    const HEADER: &'static str = "PMAN";
    const COPYRIGHT_LENGTH: usize = 56;

    pub fn new(input: &[u8]) -> Result<Self> {
        let (copyright, entries) = {
            let (input, (copyright, total_entries)) = Self::header(input)?;
            let (input, (headers)) = Self::entry_headers(input, total_entries)?;
            (copyright, headers)
        };
        let (input, entries) = Self::entries(input, &entries)?;

        Ok((input, Self { copyright, entries }))
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

    #[allow(clippy::unnecessary_wraps)] // TODO(Unavailable): Rewrite using nom
    fn entries<'a>(
        input: &'a [u8],
        entry_headers: &'_ [EntryHeader],
    ) -> Result<'a, Vec<EntryData>> {
        fn entry(input: &[u8], entry_header: &EntryHeader) -> EntryData {
            let bytes = &input[entry_header.offset as usize..][..entry_header.size as usize];
            let bytes = if let [b'Z', b'L', s1, s2, s3, bytes @ ..] = bytes {
                let size = u32::from_le_bytes([*s1, *s2, *s3, 0]);

                let mut decoder = ZlibDecoder::new(bytes);
                let mut data = Vec::with_capacity(size as usize);
                decoder
                    .read_to_end(&mut data)
                    .expect("Data should be a valid zlib stream");
                // TODO(nenikitov): Check if `data.len() == size`

                data
            } else {
                bytes.to_vec()
            };

            EntryData { bytes }
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

    #[test]
    fn packfile_entry_data_works() -> eyre::Result<()> {
        #[rustfmt::skip]
        let (_, entries) = PackFile::entries(
            &[
                // File 1
                b'A', b's', b'h', b'e', b'n',
                // File 2
                b'Z', b'L', // Asset Zlib signature
                0x06, 0x00, 0x00, // Stream size
                0x78, 0xDA, // Actual Zlib signature
                0x73, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
            ],
            &[
                EntryHeader { offset: 0, size: 5 },
                EntryHeader {
                    offset: 5,
                    size: 19,
                },
            ],
        )?;

        assert_eq!(
            entries,
            [
                EntryData {
                    bytes: b"Ashen".to_vec(),
                },
                EntryData {
                    bytes: b"Ashen\n".to_vec(),
                }
            ]
        );

        Ok(())
    }
}
