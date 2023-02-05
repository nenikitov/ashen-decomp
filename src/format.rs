use std::fmt::Debug;
use std::str;
use std::io::Read;
use flate2::read::ZlibDecoder;

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

#[derive(Debug)]
pub enum ZlibDataError {
    NotZlibData,
    InvalidSize {
        expected_size: usize,
        actual_size: usize,
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
pub trait SizedBinaryChunk {
    fn new_read(buffer: &[u8], offset: &mut usize, size: usize) -> Result<Self, BinaryError>
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

impl SizedBinaryChunk for PmanFileData {
    fn new_read(buffer: &[u8], offset: &mut usize, size: usize) -> Result<Self, BinaryError>
    where Self: Sized {
        let offset = *offset;
        Ok(Self {
            data: buffer[offset .. offset + size].to_vec()
        })
    }
}

impl PmanFileData {
    pub fn is_zlib(&self) -> bool {
        return
            self.data[0] == b'Z'
            && self.data[1] == b'L'
            && self.data[5] == 0x78
            && self.data[6] == 0xDA;
    }

    pub fn zlib_data(&self) -> Result<Vec<u8>, ZlibDataError> {
        if !self.is_zlib() {
            Err(ZlibDataError::NotZlibData)
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
                Err(ZlibDataError::InvalidSize { expected_size: size as usize, actual_size: size_calc })
            }
            else {
                Ok(s)
            }
        }
    }
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
        let files =
            file_declarations.iter()
                .map(|d| PmanFileData::new_read(buffer, &mut (d.offset.clone() as usize), d.size.clone() as usize))
                .collect::<Result<Vec<_>,_>>()?;

        Ok(Self {
            header,
            file_declarations,
            files
        })
    }
}

