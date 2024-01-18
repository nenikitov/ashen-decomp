use std::ops::Index;

use crate::asset::AssetChunkWithContext;

use super::offset::TextureOffset;

pub struct Texture {
    texture: Vec<Vec<u8>>,
}

pub struct TextureContext<'a> {
    pub full_data: &'a [u8],
    pub offset: TextureOffset,
}

impl Index<&TextureOffset> for [u8] {
    type Output = [u8];

    fn index(&self, index: &TextureOffset) -> &Self::Output {
        &self[index.offset as usize..][..index.size_compressed as usize]
    }
}

impl AssetChunkWithContext for Texture {
    type Context<'a> = TextureContext<'a>;

    fn parse(context: Self::Context<'_>) -> impl Fn(&[u8]) -> crate::utils::nom::Result<Self> {
        let texture_data = &context.full_data[&context.offset];
        move |input| {
            dbg!(context.offset.offset);
            //dbg!(texture_data);
            dbg!(texture_data.len());
            dbg!(context.offset.size_compressed);
            dbg!(context.offset.size_decompressed);
            dbg!(context.offset.width * context.offset.height);
            todo!()
        }
    }
}
