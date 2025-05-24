use super::*;

#[derive(Clone, Debug, NamedArgs)]
pub struct TextureReadArgs {
    pub width: usize,
    pub height: usize,
}

#[binrw]
#[br(import_raw(args: TextureReadArgs))]
#[derive(Clone, Debug)]
pub struct Texture(
    #[br(
        args { count: args.height, inner: binrw::args! { count: args.width, inner: () }},
    )]
    Vec<Vec<u8>>,
);

impl Texture {
    pub fn width(&self) -> usize {
        self.0.first().map_or(0, Vec::len)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
}
