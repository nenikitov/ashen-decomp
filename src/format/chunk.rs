use std::io::Read;

use flate2::read::ZlibDecoder;

use super::data::*;

pub struct PManChunk {
    pub offset: u32,
    pub size: u32,
    pub data: Vec<u8>
}

impl DataFile for PManChunk {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, DataError>
    where Self: Sized {
        let mut read_part = |size| (offset.clone(), read_part(buffer, offset, size));

        // Start
        let (start_padding_offset, start_padding) = read_part(4);
        let start = u32::from_le_bytes(start_padding.try_into().unwrap());
        if start != 0 {
            return Err(DataError {
                file_type: String::from("PMan chunk"),
                offset: start_padding_offset,
                section: String::from("Start padding"),
                exepcted: ExpectedData::Equal {
                    value: Box::new(0)
                },
                actual: Box::new(start_padding.to_owned())
            })
        }

        // Offset
        let offset = u32::from_le_bytes(
            (*read_part(4).1).try_into().unwrap()
        );

        // Size
        let size = u32::from_le_bytes(
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
            offset,
            size,
            data: buffer[(offset as usize) .. (offset + size) as usize].to_vec()
        })
    }
}

impl PManChunk {
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

