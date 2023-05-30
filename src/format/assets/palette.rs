use crate::format::*;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new_from_12_bit(value: u16) -> Color {
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
    type Data = ();

    fn load(bytes: &[u8], _: Self::Data) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        if bytes.len() % 4 != 0 {
            Err(DataError {
                file_type: Some("Palette".to_string()),
                section: None,
                offset: None,
                actual: Box::new(bytes.len()),
                expected: ExpectedData::Other {
                    description: "Divisible by 4 (because each color is 4 bytes)".to_string(),
                },
            })
        } else {
            let size = bytes.len() / 4;
            todo!()
        }
    }

    fn file_type() -> String {
        "Color map".to_string()
    }
}

impl AssetConvert for ColorMap {
    fn convert(&self) -> Vec<ConvertedFile> {
        todo!()
    }
}
