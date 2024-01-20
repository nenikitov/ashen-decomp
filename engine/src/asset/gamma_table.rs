use super::{extension::*, AssetParser};
use crate::{error, utils::nom::*};
use std::mem;

const ROWS_COUNT: usize = 256;
const COLS_COUNT: usize = 101;
const GAMMA_TABLE_LENGTH: usize = ROWS_COUNT * COLS_COUNT;

#[derive(Clone, Debug)]
pub struct GammaTable {
    pub lookups: Box<[[u8; ROWS_COUNT]; COLS_COUNT]>,
}

impl AssetParser<Pack> for GammaTable {
    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            error::ensure_bytes_length(
                input,
                GAMMA_TABLE_LENGTH,
                "Incorrect `GammaTable` format (256x101 array of u8s)",
            )?;

            // Technically this can't never fail.
            let (input, bytes) = bytes::take(GAMMA_TABLE_LENGTH)(input)?;

            // SAFETY(Unavailable): Dont transmute references!!!!!!!
            //
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
        let (_, gamma_table) = <GammaTable as AssetParser<Pack>>::parser(())(&GAMMA_TABLE_DATA)?;

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
