//! Re-export `nom::{*}::complete` stuff because it is easier to use :).

macro_rules! re_export {
    ($path:ident) => {
        pub mod $path {
            pub use nom::$path::complete::*;
        }
    };
}

re_export!(bits);
re_export!(bytes);
re_export!(character);
re_export!(number);

pub mod multi {
    pub use nom::multi::*;
}

pub type Result<'a, T> = nom::IResult<&'a [u8], T>;
