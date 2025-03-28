use std::{
    io::{Read, Seek, SeekFrom},
};

use ouroboros::self_referencing;

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

pub struct WorldTextureAnimated<'a> {
    texture: &'a WorldTexture,
    next_texture: Option<&'a WorldTexture>,
}

pub struct WorldTextureBank {
    bank: Vec<WorldTexture>,
}

impl BinRead for WorldTextureBank {
    type Args<'a> = &'a WorldTextureInfoBank;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        info_bank: Self::Args<'_>,
    ) -> BinResult<Self> {
        let bank = info_bank
            .0
            .iter()
            .map(|i| {
                reader.seek(SeekFrom::Start(i.offset.value as u64))?;
                <Compressed<WorldTexture>>::read_options(
                    reader,
                    endian,
                    args! { height: i.height as usize, width: i.width as usize },
                )
                .map(Compressed::into_inner)
            })
            .collect::<BinResult<_>>()?;

        Ok(Self { bank })
    }
}

// map = |bank: Vec<(&WorldTextureInfo, WorldTexture<'a>)>| {
//     todo!()
// bank
//     .iter()
//     .enumerate()
//     .map(|(i, (info, texture))| {
//         match info.animation_frames {
//             0 => WorldTextureKind::Static(texture.clone()),
//             _ => {
//                 let frames = (0..info.animation_frames)
//                     .scan(i, |i, _| {
//                         let (info, texture) = &bank[*i];
//                         *i = info.next_animation_texture_id.value as usize;
//                         Some(texture.clone())
//                     })
//                     .collect();
//                 WorldTextureKind::Animated(frames)
//             },
//         }
//     })
//     .collect()
// }

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
