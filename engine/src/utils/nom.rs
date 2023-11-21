//! Re-exports some `nom` items, to make it easier to use.
//!
//! This is pretty much the library's nom prelude.

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

    // TODO(nenikitov): Find a better way to do it without using 3 generics so we don't have to call with `_, _, _`
    // Maybe remove `I` and `E` because we are only using `&[u8]` input and our own error type.
    pub fn count_const<const COUNT: usize, I, O, E>(
        mut f: impl nom::Parser<I, O, E>,
    ) -> impl FnMut(I) -> nom::IResult<I, [O; COUNT], E>
    where
        I: Clone + PartialEq,
        E: nom::error::ParseError<I>,
    {
        let mut f = nom::multi::count(f, COUNT);
        move |i: I| {
            let (i, items) = f(i)?;
            match items.try_into() {
                Ok(items) => Ok((i, items)),
                Err(_) => unreachable!(),
            }
        }
    }
}

pub type Input<'a> = &'a [u8];

pub type Result<'a, T> = nom::IResult<Input<'a>, T, crate::error::ParseError>;
