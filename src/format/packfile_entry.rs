use std::io::Read;

use flate2::read::ZlibDecoder;

use super::traits::*;

#[derive(Debug)]
pub struct PackFileEntryHeader {
    pub asset_type: u32,
    pub offset: u32,
    pub length: u32,
}

impl AssetLoad for PackFileEntryHeader {
    type Data = ();

    fn load(bytes: &[u8], _: Self::Data) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let mut offset = 0usize;

        // Asset type
        let asset_type = match read_part::<4>(bytes, &mut offset) {
            Ok(asset_type) => u32::from_le_bytes(*asset_type.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Asset type".to_string());
                return Err(error);
            }
        };
        if asset_type != 0 {
            return Err(DataError {
                file_type: Some(Self::file_type()),
                section: Some("Asset type".to_string()),
                offset: Some(offset),
                actual: Box::new(asset_type),
                expected: ExpectedData::Equal { value: Box::new(0) },
            });
        }

        // Offset
        let offset_asset = match read_part::<4>(bytes, &mut offset) {
            Ok(offset_asset) => u32::from_le_bytes(*offset_asset.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Offset".to_string());
                return Err(error);
            }
        };

        // Length
        let length = match read_part::<4>(bytes, &mut offset) {
            Ok(length) => u32::from_le_bytes(*length.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Offset".to_string());
                return Err(error);
            }
        };

        // Reserved
        let reserved = match read_part::<4>(bytes, &mut offset) {
            Ok(reserved) => u32::from_le_bytes(*reserved.0),
            Err(error) => {
                let mut error = DataError::from(error);
                error.file_type = Some(Self::file_type());
                error.section = Some("Asset type".to_string());
                return Err(error);
            }
        };
        if reserved != 0 {
            return Err(DataError {
                file_type: Some(Self::file_type()),
                section: Some("Reserved".to_string()),
                offset: Some(offset),
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

    fn file_type() -> String {
        "PackFileEntryHeader".to_string()
    }
}

#[derive(Debug)]
pub struct PackFileEntryData {
    data: Vec<u8>,
}

impl PackFileEntryData {
    fn is_zlib(&self) -> bool {
        return self.data[0] == b'Z'
            && self.data[1] == b'L'
            && self.data[5] == 0x78
            && self.data[6] == 0xDA;
    }

    pub fn data(&self) -> Result<Vec<u8>, DataError> {
        if !self.is_zlib() {
            Ok(self.data.clone())
        } else {
            let size = u32::from_le_bytes([self.data[2], self.data[3], self.data[4], 0]);
            let mut decoder = ZlibDecoder::new(&self.data[5..]);
            let mut data = Vec::<u8>::with_capacity(size as usize);
            let size_calc = decoder.read_to_end(&mut data).expect("Can read the buffer");
            if size_calc as u32 != size {
                Err(DataError {
                    file_type: None,
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
        }
    }

    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }
}

impl AssetLoad for PackFileEntryData {
    type Data = ();

    fn load(bytes: &[u8], _: Self::Data) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        Ok((
            Self {
                data: bytes.to_vec(),
            },
            bytes.len(),
        ))
    }

    fn file_type() -> String {
        "PackFileEntryData".to_string()
    }
}
