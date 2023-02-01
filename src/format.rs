use std::fmt::Debug;
use std::str;

#[derive(Debug)]
pub enum BinaryError {
    InvalidSection {
        section: &'static str,
        offset: usize,
    },
    InvalidFormat {
        section: &'static str,
        offset: usize,
        expected: Box<dyn Debug>,
        actual: Box<dyn Debug>,
    },
}

fn read_part<'a>(buffer: &'a [u8], offset: &mut usize, size: usize) -> &'a [u8] {
    let start = *offset;
    let end = start + size;

    *offset += size;
    &buffer[start .. end]
}

pub trait BinaryChunk {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError>
    where Self: Sized;
}

#[derive(Debug)]
pub struct PmanHeader {
    pub num_files: u32,
    pub copyright: String,
}
impl BinaryChunk for PmanHeader {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError>
    where Self: Sized {
        let mut read_part = |size| (offset.clone(), read_part(buffer, offset, size));

        // PMAN
        let (pman_offset, pman) = read_part(4);
        let pman = match str::from_utf8(pman) {
            Ok(s) => s,
            Err(_) => {
                return Err(BinaryError::InvalidSection {
                    section: "HEADER - PMAN",
                    offset: pman_offset,
                })
            }
        };
        if pman != "PMAN" {
            return Err(BinaryError::InvalidFormat {
                section: "HEADER - PMAN",
                offset: pman_offset,
                expected: Box::new("PMAN"),
                actual: Box::new(pman.to_owned()),
            });
        }

        // Number of files
        let num_files = u32::from_le_bytes((*read_part(4).1).try_into().unwrap());

        // Copyright
        let (copyright_offset, copyright) = read_part(56);
        let copyright = match str::from_utf8(copyright) {
            Ok(s) => s,
            Err(_) => {
                return Err(BinaryError::InvalidSection {
                    section: "HEADER - Copyright",
                    offset: copyright_offset,
                })
            }
        };

        Ok(Self {
            num_files,
            copyright: copyright.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct PmanFileDeclaration {
    // pub start: u32
    pub offset: u32,
    pub size: u32,
    // pub end: u32
}
impl BinaryChunk for PmanFileDeclaration {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError>
    where Self: Sized {
        let mut read_part = |size| (offset.clone(), read_part(buffer, offset, size));

        // Start
        let (start_offset, start) = read_part(4);
        let start = u32::from_le_bytes(start.try_into().unwrap());
        if start != 0 {
            return Err(BinaryError::InvalidFormat {
                section: "FILE DECLARATION - Start",
                offset: start_offset,
                expected: Box::new(0),
                actual: Box::new(start)
            })
        }

        // Offset
        let offset = u32::from_le_bytes((*read_part(4).1).try_into().unwrap());

        // Size
        let size = u32::from_le_bytes((*read_part(4).1).try_into().unwrap());

        // End
        let (end_offset, end) = read_part(4);
        let end = u32::from_le_bytes(end.try_into().unwrap());
        if end != 0 {
            return Err(BinaryError::InvalidFormat {
                section: "FILE DECLARATION - Start",
                offset: end_offset,
                expected: Box::new(0),
                actual: Box::new(start)
            })
        }

        Ok(Self {
            offset,
            size
        })
    }
}

#[derive(Debug)]
pub struct PmanFileData {
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct PmanFile {
    pub header: PmanHeader,
    pub file_declarations: Vec<PmanFileDeclaration>,
    pub files: Vec<PmanFileData>
}

impl BinaryChunk for PmanFile {
    fn new_read(buffer: &[u8], offset: &mut usize) -> Result<Self, BinaryError>
    where Self: Sized {
        // Header
        let header = PmanHeader::new_read(buffer, offset)?;

        // File declarations
        let file_declarations =
            (0 .. header.num_files)
                .map(|_| PmanFileDeclaration::new_read(buffer, offset))
                .collect::<Result<Vec<_>, _>>()?;

        // Files
        let files: Vec<PmanFileData> =
            file_declarations.iter()
                .map(|d| PmanFileData { data: buffer[d.offset as usize .. (d.offset + d.size) as usize].to_vec() })
                .collect();


        Ok(Self {
            header,
            file_declarations,
            files
        })
    }
}

