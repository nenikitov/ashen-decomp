// TODO(nenikitov): When textures are moved to a separate public module later,
// this `pub(crate)` could be deleted
pub(crate) mod dat;

use dat::{offset::TextureOffset, size::TextureSize, texture::MippedTexture};

use self::dat::texture::Texture;
use super::{extension::*, AssetParser};
use crate::utils::{compression::decompress, nom::*};

pub enum TextureMipKind {
    NonMipped(Texture),
    Mipped(MippedTexture),
}

pub enum TextureAnimationKind {
    Static(TextureMipKind),
    Animated(Vec<TextureMipKind>),
}

pub struct TextureOffsetCollection;

impl AssetParser<Pack> for TextureOffsetCollection {
    type Output = Vec<TextureOffset>;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (_, offsets) = multi::many0(TextureOffset::parser(()))(input)?;

            Ok((&[], offsets))
        }
    }
}

pub struct MippedTextureCollection;

impl AssetParser<Pack> for MippedTextureCollection {
    type Output = Vec<TextureAnimationKind>;

    type Context<'ctx> = &'ctx [TextureOffset];

    fn parser(offsets: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let textures = offsets
                .iter()
                .map(|o| {
                    let input = &input[o.offset as usize..][..o.size_compressed as usize];
                    let input = decompress(input);

                    MippedTexture::parser(TextureSize {
                        width: o.width as usize,
                        height: o.height as usize,
                    })(&input)
                    .map(|(_, d)| (d, o))
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;

            let textures = textures
                .iter()
                .cloned()
                .map(|(texture, offset)| {
                    if offset.animation_frames == 0 {
                        TextureAnimationKind::Static(TextureMipKind::Mipped(texture))
                    } else {
                        let mut frames = Vec::with_capacity(offset.animation_frames as usize);

                        (0..offset.animation_frames).fold(
                            (texture, offset),
                            |(texture, offset), _| {
                                frames.push(TextureMipKind::Mipped(texture));
                                textures[offset.next_animation_texture_id as usize].clone()
                            },
                        );

                        TextureAnimationKind::Animated(frames)
                    }
                })
                .collect::<Vec<_>>();

            Ok((&[], textures))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, path::PathBuf};

    use super::*;
    use crate::{
        asset::color_map::{ColorMap, PaletteTexture},
        utils::{format::*, test::*},
    };

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("4F.dat");
    const TEXTURE_INFO_DATA: LazyCell<Vec<u8>> = deflated_file!("93.dat");
    const TEXTURE_DATA: LazyCell<Vec<u8>> = deflated_file!("95.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, color_map) = <ColorMap as AssetParser<Pack>>::parser(())(&COLOR_MAP_DATA)?;
        let color_map = &color_map.shades[15];
        let (_, offsets) =
            <TextureOffsetCollection as AssetParser<Pack>>::parser(())(&TEXTURE_INFO_DATA)?;
        let (_, textures) =
            <MippedTextureCollection as AssetParser<Pack>>::parser(&offsets)(&TEXTURE_DATA)?;

        let output_dir = PathBuf::from(parsed_file_path!("textures/"));

        textures
            .iter()
            .enumerate()
            .try_for_each(|(i, texture)| match texture {
                TextureAnimationKind::Static(TextureMipKind::NonMipped(_)) => {
                    unreachable!("World textures are always mipped")
                }
                TextureAnimationKind::Static(TextureMipKind::Mipped(t)) => {
                    t.mips.iter().enumerate().try_for_each(|(m, mip)| {
                        let file = &output_dir.join(format!("{i:0>3X}-mip-{m}.png"));
                        output_file(file, mip.colors.with_palette(color_map).to_png())
                    })
                }
                TextureAnimationKind::Animated(t) => {
                    let frames = t.iter().map(|t| match t {
                        TextureMipKind::NonMipped(_) => {
                            unreachable!("World textures are always mipped")
                        }
                        TextureMipKind::Mipped(t) => t,
                    });

                    let mut data = vec![];
                    for (f, frame) in frames.enumerate() {
                        for (m, mip) in frame.mips.iter().enumerate() {
                            if data.len() <= m {
                                data.push(vec![]);
                            }

                            data[m].push(mip.colors.with_palette(color_map))
                        }
                    }

                    data.iter().enumerate().try_for_each(|(m, mip)| {
                        let file = &output_dir.join(format!("{i:0>3X}-mip-{m}.gif"));
                        output_file(file, mip.to_gif())
                    })
                }
            });

        Ok(())
    }
}
