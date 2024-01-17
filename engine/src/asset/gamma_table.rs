use super::{Asset, Extension, Kind};
use crate::{error, utils::nom::*};
use std::mem;

const ROWS_COUNT: usize = 256;
const COLS_COUNT: usize = 101;
const GAMMA_TABLE_LENGTH: usize = ROWS_COUNT * COLS_COUNT;

#[derive(Clone, Debug)]
pub struct GammaTable {
    pub lookups: Box<[[u8; ROWS_COUNT]; COLS_COUNT]>,
}

impl Asset for GammaTable {
    type Context = ();

    fn kind() -> Kind {
        Kind::GammaTable
    }

    fn parse(input: &[u8], extension: Extension, _: Self::Context) -> Result<Self> {
        match extension {
            Extension::Dat => {
                error::ensure_bytes_length(
                    input,
                    GAMMA_TABLE_LENGTH,
                    "Incorrect `GammaTable` format (256x101 array of u8s)",
                )?;

                // Technically this can't never fail.
                let (input, bytes) = bytes::take(GAMMA_TABLE_LENGTH)(input)?;

                // SAFETY: bytes::take() should return exactly `ROWS_COUNT * COLS_COUNT`
                // bytes; also slices and arrays are guaranteed to have the same memory
                // layout.
                let lookups =
                    unsafe { mem::transmute_copy::<_, &[[u8; ROWS_COUNT]; COLS_COUNT]>(&bytes) };

                Ok((
                    input,
                    Self {
                        lookups: Box::new(*lookups),
                    },
                ))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset::color_map::Color,
        utils::{format::*, test::*},
    };
    use std::cell::LazyCell;

    const GAMMA_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("00.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, gamma_table) = GammaTable::parse(&GAMMA_TABLE_DATA, Extension::Dat, ())?;

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

        output_file(parsed_file_path!("gamma-table.ppm"), gamma_table.to_ppm())?;

        Ok(())
    }
}
