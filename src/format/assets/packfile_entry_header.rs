use crate::format::{asset_table::AssetType, traits::*};

#[derive(Debug, PartialEq)]
pub struct PackFileEntryHeader {
    pub asset_type: u32,
    pub offset: u32,
    pub length: u32,
}

impl AssetLoad for PackFileEntryHeader {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let mut offset = 0usize;

        // Asset type
        let (asset_type, offset_old) = match read_part::<4>(bytes, &mut offset) {
            Ok(asset_type) => (u32::from_le_bytes(*asset_type.0), asset_type.1),
            Err(error) => {
                let mut error = DataError::from(error);
                error.asset_type = Some(Self::file_type());
                error.section = Some("Asset type".to_string());
                error.offset = Some(offset);
                return Err(error);
            }
        };
        if asset_type != 0 {
            return Err(DataError {
                asset_type: Some(Self::file_type()),
                section: Some("Asset type".to_string()),
                offset: Some(offset_old),
                actual: Box::new(asset_type),
                expected: ExpectedData::Equal { value: Box::new(0) },
            });
        }

        // Offset
        let offset_asset = match read_part::<4>(bytes, &mut offset) {
            Ok(offset_asset) => u32::from_le_bytes(*offset_asset.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.asset_type = Some(Self::file_type());
                error.section = Some("Offset".to_string());
                error.offset = Some(offset);
                return Err(error);
            }
        };

        // Length
        let length = match read_part::<4>(bytes, &mut offset) {
            Ok(length) => u32::from_le_bytes(*length.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.asset_type = Some(Self::file_type());
                error.section = Some("Length".to_string());
                error.offset = Some(offset);
                return Err(error);
            }
        };

        // Reserved
        let (reserved, offset_old) = match read_part::<4>(bytes, &mut offset) {
            Ok(reserved) => (u32::from_le_bytes(*reserved.0), reserved.1),
            Err(error) => {
                let mut error = DataError::from(error);
                error.asset_type = Some(Self::file_type());
                error.section = Some("Reserved".to_string());
                error.offset = Some(offset);
                return Err(error);
            }
        };
        if reserved != 0 {
            return Err(DataError {
                asset_type: Some(Self::file_type()),
                section: Some("Reserved".to_string()),
                offset: Some(offset_old),
                actual: Box::new(reserved),
                expected: ExpectedData::Equal { value: Box::new(0) },
            });
        }

        Ok((
            Self {
                asset_type,
                offset: offset_asset,
                length,
            },
            offset,
        ))
    }

    fn file_type() -> AssetType {
        AssetType::PackFileEntryHeader
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod pack_file_entry_header {
        use super::*;

        mod asset_load {
            use super::*;

            mod load {
                use super::*;

                #[test]
                fn returns_parsed_packfile_entry_header() {
                    let bytes = [
                        0x00, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, 0x00, // Offset
                        0x8B, 0x06, 0x00, 0x00, // Length
                        0x00, 0x00, 0x00, 0x00, // Reserved
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap();

                    assert_eq!(
                        header.0,
                        PackFileEntryHeader {
                            asset_type: 0,
                            offset: 0x180D38,
                            length: 0x68B
                        }
                    );
                    assert_eq!(header.1, 16);
                }

                #[test]
                fn returns_error_if_asset_type_is_not_0() {
                    let bytes = [
                        0x01, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, 0x00, // Offset
                        0x8B, 0x06, 0x00, 0x00, // Length
                        0x00, 0x00, 0x00, 0x00, // Reserved
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Asset type".to_string()),
                            offset: Some(0),
                            actual: Box::new(1),
                            expected: ExpectedData::Equal { value: Box::new(0) }
                        }
                    )
                }

                #[test]
                fn returns_error_if_asset_type_is_not_long_enough() {
                    let bytes = [
                        0x01, 0x00, 0x00, // Asset type INVALID
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Asset type".to_string()),
                            offset: Some(0),
                            actual: Box::new("Tried reading bytes from 0 to 4 when length is 3"),
                            expected: ExpectedData::Other {
                                description: "".to_string()
                            }
                        }
                    )
                }

                #[test]
                fn returns_error_if_offset_is_not_long_enough() {
                    let bytes = [
                        0x00, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, // Offset INVALID
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Offset".to_string()),
                            offset: Some(4),
                            actual: Box::new("Tried reading bytes from 4 to 8 when length is 7"),
                            expected: ExpectedData::Other {
                                description: "".to_string()
                            }
                        }
                    )
                }

                #[test]
                fn returns_error_if_length_is_not_long_enough() {
                    let bytes = [
                        0x00, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, 0x00, // Offset
                        0x8B, 0x06, 0x00, // Length INVALID
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Length".to_string()),
                            offset: Some(8),
                            actual: Box::new("Tried reading bytes from 8 to 12 when length is 11"),
                            expected: ExpectedData::Other {
                                description: "".to_string()
                            }
                        }
                    )
                }

                #[test]
                fn returns_error_if_reserved_is_not_0() {
                    let bytes = [
                        0x00, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, 0x00, // Offset
                        0x8B, 0x06, 0x00, 0x00, // Length
                        0x01, 0x00, 0x00, 0x00, // Reserved
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Reserved".to_string()),
                            offset: Some(12),
                            actual: Box::new(1),
                            expected: ExpectedData::Equal { value: Box::new(0) }
                        }
                    )
                }

                #[test]
                fn returns_error_if_reserved_is_not_long_enough() {
                    let bytes = [
                        0x00, 0x00, 0x00, 0x00, // Asset type
                        0x38, 0x0D, 0x18, 0x00, // Offset
                        0x8B, 0x06, 0x00, 0x00, // Length
                        0x01, 0x00, 0x00, // Reserved INVLAID
                    ];
                    let header = PackFileEntryHeader::load(&bytes).unwrap_err();
                    assert_eq!(
                        header,
                        DataError {
                            asset_type: Some(AssetType::PackFileEntryHeader),
                            section: Some("Reserved".to_string()),
                            offset: Some(12),
                            actual: Box::new("Tried reading bytes from 12 to 16 when length is 15"),
                            expected: ExpectedData::Other {
                                description: "".to_string()
                            }
                        }
                    )
                }
            }

            mod asset_type {
                use super::*;

                #[test]
                fn returns_correctly() {
                    assert_eq!(
                        PackFileEntryHeader::file_type(),
                        AssetType::PackFileEntryHeader
                    )
                }
            }
        }
    }
}
