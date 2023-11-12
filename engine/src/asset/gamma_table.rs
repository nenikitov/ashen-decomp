use super::{Asset, Kind};

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

    fn parse(bytes: &[u8], _extension: &str) -> Self {
        assert!(bytes.len() == ROWS_COUNT * COLS_COUNT);

        // SAFETY: we already check that bytes are the same length, and slices and arrays are
        // guaranteed to have the same memory layout.
        let lookups =
            unsafe { std::mem::transmute_copy::<_, [[u8; ROWS_COUNT]; COLS_COUNT]>(&bytes) };

        Self {
            lookups: Box::new(lookups),
        }
    }
}
