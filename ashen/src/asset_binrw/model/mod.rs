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
    frames: Marker<u32>,
    offset: Marker<u32>,
}

#[binrw]
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
        -1.0 * (pos * scale + scale_origin) / UNITS_PER_METER
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
            x: I16F16::from_bits(x).to_num(),
            y:I16F16::from_bits(y).to_num(),
            z: I16F16::from_bits(z).to_num(),
        }
    )]
    _scale: Vec3,

    #[br(
        temp,
        map = |[x, y, z]: [i32; 3]| Vec3 {
            x: I16F16::from_bits(x).to_num(),
            y: I16F16::from_bits(y).to_num(),
            z: I16F16::from_bits(z).to_num(),
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

#[binrw]
#[derive(Debug)]
pub struct Model {
    #[br(temp)]
    #[bw(calc = triangles.len() as u32)]
    _triangles_len: u32,

    #[br(temp)]
    #[bw(calc = frames.get(0).map(|f| f.0.vertices.len()).unwrap_or_default() as u32)]
    _vertices_len: u32,

    #[br(temp)]
    #[bw(calc = texture.width() as u32)]
    _texture_width: u32,

    #[br(temp)]
    #[bw(calc = texture.height() as u32)]
    _texture_height: u32,

    #[br(temp)]
    #[bw(calc = frames.len() as u32)]
    _frames_len: u32,

    #[br(temp)]
    #[bw(ignore)]
    _frames_stride: u32,

    #[br(temp)]
    #[bw(calc = sequences.len() as u32)]
    _sequences_len: u32,

    #[br(temp)]
    #[bw(calc = Default::default())]
    _texture_offset: Marker<u32>,

    #[br(temp)]
    #[bw(calc = Default::default())]
    _triangles_offset: Marker<u32>,

    #[br(temp)]
    #[bw(calc = Default::default())]
    _frames_offset: Marker<u32>,

    #[br(temp)]
    #[bw(calc = Default::default())]
    _sequences_offset: Marker<u32>,

    locator_nodes: [u8; 0x10],

    #[br(
        args { count: _sequences_len as usize },
        seek_before = SeekFrom::Start(_sequences_offset.value as u64),
        temp,
    )]
    #[bw(ignore)]
    _sequences: Vec<ModelSequenceHeader>,

    #[br(parse_with = args_iter(_sequences))]
    #[bw(map = |s| s.store_offset(&_sequences_offset))]
    sequences: Vec<ModelSequence>,

    #[br(
        args {
            height: _texture_height as usize,
            width: _texture_width as usize
        },
        seek_before = SeekFrom::Start(_texture_offset.value as u64)
    )]
    #[bw(ignore)]
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
    #[bw(ignore)]
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
    #[bw(ignore)]
    frames: Vec<ModelFramePadded>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor, iter};

    use super::*;
    use crate::{asset::Parser, utils::test::*};

    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("0E-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let model = Model::read_le(&mut Cursor::new(MODEL_DATA.as_slice()))?;
        let (_, model_old) = crate::asset::model::Model::parser(())(&MODEL_DATA)?;

        for (n, o) in iter::zip(model.frames, model_old.frames) {
            assert_eq!(n.0.bounding_sphere_radius, o.bounding_sphere_radius);
            assert_eq!(n.0.triangle_normal_indices, o.triangle_normal_indexes);

            for (n, o) in iter::zip(n.0.vertices, o.vertices) {
                assert_approx_eq::assert_approx_eq!(
                    n.pos.distance(Vec3 { x: o.x, y: o.y, z: o.z }),
                    0.0,
                    0.015
                );
            }
        }

        Ok(())
    }
}
