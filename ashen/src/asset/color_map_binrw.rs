use std::ops::Deref;

use binrw::{BinRead, BinWrite, binrw};
use glam::Vec3;

const LEN_ROWS: usize = 256;
const LEN_COLS: usize = 32;

#[binrw]
#[derive(Debug)]
pub struct Color12Bit(
);

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct ColorMap {
    #[br(args {
        count: LEN_ROWS,
        inner: binrw::args! { count: LEN_COLS, inner: () },
    })]
    shades: Vec<Vec<Color12Bit>>,
}
