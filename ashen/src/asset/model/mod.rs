mod dat;

use dat::{
    frame::{ModelFrame, ModelSpecs},
    header::ModelHeader,
    sequence::ModelSequence,
    triangle::{ModelTriangle, TextureDimensions},
};

use super::{
    Parser,
    color_map::Color,
    texture::{Texture, TextureSize},
};
use crate::utils::{format::ModelPythonFile, nom::*};

pub struct Model {
    pub texture: Texture,
    pub triangles: Vec<ModelTriangle>,
    pub sequences: Vec<ModelSequence>,
    pub frames: Vec<ModelFrame>,
}

impl Parser for Model {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (_, header) = ModelHeader::parser(())(input)?;

            let (_, triangles) = multi::count!(
                ModelTriangle::parser(TextureDimensions {
                    width: header.texture_width,
                    height: header.texture_height
                }),
                header.triangle_count as usize
            )(&input[header.offset_triangles as usize..])?;

            let (_, texture) = Texture::parser(TextureSize {
                width: header.texture_width as usize,
                height: header.texture_height as usize,
            })(&input[header.offset_texture as usize..])?;

            let (_, sequences) = multi::count!(
                ModelSequence::parser(input),
                header.sequence_count as usize
            )(&input[header.offset_sequences as usize..])?;

            let (_, frames) = multi::count!(
                ModelFrame::parser(ModelSpecs {
                    vertex_count: header.vertex_count,
                    triangle_count: header.triangle_count,
                    frame_size: header.frame_size
                }),
                header.frame_count as usize
            )(&input[header.offset_frames as usize..])?;

            Ok((
                &[],
                Self {
                    texture,
                    triangles,
                    sequences,
                    frames,
                },
            ))
        }
    }
}

