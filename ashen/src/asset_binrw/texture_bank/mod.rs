use std::io::{Cursor, Read, Seek, SeekFrom};

use super::utils::*;

#[binrw]
pub struct WorldTextureOffset {
    width: u16,
    height: u16,
    offset: PosMarker<u32>,
    size_compressed: PosMarker<u32>,
    size_decompressed: u32,
    animation_frames: u32,
    next_animation_texture_id: PosMarker<u32>,
}

#[binrw]
#[brw(little)]
pub struct WorldTextureOffsetBank(#[br(parse_with = until_eof)] Vec<WorldTextureOffset>);

#[binrw]
#[br(import { height: usize, width: usize })]
pub struct TextureMip(
    #[br(
        parse_with = args_iter((1..=4).map(|s| -> TextureBinReadArgs { args! { width: width / s, height: height / s }})),
        map = |x: Vec<Texture>| x.try_into().unwrap()
    )]
    [Texture; 4],
);

#[binrw]
#[br(import { height: usize, width: usize })]
pub enum WorldTextureMip {
    NonMipped(#[br(args { height: height, width: width })] Texture),
    Mipped(#[br(args { height: height, width: width })] TextureMip),
}

#[binrw::parser(reader, endian)]
fn world_texture_parse(header: &WorldTextureOffset) -> BinResult<WorldTextureMip> {
    reader.seek(SeekFrom::Start(header.offset.value as u64));
    todo!()
}

#[binread]
#[br(import_raw(header: &WorldTextureOffset))]
pub enum WorldTexture {
    Static(#[br(parse_with = world_texture_parse, args(header))] WorldTextureMip),
    //Animated(Vec<WorldTextureMip>),
}

pub struct WorldTextureBank(Vec<WorldTexture>);
