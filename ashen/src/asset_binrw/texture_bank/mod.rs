use std::io::{Cursor, Read, Seek, SeekFrom};

use super::utils::*;

#[binrw]
#[derive(Debug, Clone)]
pub struct TextureInfo {
    width: u16,
    height: u16,
    offset: PosMarker<u32>,
    size_compressed: PosMarker<u32>,
    size_decompressed: u32,
    animation_frames: u32,
    next_animation_texture_id: PosMarker<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct TextureInfoBank(#[br(parse_with = until_eof)] Vec<TextureInfo>);

#[binrw]
#[br(import { height: usize, width: usize })]
#[derive(Debug)]
pub struct TextureMip(
    #[br(
        parse_with = args_iter((1..=4).map(|s| -> TextureBinReadArgs { args! { width: width / s, height: height / s }})),
        map = |x: Vec<Texture>| x.try_into().unwrap()
    )]
    [Texture; 4],
);

#[binrw::parser(reader, endian)]
fn world_texture_parse(header: TextureInfo) -> BinResult<TextureMip> {
    reader.seek(SeekFrom::Start(header.offset.value as u64));
    let a = <Compressed<TextureMip>>::read_options(
        reader,
        endian,
        args! { height: header.height as usize, width: header.width as usize },
    );

    dbg!(a);
    todo!()
}

#[binread]
#[br(import_raw(info: TextureInfo))]
pub enum WorldTexture {
    Static(#[br(parse_with = world_texture_parse, args(info))] TextureMip),
    //Animated(Vec<WorldTextureMip>),
}

#[binread]
#[br(import_raw(info_bank: TextureInfoBank))]
#[bw(import_raw(info_bank: &TextureInfoBank))]
pub struct WorldTextureBank(
    #[br(parse_with = args_iter(info_bank.0))]
    #[bw(write_with = args_iter_write(&_entries))]
    Vec<WorldTexture>,
);

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const TEXTURE_INFO_DATA: LazyCell<Vec<u8>> = deflated_file!("93.dat");
    const TEXTURE_DATA: LazyCell<Vec<u8>> = deflated_file!("95.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let info_bank = TextureInfoBank::read_le(&mut Cursor::new(TEXTURE_INFO_DATA.as_slice()))?;
        let texture_bank =
            WorldTextureBank::read_le_args(&mut Cursor::new(TEXTURE_DATA.as_slice()), info_bank)?;
        Ok(())
    }
}
