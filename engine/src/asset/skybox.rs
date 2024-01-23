use super::{extension::*, AssetParser};
use crate::{asset::color_map::Color, utils::nom::*};
use itertools::Itertools;

const COLOR_COUNT: usize = 256;

#[derive(Debug)]
pub struct Skybox {
    pub palette: Vec<Color>,
    pub texture: Vec<Vec<u8>>,
}

impl AssetParser<Pack> for Skybox {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (input, width) = number::le_u32(input)?;

            let (input, height) = number::le_u32(input)?;

            let (input, palette) = multi::count!(number::le_u16, 256)(input)?;
            let palette: Vec<_> = palette.into_iter().map(Color::from_12_bit).collect();

            let (input, texture) = multi::count!(number::le_u8, (width * height) as usize)(input)?;
            let texture = texture
                .into_iter()
                .chunks(width as usize)
                .into_iter()
                .map(Iterator::collect)
                .collect();

            Ok((&[], Self { palette, texture }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset::color_map::PaletteTexture,
        utils::{format::*, test::*},
    };
    use std::cell::LazyCell;

    const SKYBOX_DATA: LazyCell<Vec<u8>> = deflated_file!("3C.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, skybox) = <Skybox as AssetParser<Pack>>::parser(())(&SKYBOX_DATA)?;

        output_file(
            parsed_file_path!("skyboxes/level-1.ppm"),
            skybox.texture.with_palette(&skybox.palette).to_ppm(),
        )?;

        Ok(())
    }
}
