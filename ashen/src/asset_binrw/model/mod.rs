use std::io::SeekFrom;

use fixed::{
    traits::LossyInto,
    types::{I16F16, I24F8},
};
use glam::{Vec2, Vec3};

use super::utils::*;

const UNITS_PER_METER: f32 = 32.0;

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
    [ModelPoint; 3],
);

#[binread]
#[derive(Clone, Debug)]
pub struct ModelSequenceHeader {
    frames: PosMarker<u32>,
    offset: PosMarker<u32>,
}

#[binread]
#[br(import_raw(header: ModelSequenceHeader))]
#[derive(Debug)]
pub struct ModelSequence(
    #[br(
        seek_before = SeekFrom::Start(header.offset.value as u64),
        count = header.frames.value as usize
    )]
    pub Vec<u32>,
);

#[binread]
#[br(import {
    scale: Vec3,
    scale_origin: Vec3,
})]
#[derive(Debug)]
pub struct ModelVertex {
    #[br(map = |[x, y, z]: [NormalizedF32<u8>; 3]| {
        let pos = Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        };
        -((pos * scale) / (u8::MAX as f32) - scale_origin) / UNITS_PER_METER
    })]
    pos: Vec3,
    normal_index: u8,
}

#[binread]
#[br(
    import {
        vertices: usize,
        triangles: usize,
    },
)]
#[derive(Debug)]
pub struct ModelFrame {
    #[br(
        temp,
        map = |[x, y, z]: [i32; 3]| Vec3 {
            x: I16F16::from_bits(x).lossy_into(),
            y: I16F16::from_bits(y).lossy_into(),
            z: I16F16::from_bits(z).lossy_into(),
        }
    )]
    _scale: Vec3,

    #[br(
        temp,
        map = |[x, y, z]: [i32; 3]| Vec3 {
            x: I16F16::from_bits(x).lossy_into(),
            y: I16F16::from_bits(y).lossy_into(),
            z: I16F16::from_bits(z).lossy_into(),
        }
    )]
    _scale_origin: Vec3,

    #[br(map = |x: i32| I24F8::from_bits(x).lossy_into())]
    bounding_sphere_radius: f32,

    #[br(args {
        count: vertices as usize,
        inner: args! {
            scale: _scale,
            scale_origin: _scale_origin,
        }
    })]
    vertices: Vec<ModelVertex>,

    #[br(args { count: triangles as usize })]
    triangle_normal_indices: Vec<u8>,
}

#[binread]
#[br(
    import {
        vertices: usize,
        triangles: usize,
        stride: usize,
    },
)]
#[derive(Debug)]
pub struct ModelFramePadded(#[br(args { vertices, triangles }, pad_size_to = stride)] ModelFrame);

#[binread]
#[derive(Debug)]
pub struct Model {
    #[br(temp)]
    _triangles_len: u32,
    #[br(temp)]
    _vertices_len: u32,
    #[br(temp)]
    _texture_width: u32,
    #[br(temp)]
    _texture_height: u32,
    #[br(temp)]
    _frames_len: u32,
    #[br(temp)]
    _frames_stride: u32,
    #[br(temp)]
    _sequences_len: u32,
    #[br(temp)]
    _texture_offset: PosMarker<u32>,
    #[br(temp)]
    _triangles_offset: PosMarker<u32>,
    #[br(temp)]
    _frames_offset: PosMarker<u32>,
    #[br(temp)]
    _sequences_offset: PosMarker<u32>,

    locator_nodes: [u8; 0x10],

    #[br(
        args { count: _sequences_len as usize },
        seek_before = SeekFrom::Start(_sequences_offset.value as u64),
        temp,
    )]
    _sequences: Vec<ModelSequenceHeader>,

    #[br(parse_with = args_iter(_sequences))]
    sequences: Vec<ModelSequence>,

    #[br(
        args {
            height: _texture_height as usize,
            width: _texture_width as usize
        },
        seek_before = SeekFrom::Start(_texture_offset.value as u64)
    )]
    texture: Texture,

    #[br(
        args {
            count: _triangles_len as usize,
            inner: args! {
                width: _texture_width as usize,
                height: _texture_height as usize
            }
        },
        seek_before = SeekFrom::Start(_triangles_offset.value as u64)
    )]
    triangles: Vec<ModelTriangle>,

    #[br(
        args {
            count: _frames_len as usize,
            inner: args! {
                vertices: _vertices_len as usize,
                triangles: _triangles_len as usize,
                stride: _frames_stride as usize,
            }
        },
        seek_before = SeekFrom::Start(_frames_offset.value as u64)
    )]
    frames: Vec<ModelFramePadded>,
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

        dbg!(model.frames);

        Ok(())
    }
}
