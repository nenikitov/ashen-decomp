use std::io::{Cursor, Read, Seek, SeekFrom};

use flate2::read::ZlibDecoder;

use super::utils::*;

#[binrw]
#[brw(magic = b"ZL")]
struct Zlib {
    #[br(parse_with = read_u24)]
    #[bw(write_with = write_u24)]
    len: u32,
}

fn decompress<Stream: Read + Seek>(stream: &mut Stream) -> BinResult<Cursor<Vec<u8>>> {
    let header = Zlib::read_options(stream, endian, ())?;
    let pos = stream.stream_position();

    let mut decoder = ZlibDecoder::new(stream);
    let mut data = Vec::with_capacity(header.len as usize);
    decoder.read_to_end(&mut data)?;

    if data.len() != header.len as usize {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "zlib decompression resulted into invalid size",
        )));
    }

    Ok(Cursor::new(data))
}

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
