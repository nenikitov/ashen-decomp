use std::io::Read;

use flate2::read::ZlibDecoder;

use super::{traits::*, asset_table::AssetType};

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
        }
    }

    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }
}

impl AssetLoad for PackFileEntryData {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
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

    fn file_type() -> AssetType {
        AssetType::PackFileEntryData
    }
}
