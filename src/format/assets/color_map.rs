use crate::format::{asset_table::AssetType, *};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new_from_12_bit(value: u16) -> Result<Color, DataError> {
        if value > 0xFFF {
            return Err(DataError {
                file_type: None,
                section: None,
                offset: None,
                actual: Box::from(value),
                expected: ExpectedData::Bound {
                    min: None,
                    max: Some(Box::from(0xFFF)),
                },
            });
        }

        let r = (value & 0xF00) >> 8;
        let g = (value & 0x0F0) >> 4;
        let b = value & 0x00F;

        let r = (r | r << 4) as u8;
        let g = (g | g << 4) as u8;
        let b = (b | b << 4) as u8;

        Ok(Color { r, g, b })
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
            .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
            .enumerate()
            .map(|(i, value)| {
                Color::new_from_12_bit(value).map_err(|mut e| {
                    e.offset = Some(i * 4);
                    e
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|mut e| {
                e.file_type = Some(Self::file_type());
                e
            })?;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_new_from_12_bit_assigns_rgb() {
        let color = Color::new_from_12_bit(0x57D).unwrap();
        assert_eq!(0x55, color.r);
        assert_eq!(0x77, color.g);
        assert_eq!(0xDD, color.b);
    }

    #[test]
    fn test_color_new_from_12_bit_returns_error_if_value_is_not_12_bit() {
        let color = Color::new_from_12_bit(0xFFF1).unwrap_err();
        assert_eq!(
            DataError {
                file_type: None,
                section: None,
                offset: None,
                actual: Box::from(0xFFF1),
                expected: ExpectedData::Bound {
                    min: None,
                    max: Some(Box::from(0xFFF))
                }
            },
            color
        );
    }
}
