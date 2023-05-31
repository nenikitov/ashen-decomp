use crate::format::{asset_table::AssetType, *};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new_from_12_bit(value: u32) -> Color {
        let r = (value & 0xF00) >> 8;
        let g = (value & 0x0F0) >> 4;
        let b = value & 0x00F;

        let r = (r | r << 4) as u8;
        let g = (g | g << 4) as u8;
        let b = (b | b << 4) as u8;

        Color { r, g, b }
    }
}

#[derive(Debug)]
pub struct ColorMap {
    colors: Vec<Color>,
}

impl AssetLoad for ColorMap {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        let size = bytes.len();
        if size % 4 != 0 {
            return Err(DataError {
                file_type: Some(Self::file_type()),
                section: None,
                offset: None,
                actual: Box::new(bytes.len()),
                expected: ExpectedData::Other {
                    description: "Divisible by 4 (because each color is 4 bytes)".to_string(),
                },
            });
        }

        let colors: Vec<_> = bytes
            .chunks(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
            .map(Color::new_from_12_bit)
            .collect();

        Ok((Self { colors }, size))
    }

    fn file_type() -> AssetType {
        AssetType::ColorMap
    }
}

impl AssetConvert for ColorMap {
    type Extra = ();

    fn convert(&self, _: &Self::Extra) -> Vec<ConvertedFile> {
        todo!()
    }
}
