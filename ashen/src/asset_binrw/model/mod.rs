use super::utils::*;

#[binrw]
#[derive(Debug)]
pub struct Model {
    _triangles_len: u32,
    _vertices_len: u32,
    _texture_width: u32,
    _texture_height: u32,
    _frames_len: u32,
    _frames_stride: u32,
    _sequences_len: u32,
    _texture_offset: u32,
    _triangles_offset: u32,
    _frames_offset: u32,
    _sequences_offset: u32,
    locator_nodes: [u8; 0x10],
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("0E-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let model = Model::read_le(&mut Cursor::new(MODEL_DATA.as_slice()))?;

        dbg!(model);

        Ok(())
    }
}
