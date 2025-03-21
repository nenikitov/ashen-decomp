use crate::utils::binrw::*;

const LEN_ROWS: usize = 256;
const LEN_COLS: usize = 101;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct GammaTable {
    #[br(args {
        count: LEN_ROWS,
        inner: binrw::args! { count: LEN_COLS, inner: () },
    })]
    #[bw()]
    pub gamma: Vec<Vec<NormalizedF32<u8>>>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const GAMMA_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("00.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let gamma_table = GammaTable::read(&mut Cursor::new(GAMMA_TABLE_DATA.as_slice()))?;

        dbg!(gamma_table);

        Ok(())
    }
}
