use super::{
    Parser,
    texture::{Texture, TextureSize},
};
use crate::{asset::color_map::Color, utils::nom::*};

const COLOR_COUNT: usize = 256;

pub struct Skybox {
    pub palette: Vec<Color>,
    pub texture: Texture,
}

impl Parser for Skybox {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
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

impl Skybox {
    #[cfg(feature = "conv")]
    pub fn to_png<W>(&self, mut writer: W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use crate::{asset::texture::PaletteTexture, utils::format::PngFile};

        writer.write_all(&self.texture.with_palette(&self.palette).to_png())
    }
}

#[cfg(test)]
#[cfg(feature = "conv")]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::utils::test::*;

    const SKYBOXES: LazyCell<Vec<(&str, Vec<u8>)>> = LazyCell::new(|| {
        vec![
            ("level1", deflated_file!("3C.dat")),
            ("level2", deflated_file!("3D.dat")),
            ("level3", deflated_file!("3E.dat")),
            ("level4", deflated_file!("3F.dat")),
            ("level5", deflated_file!("40.dat")),
            ("level6", deflated_file!("41.dat")),
        ]
    });

    #[cfg(feature = "conv")]
    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        SKYBOXES.iter().try_for_each(|(name, data)| {
            let (_, skybox) = Skybox::parser(())(data)?;

            output_file(PARSED_PATH.join(format!("skybox/{name}.png")))
                .and_then(|w| skybox.to_png(w))?;

            Ok(())
        })
    }
}
