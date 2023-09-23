use crate::format::{asset_table::AssetType, *};

pub struct Point {
    vertex_index: u16,
    u: u16,
    v: u16,
}

pub struct Triangle {
    points: [Point; 3],
}

#[derive(Debug, PartialEq)]
pub struct Model {
    texture: Vec<Vec<u8>>,
}

impl AssetLoad for Model {
    fn load(bytes: &[u8]) -> Result<(Self, usize), DataError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn file_type() -> AssetType
    where
        Self: Sized,
    {
        todo!()
    }
}
