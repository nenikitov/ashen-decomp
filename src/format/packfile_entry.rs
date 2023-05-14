use super::traits::*;

#[derive(Debug)]
pub struct PackFileEntry {
    pub asset_type: u32,
    pub offset: u32,
    pub length: u32,
}

impl AssetLoad for PackFileEntry {
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
        "PackFileEntry".to_string()
    }
}

/*
pub struct PackFileEntry {
    pub asset_type: u32,
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>
}

impl DataFile for PackFileEntry {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, DataError>
    where Self: Sized {
        let mut read_part = |size| (offset.clone(), read_part(buffer, offset, size));

        // Start
        let asset_type = u32::from_le_bytes(
            (*read_part(4).1).try_into().unwrap()
        );

        // Offset
        let offset = u32::from_le_bytes(
            (*read_part(4).1).try_into().unwrap()
        );

        // Size
        let length = u32::from_le_bytes(
            (*read_part(4).1).try_into().unwrap()
        );

        // End
        let (end_padding_offset, end_padding) = read_part(4);
        let end = u32::from_le_bytes(end_padding.try_into().unwrap());
        if end != 0 {
            return Err(DataError {
                file_type: String::from("PMan chunk"),
                offset: end_padding_offset,
                section: String::from("End padding"),
                exepcted: ExpectedData::Equal {
                    value: Box::new(0)
                },
                actual: Box::new(end_padding.to_owned())
            })
        }

        Ok(Self {
            asset_type,
            offset,
            length,
            data: buffer[(offset as usize) .. (offset + length) as usize].to_vec()
        })
    }
}

impl PackFileEntry {
    pub fn is_zlib(&self) -> bool {
        return
            self.data[0] == b'Z'
            && self.data[1] == b'L'
            && self.data[5] == 0x78
            && self.data[6] == 0xDA;
    }

    pub fn zlib_data(&self) -> Result<Vec<u8>, DataError> {
        if !self.is_zlib() {
            Err(DataError {
                file_type: String::from("Zlib chunk"),
                offset: 0,
                section: String::from("Header"),
                actual: Box::new(self.data[..=6].to_vec()),
                exepcted: ExpectedData::Equal {
                    value: Box::new(vec![b'Z', b'L', 0, 0, 0, 0x78, 0xDA])
                },
            })
        }
        else {
            let size = u32::from_le_bytes([
                self.data[2],
                self.data[3],
                self.data[4],
                0
            ]);

            let mut d = ZlibDecoder::new(&self.data[5..]);
            let mut s = Vec::<u8>::with_capacity(size as usize);
            let size_calc = d.read_to_end(&mut s).unwrap();
            if size_calc as u32 != size {
                Err(DataError {
                    file_type: String::from("Zlib chunk"),
                    offset: 0,
                    section: String::from("Deflated size"),
                    actual: Box::new(size_calc),
                    exepcted: ExpectedData::Equal {
                        value: Box::new(size),
                    }
                })
            }
            else {
                Ok(s)
            }
        }
    }
}

*/
