use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::format::{asset_table::AssetType, traits::*};

#[derive(Debug)]
pub struct PackFileEntryData {
    pub data: Vec<u8>,
}

impl PackFileEntryData {
    fn is_zlib(bytes: &[u8]) -> bool {
        return bytes[0] == b'Z' && bytes[1] == b'L' && bytes[5] == 0x78 && bytes[6] == 0xDA;
    }
}

impl AssetLoad for PackFileEntryData {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let data = if !Self::is_zlib(bytes) {
            Ok(bytes.to_vec())
        } else {
            let size = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], 0]);
            let mut decoder = ZlibDecoder::new(&bytes[5..]);
            let mut data = Vec::<u8>::with_capacity(size as usize);
            let size_calc = decoder.read_to_end(&mut data).map_err(|e| DataError {
                asset_type: None,
                section: Some("Zlib data stream".to_string()),
                offset: None,
                actual: Box::new(format!("{:?}", &bytes[5..])),
                expected: ExpectedData::Other {
                    description: e.to_string(),
                },
            })?;
            if size_calc as u32 != size {
                Err(DataError {
                    asset_type: None,
                    section: Some("Zlib data size".to_string()),
                    offset: None,
                    actual: Box::new(size_calc),
                    expected: ExpectedData::Equal {
                        value: Box::new(size),
                    },
                })
            } else {
                Ok(data)
            }
        }?;
        let length = data.len();

        Ok((Self { data }, length))
    }

    fn file_type() -> AssetType {
        AssetType::PackFileEntryData
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod pack_file_entry_data {
        use super::*;

        mod is_zlib {
            use super::*;

            #[test]
            fn returns_true_for_asset_zlib_stream() {
                let data = [
                    b'Z', b'L', // Asset Zlib signature
                    0x06, 0x00, 0x00, // Stream size
                    0x78, 0xDA, // Actual Zlib signature
                    0x73, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
                ];
                assert_eq!(PackFileEntryData::is_zlib(&data), true);
            }

            #[test]
            fn returns_false_for_asset_non_zlib_stream() {
                let data = "Hello".as_bytes();
                assert_eq!(PackFileEntryData::is_zlib(&data), false);
            }
        }

        mod data {
            use super::*;

            #[test]
            fn returns_deflated_data_for_asset_zlib_stream() {
                let data = [
                    b'Z', b'L', // Asset Zlib signature
                    0x06, 0x00, 0x00, // Stream size
                    0x78, 0xDA, // Actual Zlib signature
                    0x73, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
                ];
                let data = PackFileEntryData::load(&data).unwrap().0;
                assert_eq!(data.data, "Ashen\n".as_bytes());
            }

            #[test]
            fn returns_error_if_steam_has_invalid_size() {
                let data = [
                    b'Z', b'L', // Asset Zlib signature
                    0x01, 0x00, 0x00, // Stream size INVALID
                    0x78, 0xDA, // Actual Zlib signature
                    0x73, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
                ];
                let data = PackFileEntryData::load(&data).unwrap_err();
                assert_eq!(
                    data,
                    DataError {
                        asset_type: None,
                        section: Some("Zlib data size".to_string()),
                        offset: None,
                        actual: Box::new(6),
                        expected: ExpectedData::Equal { value: Box::new(1) }
                    }
                );
            }

            #[test]
            fn returns_error_if_steam_has_corruption() {
                let bytes = [
                    b'Z', b'L', // Asset Zlib signature
                    0x06, 0x00, 0x00, // Stream size INVALID
                    0x78, 0xDA, // Actual Zlib signature
                    0x72, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
                ];
                let data = PackFileEntryData::load(&bytes).unwrap_err();
                assert_eq!(
                    data,
                    DataError {
                        asset_type: None,
                        section: Some("Zlib data stream".to_string()),
                        offset: None,
                        actual: Box::new(format!("{:?}", &bytes[5..])),
                        expected: ExpectedData::Other {
                            description: "corrupt deflate stream".to_string()
                        }
                    }
                );
            }
        }
    }
}