impl Model {
    // TODO(Unavailable): Could provide conversions to gif using `shadybug`.
    pub fn to_blender_script<W>(&self, mut writer: W, palette: &[Color; 256]) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(writer, "{}", &self.to_py(&*palette))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, collections::HashMap};

    use super::*;
    use crate::{asset::color_map::ColorMap, utils::test::*};

    const COLOR_MAPS: LazyCell<HashMap<&str, Vec<u8>>> = LazyCell::new(|| {
        HashMap::from([
            ("creature", deflated_file!("01.dat")),
            ("ghost-creature", deflated_file!("03.dat")),
            ("pickup", deflated_file!("04.dat")),
        ])
    });

    const MODELS: LazyCell<Vec<(&str, &str, Vec<u8>)>> = LazyCell::new(|| {
        vec![
            ("aquagore", "creature", deflated_file!("0A-deflated.dat")),
            ("broodmaw", "creature", deflated_file!("0B-deflated.dat")),
            (
                "cryptcrawler",
                "creature",
                deflated_file!("0C-deflated.dat"),
            ),
            ("firedeacon", "creature", deflated_file!("0D-deflated.dat")),
            ("hunter", "creature", deflated_file!("0E-deflated.dat")),
            ("psistalker", "creature", deflated_file!("0F-deflated.dat")),
            ("stormfluke", "creature", deflated_file!("10-deflated.dat")),
            ("tentacle", "creature", deflated_file!("11-deflated.dat")),
            (
                "wraith",
                "ghost-creature",
                deflated_file!("12-deflated.dat"),
            ),
            ("prime-entity", "pickup", deflated_file!("13-deflated.dat")),
            // TODO(nenikitov): This model doesn't look good with any palette
            (
                "player-model",
                "creature",
                deflated_file!("14-deflated.dat"),
            ),
            ("vanessa", "pickup", deflated_file!("15-deflated.dat")),
            ("rocket", "pickup", deflated_file!("16-deflated.dat")),
            ("grenade", "pickup", deflated_file!("17-deflated.dat")),
            ("fx-blast", "pickup", deflated_file!("18-deflated.dat")),
            ("aquagore-shot", "pickup", deflated_file!("19-deflated.dat")),
            ("broodmaw-shot", "pickup", deflated_file!("1A-deflated.dat")),
            (
                "cryptcrawler-shot",
                "pickup",
                deflated_file!("1B-deflated.dat"),
            ),
            (
                "firedeacon-shot",
                "pickup",
                deflated_file!("1C-deflated.dat"),
            ),
            ("gib-generic-1", "pickup", deflated_file!("1D-deflated.dat")),
            ("gib-generic-2", "pickup", deflated_file!("1E-deflated.dat")),
            ("gib-generic-3", "pickup", deflated_file!("1F-deflated.dat")),
            (
                "blood-generic-1",
                "pickup",
                deflated_file!("20-deflated.dat"),
            ),
            ("charles", "pickup", deflated_file!("21-deflated.dat")),
            (
                "human-gib-generic-1",
                "pickup",
                deflated_file!("22-deflated.dat"),
            ),
            (
                "human-gib-generic-2",
                "pickup",
                deflated_file!("23-deflated.dat"),
            ),
            (
                "human-gib-generic-3",
                "pickup",
                deflated_file!("24-deflated.dat"),
            ),
            (
                "pickup-ammo-pistol",
                "pickup",
                deflated_file!("25-deflated.dat"),
            ),
            (
                "pickup-ammo-double-pistol",
                "pickup",
                deflated_file!("26-deflated.dat"),
            ),
            (
                "pickup-ammo-shotgun",
                "pickup",
                deflated_file!("27-deflated.dat"),
            ),
            (
                "pickup-ammo-machinegun",
                "pickup",
                deflated_file!("28-deflated.dat"),
            ),
            (
                "pickup-ammo-sniper",
                "pickup",
                deflated_file!("29-deflated.dat"),
            ),
            (
                "pickup-ammo-grenade",
                "pickup",
                deflated_file!("2A-deflated.dat"),
            ),
            (
                "pickup-ammo-rocket",
                "pickup",
                deflated_file!("2B-deflated.dat"),
            ),
            (
                "pickup-ammo-gatlinggun",
                "pickup",
                deflated_file!("2C-deflated.dat"),
            ),
            (
                "pickup-weapon-pistol",
                "pickup",
                deflated_file!("2D-deflated.dat"),
            ),
            (
                "pickup-weapon-double-pistol",
                "pickup",
                deflated_file!("2E-deflated.dat"),
            ),
            (
                "pickup-weapon-shotgun",
                "pickup",
                deflated_file!("2F-deflated.dat"),
            ),
            (
                "pickup-weapon-machinegun",
                "pickup",
                deflated_file!("30-deflated.dat"),
            ),
            (
                "pickup-weapon-sniper",
                "pickup",
                deflated_file!("31-deflated.dat"),
            ),
            (
                "pickup-weapon-grenade",
                "pickup",
                deflated_file!("32-deflated.dat"),
            ),
            (
                "pickup-weapon-gatlinggun",
                "pickup",
                deflated_file!("33-deflated.dat"),
            ),
            (
                "pickup-weapon-shockwave",
                "pickup",
                deflated_file!("34-deflated.dat"),
            ),
            (
                "pickup-ghost-vision",
                "pickup",
                deflated_file!("35-deflated.dat"),
            ),
            (
                "pickup-focitalisman",
                "pickup",
                deflated_file!("36-deflated.dat"),
            ),
            ("pickup-letter", "pickup", deflated_file!("37-deflated.dat")),
            // TODO(nenikitov): This model doesn't look good with any palette
            ("pickup-key-1", "pickup", deflated_file!("38-deflated.dat")),
            (
                "pickup-flakjacket-25",
                "pickup",
                deflated_file!("39-deflated.dat"),
            ),
            (
                "pickup-flakjacket-50",
                "pickup",
                deflated_file!("3A-deflated.dat"),
            ),
            (
                "pickup-flakjacket-100",
                "pickup",
                deflated_file!("3B-deflated.dat"),
            ),
        ]
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let palettes = <HashMap<_, _>>::from_iter(COLOR_MAPS.iter().map(|(name, data)| {
            let (_, color_map) = ColorMap::parser(())(data).expect("Color map is valid");
            (*name, color_map.shades[15])
        }));

        MODELS.iter().try_for_each(|(name, palette, data)| {
            let palette = palettes.get(palette).expect("Color map is present");
            let (_, model) = Model::parser(())(data)?;

            output_file(PARSED_PATH.join(format!("model/{name}.py")))
                .and_then(|w| model.to_blender_script(w, palette))?;

            Ok(())
        })
    }
}
