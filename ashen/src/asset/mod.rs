pub mod color_map;
pub mod color_map_binrw;
pub mod gamma_table;
pub mod gamma_table_binrw;
pub mod model;
pub mod pack_file;
pub mod pack_file_binrw;
pub mod pack_info;
pub mod skybox;
pub mod sound;
pub mod string_table;
pub mod texture;

use crate::utils::nom::{Input, Result};

pub trait Parser
where
    Self: Sized,
{
    /// Extra information passed down to the parser.
    type Context<'ctx>;

    /// Generates a new parser with the provided context.
    fn parser(context: Self::Context<'_>) -> impl Fn(Input) -> Result<Self>;
}
