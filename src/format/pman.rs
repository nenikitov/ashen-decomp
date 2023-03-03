use super::{chunk::PManChunk, data::*};

pub struct PMan {
    pub copyright: String,
    pub chunks: Vec<PManChunk>
}

impl DataFile for PMan {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, DataError>
    where Self: Sized {
        let mut read_part = |size| (offset.clone(), read_part(buffer, offset, size));

        // PMAN
        let (pman_offset, pman) = read_part(4);
        let pman =
            if let Ok(s) = std::str::from_utf8(pman) {
                s
            }
            else {
                return Err(DataError {
                    file_type: String::from("PMan file"),
                    offset: pman_offset,
                    section: String::from("Header"),
                    exepcted: ExpectedData::Other {
                        description: String::from("Not a UTF-8 string")
                    },
                    actual: Box::new(pman.to_owned()),
                })
            };
        if pman != "PMAN" {
            return Err(DataError {
                file_type: String::from("PMan file"),
                offset: pman_offset,
                section: String::from("Header"),
                exepcted: ExpectedData::Equal {
                    value: Box::new(String::from("PMAN"))
                },
                actual: Box::new(pman.to_owned()),
            })
        }

        // Number of files
        let num_files = u32::from_le_bytes(
            (*read_part(4).1).try_into().unwrap()
        );

        // Copyright
        let (copyright_offset, copyright) = read_part(56);
        let copyright =
            if let Ok(s) = std::str::from_utf8(copyright) {
                s
            }
            else {
                return Err(DataError {
                    file_type: String::from("PMan file"),
                    offset: copyright_offset,
                    section: String::from("Copyright"),
                    exepcted: ExpectedData::Other {
                        description: String::from("Not a UTF-8 string")
                    },
                    actual: Box::new(pman.to_owned()),
                })
            };

        // Chunks
        let chunks = (0 .. num_files)
            .map(|_| PManChunk::new_read(buffer, offset))
            .collect::<Result::<_, _>>()?;

        Ok(Self {
            copyright: copyright.to_owned(),
            chunks
        })
    }
}

