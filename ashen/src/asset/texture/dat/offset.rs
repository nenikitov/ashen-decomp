use crate::{asset::AssetParser, utils::nom::*};

pub struct TextureOffset {
    pub width: u16,
    pub height: u16,
    pub offset: u32,
    pub size_compressed: u32,
    pub size_decompressed: u32,
    pub animation_frames: u32,
    pub next_animation_texture_id: u32,
}

impl AssetParser for TextureOffset {
    type Context<'ctx> = ();

    type Output = Self;

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, width) = number::le_u16(input)?;
            let (input, height) = number::le_u16(input)?;
            let (input, offset) = number::le_u32(input)?;
            let (input, size_compressed) = number::le_u32(input)?;
            let (input, size_decompressed) = number::le_u32(input)?;
            let (input, animation_frames) = number::le_u32(input)?;
            let (input, next_animation_texture_id) = number::le_u32(input)?;

            Ok((
                input,
                Self {
                    width,
                    height,
                    offset,
                    size_compressed,
                    size_decompressed,
                    animation_frames,
                    next_animation_texture_id,
                },
            ))
        }
    }
}
