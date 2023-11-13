use super::{Asset, Extension, Kind};
use std::mem;

const ROWS_COUNT: usize = 256;
const COLS_COUNT: usize = 101;

#[derive(Clone, Debug)]
pub struct GammaTable {
    pub lookups: Box<[[u8; ROWS_COUNT]; COLS_COUNT]>,
}

impl Asset for GammaTable {
    fn kind() -> Kind {
        Kind::GammaTable
    }

    fn parse(bytes: &[u8], extension: Extension) -> Self {
        assert!(bytes.len() == ROWS_COUNT * COLS_COUNT);
        assert!(extension == Extension::Dat);

        // SAFETY: we already check that bytes are the same length, and slices and arrays are
        // guaranteed to have the same memory layout.
        let lookups = unsafe { mem::transmute_copy::<_, &[[u8; ROWS_COUNT]; COLS_COUNT]>(&bytes) };

        Self {
            lookups: Box::new(*lookups),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let bytes: Vec<_> = (0..ROWS_COUNT)
            .flat_map(|_| vec![0u8; COLS_COUNT])
            .collect();
        GammaTable::parse(&bytes, Default::default());
    }
}
