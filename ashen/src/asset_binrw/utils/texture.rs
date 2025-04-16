use super::*;

#[binrw]
#[br(import { height: usize, width: usize })]
#[derive(Clone, Debug)]
pub struct Texture(
    #[br(
        args { count: height, inner: binrw::args! { count: width, inner: () }},
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
