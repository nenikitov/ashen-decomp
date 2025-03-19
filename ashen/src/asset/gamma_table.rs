use super::Parser;
use crate::{error, utils::nom::*};

const ROWS_COUNT: usize = 256;
const COLS_COUNT: usize = 101;
const GAMMA_TABLE_LENGTH: usize = ROWS_COUNT * COLS_COUNT;

#[derive(Clone, Debug)]
pub struct GammaTable {
    pub lookups: Box<[[u8; ROWS_COUNT]; COLS_COUNT]>,
}

impl Parser for GammaTable {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            error::ensure_bytes_length(
                input,
                GAMMA_TABLE_LENGTH,
                "Incorrect `GammaTable` format (256x101 array of u8s)",
            )?;

            // Technically this can't never fail.
            let (input, bytes) = bytes::take(GAMMA_TABLE_LENGTH)(input)?;

            let lookups = bytes.as_ptr() as *const [[_; ROWS_COUNT]; COLS_COUNT];
            // SAFETY: bytes::take() should return exactly `ROWS_COUNT * COLS_COUNT`
            // bytes (GAMMA_TABLE_LENGTH).
            let lookups = unsafe { *lookups };

            Ok((
                input,
                Self {
                    lookups: Box::new(lookups),
                },
            ))
        }
    }
}

impl GammaTable {
    // TODO(Unavailable): take a `std::io::Write` instead of a `Path`?
    #[cfg(feature = "conv")]
    pub fn to_png<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        use crate::{asset::color_map::Color, utils::format::PngFile};

        let bytes = self
            .lookups
            .to_vec()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|color| Color {
                        r: color,
                        g: color,
                        b: color,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .to_png();

        std::fs::write(path, bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::{
        asset::color_map::Color,
        utils::{format::*, test::*},
    };

    const GAMMA_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("00.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, gamma_table) = GammaTable::parser(())(&GAMMA_TABLE_DATA)?;

        let gamma_table = gamma_table
            .lookups
            .to_vec()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|color| Color {
                        r: color,
                        g: color,
                        b: color,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        output_file(parsed_file_path!("gamma-table.png"), gamma_table.to_png())?;

        Ok(())
    }
}
