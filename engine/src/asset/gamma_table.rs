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
    fn kind() -> Kind {
        Kind::GammaTable
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        if extension != Extension::Dat {
            return Err(error::ParseError::unsupported_extension(input, extension).into());
        }

        // NOTE: These checks depend on which `extension` is being parsed.
        error::ensure_bytes_length(
            input,
            GAMMA_TABLE_LENGTH,
            "Incorrect `GammaTable` format (256x101 array of u8s)",
        )?;

        // Technically this can't never fail.
        let (input, bytes) = bytes::take(GAMMA_TABLE_LENGTH)(input)?;

        // SAFETY: bytes::take() should return at least `ROWS_COUNT * COLS_COUNT`
        // bytes; also slices and arrays are guaranteed to have the same memory
        // layout.
        let lookups = unsafe { mem::transmute_copy::<_, &[[u8; ROWS_COUNT]; COLS_COUNT]>(&bytes) };

        Ok((
            input,
            Self {
                lookups: Box::new(*lookups),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() -> eyre::Result<()> {
        let bytes: Vec<_> = (0..ROWS_COUNT)
            .flat_map(|_| vec![0u8; COLS_COUNT])
            .collect();

        GammaTable::parse(&bytes, Default::default())?;

        Ok(())
    }
}
