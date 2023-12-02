use super::AssetChunk;
use crate::utils::nom::*;
use nom::error::{ErrorKind, ParseError};
use std::ops::Index;

#[derive(Debug)]
pub struct PackInfo {
    pub offset: u32,
    pub size: u32,
}

impl AssetChunk for PackInfo {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, offset) = number::le_u32(input)?;

        let (input, size) = number::le_u32(input)?;

        let (input, padding) = number::le_u32(input)?;
        // TODO(nenikitov): Make it return `Result`
        assert_eq!(padding, 0);

        Ok((input, PackInfo { offset, size }))
    }
}

impl Index<PackInfo> for [u8] {
    type Output = [u8];

    fn index(&self, index: PackInfo) -> &Self::Output {
        &self[&index]
    }
}

impl Index<&PackInfo> for [u8] {
    type Output = [u8];

    fn index(&self, index: &PackInfo) -> &Self::Output {
        &self[index.offset as usize..][..index.size as usize]
    }
}
