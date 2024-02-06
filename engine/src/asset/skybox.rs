use super::{
    extension::*,
    texture::dat::{size::TextureSize, texture::Texture},
    AssetParser,
};
use crate::{asset::color_map::Color, utils::nom::*};

const COLOR_COUNT: usize = 256;

pub struct Skybox {
    pub palette: Vec<Color>,
    pub texture: Texture,
}

impl AssetParser<Pack> for Skybox {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, width) = number::le_u32(input)?;

            let (input, height) = number::le_u32(input)?;

            let (input, palette) = multi::count!(number::le_u16, 256)(input)?;
            let palette: Vec<_> = palette.into_iter().map(Color::from_12_bit).collect();

            let (_, texture) = Texture::parser(TextureSize {
                width: width as usize,
                height: height as usize,
            })(input)?;

            Ok((&[], Self { palette, texture }))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::{
        asset::color_map::PaletteTexture,
        utils::{format::*, test::*},
    };

    const SKYBOX_DATA: LazyCell<Vec<u8>> = deflated_file!("3C.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, skybox) = <Skybox as AssetParser<Pack>>::parser(())(&SKYBOX_DATA)?;

        output_file(
            parsed_file_path!("skyboxes/level-1.png"),
            skybox.texture.with_palette(&skybox.palette).to_png(),
        )?;

        Ok(())
    }
}
