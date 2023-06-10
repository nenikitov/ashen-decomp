use std::{iter::repeat, mem::transmute};

use crate::format::{asset_table::AssetType, *};

type GammaTableMatrix = [[u8; 256]; 101];

#[derive(Debug, PartialEq)]
pub struct GammaTable {
    masks: GammaTableMatrix,
}

impl GammaTable {
    const SIZE: usize = 256 * 101;
}

impl AssetLoad for GammaTable {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        if bytes.len() < 256 * 101 {
            return Err(DataError {
                asset_type: Some(Self::file_type()),
                section: None,
                offset: Some(0),
                actual: Box::new(bytes.len()),
                expected: ExpectedData::Other {
                    description: "Length is not equal to 101 layers of 256 gamma look up values"
                        .to_string(),
                },
            });
        }

        // SAFETY: We already verified that `bytes` are of correct size to transmute into a 256 * 101 array
        let masks = unsafe { transmute::<[u8; Self::SIZE], _>(bytes.try_into().unwrap()) };

        Ok((Self { masks }, Self::SIZE))
    }

    fn file_type() -> AssetType
    where
        Self: Sized,
    {
        AssetType::GammaTable
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod gamma_table {
        use super::*;

        mod asset_load {
            use super::*;

            mod load {
                use std::iter::repeat_with;

                use rand::seq::IteratorRandom;

                use super::*;

                #[test]
                fn returns_parsed_gamma_table() {
                    let mut rng = rand::thread_rng();
                    let bytes: Vec<_> = repeat_with(|| (0..=255u8).choose_multiple(&mut rng, 256))
                        .take(101)
                        .flatten()
                        .collect();
                    let gamma_table = GammaTable::load(&bytes).unwrap();

                    assert_eq!(
                        gamma_table.0,
                        GammaTable {
                            masks: bytes
                                .array_chunks::<256>()
                                .map(|s| *s)
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        }
                    );
                    assert_eq!(gamma_table.1, 256 * 101);
                }

                #[test]
                fn returns_error_if_length_is_invalid() {
                    let bytes = "Ashen".as_bytes();
                    let gamma_table = GammaTable::load(&bytes).unwrap_err();

                    assert_eq!(
                        gamma_table,
                        DataError {
                            asset_type: Some(AssetType::GammaTable),
                            section: None,
                            offset: Some(0),
                            actual: Box::new(5),
                            expected: ExpectedData::Other {
                                description:
                                    "Length is not equal to 101 layers of 256 gamma look up values"
                                        .to_string()
                            }
                        }
                    );
                }
            }
        }
    }
}
