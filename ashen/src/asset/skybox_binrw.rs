use glam::Vec3;

use crate::utils::binrw::*;

const LEN_PALETTE: usize = 256;

#[binrw]
#[br(import{ height: usize, width: usize })]
#[derive(Debug)]
pub struct Texture(
    #[br(
        args { count: height, inner: binrw::args! { count: width, inner: () }},
    )]
    Vec<Vec<u8>>,
);

impl Texture {
    fn width(&self) -> usize {
        self.0.get(0).map_or(0, |v| v.len())
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Skybox {
    #[br(temp)]
    #[bw(calc = texture.width() as u32)]
    _width: u32,

    #[br(temp)]
    #[bw(calc = texture.height() as u32)]
    _height: u32,

    #[br(
        count = LEN_PALETTE,
        parse_with = map_vec_parse(ColorU16::into)
    )]
    #[bw(
        assert(palette.len() == LEN_PALETTE),
        write_with = map_vec_write(|&x| (ColorU16::from(x), 0u16))
    )]
    palette: Vec<Vec3>,

    #[br(args { width: _width as usize, height: _height as usize })]
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
        let skybox = Skybox::read(&mut Cursor::new(SKYBOX_DATA.as_slice()))?;

        dbg!(skybox);

        Ok(())
    }
}
