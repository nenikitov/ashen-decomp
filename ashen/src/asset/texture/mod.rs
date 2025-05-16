mod dat;

use dat::{offset::TextureOffset, texture::MippedTexture};
pub use dat::{size::TextureSize, texture::Texture};

use super::Parser;
use crate::utils::{compression::decompress, nom::*};

pub struct AnimatedTexture {
    pub frames: Vec<Texture>,
}

// TODO(Unavailable): Implement `Index` and `IntoIterator`.
pub enum WorldTexture {
    Static([Texture; 4]),
    Animated([AnimatedTexture; 4]),
}

impl Parser for Vec<TextureOffset> {
    type Context<'ctx> = ();
    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (_, offsets) = multi::many0(TextureOffset::parser(()))(input)?;

            Ok((&[], offsets))
        }
    }
}

impl Parser for Vec<WorldTexture> {
    type Context<'ctx> = &'ctx [TextureOffset];

    fn parser(offsets: Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
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
                        return WorldTexture::Static(texture.mips);
                    }

                    let mut mips: [_; 4] = std::array::from_fn(|_| {
                        Vec::with_capacity(offset.animation_frames as usize)
                    });

                    (0..offset.animation_frames).fold((texture, offset), |(texture, offset), _| {
                        for (dst, src) in Iterator::zip(mips.iter_mut(), texture.mips) {
                            dst.push(src);
                        }
                        textures[offset.next_animation_texture_id as usize].clone()
                    });

                    WorldTexture::Animated(mips.map(|frames| AnimatedTexture { frames }))
                })
                .collect::<Vec<_>>();

            Ok((&[], textures))
        }
    }
}

impl Texture {
    #[cfg(feature = "conv")]
    pub fn to_png<W>(
        &self,
        mut writer: W,
        palette: &[super::color_map::Color; 256],
    ) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use crate::utils::format::{PaletteTexture, PngFile};
        writer.write_all(&self.colors.with_palette(&*palette).to_png())
    }
}

impl AnimatedTexture {
    #[cfg(feature = "conv")]
    pub fn to_gif<W>(
        &self,
        mut writer: W,
        palette: &[super::color_map::Color; 256],
    ) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use crate::utils::format::{GifFile, PaletteTexture};
        let bytes = self
            .frames
            .iter()
            .map(|texture| texture.with_palette(palette))
            .collect::<Vec<_>>()
            .to_gif();
        writer.write_all(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::{asset::color_map::ColorMap, utils::test::*};

    const COLOR_MAP: LazyCell<Vec<u8>> = LazyCell::new(|| deflated_file!("4F.dat"));
    const TEXTURE_INFO: LazyCell<Vec<u8>> = LazyCell::new(|| deflated_file!("93.dat"));
    const TEXTURE: LazyCell<Vec<u8>> = LazyCell::new(|| deflated_file!("95.dat"));

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, color_map) = ColorMap::parser(())(&COLOR_MAP)?;
        let palette = &color_map.shades[15];

        let (_, offsets) = Vec::<TextureOffset>::parser(())(&TEXTURE_INFO)?;
        let (_, textures) = Vec::<WorldTexture>::parser(&offsets)(&TEXTURE)?;

        let output_dir = PARSED_PATH.join("texture");

        textures
            .iter()
            .enumerate()
            .try_for_each(|(i, world_texture)| match world_texture {
                WorldTexture::Static(r#static) => {
                    r#static.iter().enumerate().try_for_each(|(j, texture)| {
                        output_file(output_dir.join(format!("{i:0>3X}-mip-{j}.png")))
                            .and_then(|w| texture.to_png(w, palette))
                    })
                }
                WorldTexture::Animated(animated) => {
                    animated
                        .iter()
                        .enumerate()
                        .try_for_each(|(j, animated_texture)| {
                            output_file(output_dir.join(format!("{i:0>3X}-mip-{j}.gif")))
                                .and_then(|w| animated_texture.to_gif(w, palette))
                        })
                }
            })?;

        Ok(())
    }
}
