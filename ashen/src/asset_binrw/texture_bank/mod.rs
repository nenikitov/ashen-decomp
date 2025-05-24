use std::io::{Read, Seek, SeekFrom};

use super::utils::*;

#[binrw]
#[derive(Default, Debug, Clone)]
pub struct WorldTextureInfo {
    width: u16,
    height: u16,
    offset: u32,
    size_compressed: u32,
    size_decompressed: u32,
    animation_frames: u32,
    next_animation_texture_id: u32,
}

#[binrw]
#[derive(Default, Debug)]
pub struct WorldTextureInfoBank(#[br(parse_with = until_eof)] Vec<WorldTextureInfo>);

#[derive(Clone, Debug)]
pub struct WorldTextureAnimation {
    frames: usize,
    next_frame: usize,
}

#[binrw]
#[br(import { height: usize, width: usize, animation: Option<WorldTextureAnimation> })]
#[derive(Debug)]
pub struct WorldTexture {
    #[br(
        parse_with = args_iter(
            (0..=3)
                .map(|s| 2usize.pow(s))
                .map(|s| -> TextureReadArgs { args! { width: width / s, height: height / s } })
        ),
        map = |x: Vec<Texture>| x.try_into().unwrap()
    )]
    mips: [Texture; 4],
    #[br(calc = animation)]
    #[bw(ignore)]
    animation: Option<WorldTextureAnimation>,
}

#[derive(Debug)]
pub struct WorldTextureBank(Vec<WorldTexture>);

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
                reader.seek(SeekFrom::Start(i.offset as u64))?;
                <Compressed<WorldTexture>>::read_options(
                    reader,
                    endian,
                    args! {
                        height: i.height as usize,
                        width: i.width as usize,
                        animation: (i.animation_frames > 0).then_some(WorldTextureAnimation {
                            frames: i.animation_frames as usize,
                            next_frame: i.next_animation_texture_id as usize,
                        })
                    },
                )
                .map(Compressed::into_inner)
            })
            .collect::<BinResult<_>>()?;

        Ok(Self(bank))
    }
}

impl BinWrite for WorldTextureBank {
    type Args<'a> = &'a mut WorldTextureInfoBank;

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let infos = self
            .0
            .iter()
            .map(|t| -> BinResult<WorldTextureInfo> {
                let start = writer.stream_position()?;
                let mut size_decompressed = 0;
                Compressed::from(t).write_options(
                    writer,
                    endian,
                    CompressedArgs::OutputSize((), &mut size_decompressed),
                )?;
                let end = writer.stream_position()?;
                Ok(WorldTextureInfo {
                    width: t.mips[0].width() as u16,
                    height: t.mips[0].height() as u16,
                    offset: start as u32,
                    size_compressed: (end - start) as u32,
                    size_decompressed: size_decompressed as u32,
                    animation_frames: t.animation.as_ref().map(|a| a.frames).unwrap_or_default()
                        as u32,
                    next_animation_texture_id: t
                        .animation
                        .as_ref()
                        .map(|a| a.next_frame)
                        .unwrap_or_default() as u32,
                })
            })
            .collect::<BinResult<Vec<_>>>()?;

        *args = WorldTextureInfoBank(infos);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor, iter};

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

        let mut output = Cursor::new(vec![]);
        let mut new_info_bank = WorldTextureInfoBank::default();
        texture_bank.write_le_args(&mut output, &mut new_info_bank);

        iter::zip(info_bank.0.iter(), new_info_bank.0.iter())
            .map(|(o, n)| {
                assert_eq!(o.width, n.width);
                assert_eq!(o.height, n.height);
                assert_eq!(o.size_decompressed, n.size_decompressed);
                assert_eq!(o.animation_frames, n.animation_frames);
                assert_eq!(o.next_animation_texture_id, n.next_animation_texture_id);
            })
            .count();

        output.set_position(0);
        let new_texture_bank = WorldTextureBank::read_le_args(&mut output, &new_info_bank)?;

        Ok(())
    }
}
