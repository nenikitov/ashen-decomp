use std::io::Read;

use flate2::bufread::ZlibDecoder;

// TODO(nenikitov): Return `Result`
pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    match bytes {
        [b'Z', b'L', s1, s2, s3, bytes @ ..] => {
            let size = u32::from_le_bytes([*s1, *s2, *s3, 0]);
            let mut decoder = ZlibDecoder::new(bytes);
            let mut data = Vec::with_capacity(size as usize);
            decoder
                .read_to_end(&mut data)
                .expect("Data should be a valid zlib stream");

            // TODO(nenikitov): Check if `data.len() == size`

            data
        }
        _ => bytes.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_zlib_works() {
        let data = [
            b'Z', b'L', // Asset Zlib signature
            0x06, 0x00, 0x00, // Stream size
            0x78, 0xDA, // Actual Zlib signature
            0x73, 0x2C, 0xCE, 0x48, 0xCD, 0xE3, 0x02, 0x00, 0x07, 0x80, 0x01, 0xFA,
        ];

        assert_eq!("Ashen\n".bytes().collect::<Vec<u8>>(), decompress(&data))
    }
}
