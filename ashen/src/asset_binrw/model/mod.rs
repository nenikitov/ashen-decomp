use glam::Vec2;

use super::utils::*;

#[binrw::parser(reader, endian)]
fn parse_uv(args: TextureReadArgs, ...) -> BinResult<Vec2> {
    let u = u16::read_options(reader, endian, ())?;
    let v = u16::read_options(reader, endian, ())?;

    Ok(Vec2::new(
        u as f32 / args.width as f32,
        1f32 - (v as f32 / args.height as f32),
    ))
}

#[binread]
#[br(import_raw(args: TextureReadArgs))]
#[derive(Debug)]
pub struct ModelPoint {
    #[br(map = |x: u16| x.into())]
    index: usize,
    #[br(
        parse_with = parse_uv,
        args_raw(args)
    )]
    uv: Vec2,
}

#[binread]
#[br(import_raw(args: TextureReadArgs))]
#[derive(Debug)]
pub struct ModelTriangle(
    #[br(
        parse_with = args_iter(vec![args; 3]),
        map = |x: Vec<ModelPoint>| x.try_into().unwrap()
    )]
    [ModelPoint; 3]
);

#[binread]
#[derive(Debug)]
pub struct Model {
    _triangles_len: u32,
    _vertices_len: u32,
    _texture_width: u32,
    _texture_height: u32,
    _frames_len: u32,
    _frames_stride: u32,
    _sequences_len: u32,
    _texture_offset: PosMarker<u32>,
    _triangles_offset: PosMarker<u32>,
    _frames_offset: PosMarker<u32>,
    _sequences_offset: PosMarker<u32>,
    locator_nodes: [u8; 0x10],

    #[br(args {
        count: _triangles_len as usize,
        inner: args! {
            width: _texture_width as usize,
            height: _texture_height as usize
        }
    })]
    triangles: Vec<ModelTriangle>,

    #[br(args { height: _texture_height as usize, width: _texture_width as usize })]
    texture: Texture
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

        dbg!(model.triangles);

        Ok(())
    }
}
