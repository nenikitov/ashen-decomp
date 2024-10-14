pub mod color_map;
pub mod gamma_table;
pub mod model;
pub mod pack_file;
mod pack_info;
pub mod skybox;
pub mod sound;
pub mod string_table;
pub mod texture;

use crate::utils::nom::{Input, Result};

pub trait AssetParser
where
    Self: Sized,
{
    /// The final value that would be returned by [`parser`].
    ///
    /// _Most_ of the time this would be equal to `Self`.
    ///
    /// A hypothetical `TextureCollection` would return `Vec<Texture>` as its output.
    ///
    /// [`parser`]: Self::parser
    type Output;

    /// Extra information passed down to the parser.
    type Context<'ctx>;

    /// Generates a new parser with the provided context.
    fn parser(context: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output>;
}
