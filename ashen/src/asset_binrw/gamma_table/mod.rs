use super::utils::*;

const LEN_ROWS: usize = 256;
const LEN_COLS: usize = 101;

#[binrw]
#[derive(Debug)]
pub struct GammaTable {
    #[br(
        args { count: LEN_ROWS, inner: binrw::args! { count: LEN_COLS, inner: () }},
        parse_with = map_vec2_parse(<NormalizedF32<u8>>::into)
    )]
    #[bw(
        assert(gamma.len() == LEN_ROWS && gamma[0].len() == LEN_COLS),
        write_with = map_vec2_write(|&x| <NormalizedF32<u8>>::from(x))
    )]
    pub gamma: Vec<Vec<f32>>,
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
        let gamma_table = GammaTable::read_le(&mut Cursor::new(GAMMA_TABLE_DATA.as_slice()))?;
        Ok(())
    }
}
