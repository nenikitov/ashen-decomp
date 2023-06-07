use std::str;

use super::asset_table::AssetType;
use super::assets::ColorMap;
use super::packfile_entry::*;
use super::traits::*;

#[derive(Debug)]
pub struct PackFile {
    pub copyright: String,
    pub entries: Vec<(
        PackFileEntryHeader,
        PackFileEntryData,
        Option<Box<dyn AssetLoad>>,
    )>,
}

impl AssetLoad for PackFile {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let mut offset = 0;

        // PMAN signature
        let pman = match read_part::<4>(bytes, &mut offset) {
            Ok(pman) => match str::from_utf8(pman.0) {
                Ok(pman) => pman.to_string(),
                Err(error) => {
                    return Err(DataError {
                        file_type: Some(Self::file_type()),
                        section: Some("PMAN signature".to_string()),
                        offset: Some(offset + error.valid_up_to()),
                        actual: Box::new(format!("{:?}", pman.1)),
                        expected: ExpectedData::Other {
                            description: "To be a valid UTF-8 string".to_string(),
                        },
                    })
                }
            },
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("PMAN signature".to_string());
                return Err(error);
            }
        };
        if pman != "PMAN" {
            return Err(DataError {
                file_type: Some(Self::file_type()),
                section: Some("PMAN signature".to_string()),
                offset: Some(offset),
                actual: Box::new(pman),
                expected: ExpectedData::Equal { value: Box::new(0) },
            });
        }

        // Number of entries
        let entries = match read_part::<4>(bytes, &mut offset) {
            Ok(entries) => u32::from_le_bytes(*entries.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Offset".to_string());
                return Err(error);
            }
        };

        // Copyright
        let copyright = match read_part::<56>(bytes, &mut offset) {
            Ok(copyright) => match str::from_utf8(copyright.0) {
                Ok(copyright) => copyright.to_string(),
                Err(error) => {
                    return Err(DataError {
                        file_type: Some(Self::file_type()),
                        section: Some("Copyright".to_string()),
                        offset: Some(offset + error.valid_up_to()),
                        actual: Box::new(format!("{:?}", copyright.1)),
                        expected: ExpectedData::Other {
                            description: "To be a valid UTF-8 string".to_string(),
                        },
                    })
                }
            },
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Copyright".to_string());
                return Err(error);
            }
        };

        // Entries information
        let entries: Vec<_> = (0..entries)
            .map(|i| {
                let (header, header_offset) =
                    PackFileEntryHeader::load(&bytes[offset..]).map_err(|mut e| {
                        if let Some(error_offset) = e.offset.as_mut() {
                            *error_offset += offset;
                        }
                        e
                    })?;

                let (data, _) = PackFileEntryData::load(
                    &bytes[header.offset as usize..(header.offset + header.length) as usize],
                )
                .map_err(|mut e| {
                    if let Some(error_offset) = e.offset.as_mut() {
                        *error_offset += header.offset as usize;
                    }
                    e
                })?;

                let loaded = Self::index_to_asset_type(i).map(|loader| match loader {
                    AssetType::ColorMap => Self::load_with_loader::<ColorMap>(&header, &data),
                    _ => todo!(),
                });
                let loaded = loaded
                    .map(|result| result.map(|(loaded, _)| loaded))
                    .transpose()
                    .map_err(|err| err)?;

                offset += header_offset;
                Ok((header, data, loaded))
            })
            .collect::<Result<_, DataError>>()?;

        Ok((Self { copyright, entries }, offset))
    }

    fn file_type() -> AssetType {
        AssetType::PackFile
    }
}

impl PackFile {
    // TODO cover all cases and remove `Option`
    fn index_to_asset_type(i: u32) -> Option<AssetType> {
        match i {
            1..=9 => Some(AssetType::ColorMap),
            _ => None,
        }
    }

    fn load_with_loader<T: AssetLoad + 'static>(
        header: &PackFileEntryHeader,
        data: &PackFileEntryData,
    ) -> Result<(Box<dyn AssetLoad>, usize), DataError> {
        let bytes = data.data().map_err(|mut e| {
            e.file_type = Some(T::file_type());
            e.offset = Some(header.offset as usize);
            e
        })?;
        T::load(&bytes).map(|(load, size)| (Box::new(load) as Box<dyn AssetLoad>, size))
    }
}
