use derive_more::Deref;

use super::*;

#[binrw]
#[derive(Debug, Deref)]
pub struct EntireFile(#[br(parse_with = until_eof)] pub Vec<u8>);
