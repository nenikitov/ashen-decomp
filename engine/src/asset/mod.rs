pub mod color_map;
pub mod gamma_table;
pub mod model;
pub mod pack_file;
mod pack_info;
pub mod skybox;
pub mod sound;
pub mod string_table;

use crate::utils::nom::{Input, Result};

/// Definition for all available extensions that the engine can parse.
pub mod extension {
    #[sealed::sealed]
    pub trait Extension: AsRef<str> + for<'str> TryFrom<&'str str> {}

    #[derive(Debug, thiserror::Error)]
    #[error("The provided extension is invalid '{}'", self.0)]
    pub struct ExtensionMismatchError(String);

    macro_rules! impl_extension {
        ($(#[$docs:meta])+ $name:ident => $ext:literal) => {
            $(#[$docs])+
            pub struct $name;

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    $ext
                }
            }

            impl TryFrom<&str> for $name {
                type Error = ExtensionMismatchError;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    if value == $ext {
                        Ok(Self)
                    } else {
                        Err(ExtensionMismatchError(value.to_owned()))
                    }
                }
            }

            #[sealed::sealed]
            impl Extension for $name {}
        };
    }

    impl_extension!(
        /// Wildcard
        Wildcard => "*"
    );

    impl_extension!(
        /// Extension that implies that the asset comes from ashen's files (packfile).
        Pack => "pack"
    );
}

pub trait AssetParser<Ext>
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
