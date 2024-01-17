use crate::utils::nom::Result;

mod pack_info;

pub mod color_map;
pub mod gamma_table;
pub mod model;
pub mod pack_file;
pub mod skybox;
pub mod sound;
pub mod string_table;
pub mod texture;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    GammaTable,
    ColorMap,
    SoundCollection,
    StringTable,
    Skybox,
    Model,
    TextureInfo,
    TextureData,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Extension {
    #[default]
    Dat,
    Custom(String),
}

pub trait Asset
where
    Self: Sized,
{
    // TODO(Unavailable): Replace `kind()` with:
    //
    // ```
    // // rename to id?
    // fn kind() -> std::any::TypeId {
    //     std::any::TypeId::of::<Self>()
    // }
    // ```
    //
    // the only disadvantage is that `Self` also needs to be `'static` which
    // prevents us for implementing `Asset` for `&`-ed types. Right now is not
    // clear that we might need that, so I would hold this changes until then.

    /// Returns this Asset's kind.
    fn kind() -> Kind;

    /// Tries to parse this `Asset`'s kind.
    ///
    /// # Errors
    ///
    /// If the `input` is invalid for the provided `extension`.
    fn parse(input: &[u8], extension: Extension) -> Result<Self>;
}

pub(crate) trait AssetChunk
where
    Self: Sized,
{
    fn parse(input: &[u8]) -> Result<Self>;
}
