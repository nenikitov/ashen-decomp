use glam::Vec3;

use super::utils::*;

const LEN_ROWS: usize = 256;
const LEN_COLS: usize = 32;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct ColorMap {
    #[br(
        args { count: LEN_ROWS, inner: binrw::args! { count: LEN_COLS, inner: () }},
        parse_with = map_vec2_parse(|(x, _): (ColorU16, u16)| x.into())
    )]
    #[bw(
        assert(shades.len() == LEN_ROWS && shades[0].len() == LEN_COLS),
        write_with = map_vec2_write(|&x| (ColorU16::from(x), 0u16))
    )]
    shades: Vec<Vec<Vec3>>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("01.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let color_map = ColorMap::read(&mut Cursor::new(COLOR_MAP_DATA.as_slice()))?;
        Ok(())
    }
}
