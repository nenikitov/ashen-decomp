use glam::Vec3;

use super::utils::*;

const LEN_PALETTE: usize = 256;

#[binrw]
#[derive(Debug)]
pub struct Skybox {
    #[br(temp)]
    #[bw(calc = texture.width() as u32)]
    _texture_width: u32,

    #[br(temp)]
    #[bw(calc = texture.height() as u32)]
    _texture_height: u32,

    #[br(
        count = LEN_PALETTE,
        parse_with = map_vec_parse(ColorU16::into)
    )]
    #[bw(
        assert(palette.len() == LEN_PALETTE),
        write_with = map_vec_write(|&x| (ColorU16::from(x), 0u16))
    )]
    palette: Vec<Vec3>,

    #[br(args { width: _texture_width as usize, height: _texture_height as usize })]
    texture: Texture,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const SKYBOX_DATA: LazyCell<Vec<u8>> = deflated_file!("3C.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let skybox = Skybox::read_le(&mut Cursor::new(SKYBOX_DATA.as_slice()))?;
        Ok(())
    }
}
