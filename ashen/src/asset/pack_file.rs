//! This only works with the last version of Ashen :).

use crate::utils::nom::*;

#[derive(Debug, PartialEq)]
struct EntryHeader {
    offset: u32,
    size: u32,
}

#[derive(Debug, PartialEq)]
pub struct EntryData {
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct PackFile {
    pub copyright: String,
    pub entries: Vec<EntryData>,
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

        multi::count!(entry_header, total_entries as usize)(input)
    }

    #[allow(clippy::unnecessary_wraps)] // TODO(Unavailable): Rewrite using nom
    fn entries<'a>(
        input: &'a [u8],
        entry_headers: &'_ [EntryHeader],
    ) -> Result<'a, Vec<EntryData>> {
        fn entry(input: &[u8], entry_header: &EntryHeader) -> EntryData {
            let bytes = &input[entry_header.offset as usize..][..entry_header.size as usize];

            EntryData {
                bytes: bytes.to_vec(),
            }
        }

        let entries = entry_headers.iter().map(|h| entry(input, h)).collect();

        Ok((&[], entries))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io, path::PathBuf};

    use super::*;
    use crate::utils::{compression::decompress, test::*};

    #[test]
    fn header_works() -> eyre::Result<()> {
        let (_, (copyright, file_count)) = PackFile::header(b"PMAN\x64\x00\x00\x00Copyright string goes here...\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")?;

        assert_eq!(copyright, "Copyright string goes here...");
        assert_eq!(file_count, 100);

        Ok(())
    }

    #[test]
    fn entries_works() -> eyre::Result<()> {
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
    fn entry_data_works() -> eyre::Result<()> {
        #[rustfmt::skip]
        let (_, entries) = PackFile::entries(
            &[
                // File 1
                b'A', b's', b'h', b'e', b'n',
                // File 2
                b'Z', b'L',
            ],
            &[
                EntryHeader { offset: 0, size: 5 },
                EntryHeader {
                    offset: 5,
                    size: 2,
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
                    bytes: b"ZL".to_vec(),
                }
            ]
        );

        Ok(())
    }

    const ROM_DATA: LazyCell<Vec<u8>> = std::cell::LazyCell::new(|| {
        std::fs::read(workspace_file_path!("rom/packfile.dat")).expect("ROM is present")
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_packfile() -> eyre::Result<()> {
        let (_, pack_file) = PackFile::new(&ROM_DATA)?;

        let output_dir = PathBuf::from(workspace_file_path!(DEFLATED_PATH));

        pack_file
            .entries
            .iter()
            .enumerate()
            .try_for_each(|(i, entry)| -> io::Result<()> {
                let compressed = &entry.bytes;
                let decompressed = &decompress(&entry.bytes);

                output_file(output_dir.join(format!("{i:0>2X}.dat")), &entry.bytes)?;

                if compressed != decompressed {
                    output_file(
                        output_dir.join(format!("{i:0>2X}-deflated.dat")),
                        &decompress(&entry.bytes),
                    )?;
                }

                Ok(())
            })?;

        Ok(())
    }
}
