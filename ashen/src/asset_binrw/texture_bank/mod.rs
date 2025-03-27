use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};

use super::utils::*;

#[binrw]
#[derive(Debug, Clone)]
pub struct WorldTextureInfo {
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
pub struct WorldTextureInfoBank(#[br(parse_with = until_eof)] Vec<WorldTextureInfo>);

#[binrw]
#[br(import { height: usize, width: usize })]
#[derive(Debug)]
pub struct WorldTexture(
    #[br(
        parse_with = args_iter(
            (1..=4)
                .map(|s| 2usize.pow(s))
                .map(|s| -> TextureBinReadArgs { args! { width: width / s, height: height / s } })
        ),
        map = |x: Vec<Texture>| x.try_into().unwrap()
    )]
    [Texture; 4],
);

pub enum WorldTextureKind {
    Static(Rc<WorldTexture>),
    Animated(Vec<Rc<WorldTexture>>),
}

#[binrw::parser(reader, endian)]
fn world_texture_parser<'a>(
    info: &'a WorldTextureInfo,
    ...
) -> BinResult<(&'a WorldTextureInfo, Rc<WorldTexture>)> {
    reader.seek(SeekFrom::Start(info.offset.value as u64))?;
    let texture = <Compressed<WorldTexture>>::read_options(
        reader,
        endian,
        args! { height: info.height as usize, width: info.width as usize },
    )?;
    Ok((info, Rc::new(texture.into_inner())))
}

#[binread]
#[br(import_raw(info_bank: &WorldTextureInfoBank))]
pub struct WorldTextureBank(
    #[br(
        parse_with = args_iter_with(&info_bank.0, world_texture_parser),
        map = |bank: Vec<(&WorldTextureInfo, Rc<WorldTexture>)>| {
            bank
                .iter()
                .enumerate()
                .map(|(i, (info, texture))| {
                    match info.animation_frames {
                        0 => WorldTextureKind::Static(texture.clone()),
                        _ => {
                            let frames = (0..info.animation_frames)
                                .scan(i, |i, _| {
                                    let (info, texture) = &bank[*i];
                                    *i = info.next_animation_texture_id.value as usize;
                                    Some(texture.clone())
                                })
                                .collect();
                            WorldTextureKind::Animated(frames)
                        },
                    }
                })
                .collect()
        }
    )]
    Vec<WorldTextureKind>,
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
        let info_bank =
            WorldTextureInfoBank::read_le(&mut Cursor::new(TEXTURE_INFO_DATA.as_slice()))?;
        let texture_bank =
            WorldTextureBank::read_le_args(&mut Cursor::new(TEXTURE_DATA.as_slice()), &info_bank)?;
        Ok(())
    }
}
