use crate::format::{asset_table::AssetType, *};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const MAX: u16 = 0xFFF;

    fn new_from_12_bit(value: u16) -> Result<Color, DataError> {
        if value > 0xFFF {
            return Err(DataError {
                asset_type: None,
                section: None,
                offset: None,
                actual: Box::new(value),
                expected: ExpectedData::Bound {
                    min: None,
                    max: Some(Box::new(Self::MAX)),
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

#[derive(Debug, PartialEq)]
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
                asset_type: Some(Self::file_type()),
                section: None,
                offset: Some(0),
                actual: Box::new(bytes.len()),
                expected: ExpectedData::Other {
                    description: "Length divisible by 4 (because each color is 4 bytes)"
                        .to_string(),
                },
            });
        }

        let colors: Vec<_> = bytes
            .array_chunks::<4>()
            .map(|bytes| u32::from_le_bytes(*bytes))
            .enumerate()
            .map(|(i, value)| {
                let value_16 = value.min(u16::MAX as u32) as u16;
                Color::new_from_12_bit(value_16).map_err(|mut e| {
                    e.offset = Some(i * 4);
                    e.actual = Box::new(value);
                    e
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|mut e| {
                e.asset_type = Some(Self::file_type());
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

    mod color {
        use super::*;

        mod new_from_12_bit {
            use super::*;

            #[test]
            fn returns_parsed_color() {
                let color = Color::new_from_12_bit(0x57D).unwrap();
                assert_eq!(
                    color,
                    Color {
                        r: 0x55,
                        g: 0x77,
                        b: 0xDD
                    }
                );
            }

            #[test]
            fn returns_error_if_value_is_not_12_bit() {
                let color = Color::new_from_12_bit(0xFFF1).unwrap_err();
                assert_eq!(
                    color,
                    DataError {
                        asset_type: None,
                        section: None,
                        offset: None,
                        actual: Box::new(0xFFF1),
                        expected: ExpectedData::Bound {
                            min: None,
                            max: Some(Box::new(0xFFF))
                        }
                    }
                );
            }
        }
    }

    mod color_map {
        use super::*;

        mod asset_load {
            use super::*;

            mod load {
                use super::*;

                #[test]
                fn returns_parsed_color_map() {
                    let bytes = [
                        0xBC, 0x0B, 0x00, 0x00, // #BBBBCC
                        0x54, 0x06, 0x00, 0x00, // #665544
                        0x13, 0x0F, 0x00, 0x00, // #FF1133
                        0x06, 0x07, 0x00, 0x00, // #770066
                    ];
                    let color_map = ColorMap::load(&bytes).unwrap();

                    assert_eq!(
                        color_map.0,
                        ColorMap {
                            colors: vec![
                                Color::new_from_12_bit(0xBBC).unwrap(),
                                Color::new_from_12_bit(0x654).unwrap(),
                                Color::new_from_12_bit(0xF13).unwrap(),
                                Color::new_from_12_bit(0x706).unwrap()
                            ]
                        }
                    );
                    assert_eq!(color_map.1, 16);
                }

                #[test]
                fn returns_error_if_data_is_not_in_chunks_of_4_bytes() {
                    let bytes = [
                        0xBC, 0x0B, 0x00, 0x00, // #BBBBCC
                        0x54, 0x06, 0x00, 0x00, // #665544
                        0x13, 0x0F, 0x00, 0x00, // #FF1133
                        0x06, 0x07, 0x00, // #770066 INVALID
                    ];
                    let color_map = ColorMap::load(&bytes).unwrap_err();

                    assert_eq!(
                        color_map,
                        DataError {
                            asset_type: Some(AssetType::ColorMap),
                            section: None,
                            offset: Some(0),
                            actual: Box::new(15),
                            expected: ExpectedData::Other {
                                description:
                                    "Length divisible by 4 (because each color is 4 bytes)"
                                        .to_string()
                            }
                        }
                    );
                }

                #[test]
                fn returns_error_if_color_is_not_12_bit() {
                    let bytes = [
                        0xBC, 0x0B, 0x00, 0x00, // #BBBBCC
                        0x54, 0x06, 0x00, 0x00, // #665544
                        0x13, 0x0F, 0x00, 0x00, // #FF1133
                        0x06, 0x07, 0x00, 0x01, // #770066 INVALID
                    ];
                    let color_map = ColorMap::load(&bytes).unwrap_err();
                    assert_eq!(
                        color_map,
                        DataError {
                            asset_type: Some(AssetType::ColorMap),
                            section: None,
                            offset: Some(12),
                            actual: Box::new(0x1000706),
                            expected: ExpectedData::Bound {
                                min: None,
                                max: Some(Box::new(0xFFF))
                            }
                        }
                    );
                }
            }

            mod asset_type {
                use super::*;

                #[test]
                fn returns_correctly() {
                    assert_eq!(ColorMap::file_type(), AssetType::ColorMap)
                }
            }
        }
    }
}
