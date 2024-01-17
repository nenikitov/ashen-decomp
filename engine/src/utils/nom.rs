//! Re-exports some `nom` items, to make it easier to use.
//!
//! This is pretty much the library's nom prelude.

macro_rules! re_export {
    ($path:ident) => {
        #[doc = concat!("Re-exports all `nom::", stringify!($path), "::complete` items.")]
        pub mod $path {
            #[allow(unused_imports)]
            pub use nom::$path::complete::*;
        }
    };
}

/// Re-exports all `nom::number` items.
pub mod number {
    pub use nom::number::complete::*;

    use super::Result;
    use nom::number;
    use paste::paste;

    macro_rules! parser_for_fixed {
        ($type: ty, $bits: expr) => {
            paste! {
                pub fn [<le_$type:lower>](input: &[u8]) -> Result<$type> {
                    let (input, value) = number::complete::[<le_i$bits>](input)?;
                    Ok((input, $type::from_bits(value)))
                }
            }
        };
    }

    use fixed::types::{I16F16, I24F8, I8F24};

    parser_for_fixed!(I8F24, 32);
    parser_for_fixed!(I16F16, 32);
    parser_for_fixed!(I24F8, 32);
}

re_export!(bits);
re_export!(bytes);
re_export!(character);

/// Re-exports all `nom::multi` items.
pub mod multi {
    pub use nom::multi::*;

    /// Runs the embedded parser `count` times, gathering the results in `[O; N]`
    /// or `Vec<O>`.
    ///
    /// # Array
    ///
    /// If the macro is called with a single parameter (the nom parser), then an
    /// array with a inferred `N` (count) would be returned e.g:
    ///
    /// ```no_run
    /// use engine::utils::nom::{Result, multi, number};
    ///
    /// fn parse_u32s<const COUNT: usize>(input: &[u8]) -> Result<[u32; COUNT]> {
    ///     // N is infered by the function return type ([u32; COUNT]).
    ///     multi::count!(|i| number::le_u32(i))(input)
    /// }
    ///
    /// let input = [42, 0, 0, 0, 69, 0, 0, 0];
    ///
    /// assert_eq!(parse_u32s(&input).unwrap(), ([].as_slice(), [42, 69]));
    /// assert!(parse_u32s::<3>(&input).is_err());
    /// ```
    ///
    /// # Vec
    ///
    /// If the second parameter (count) is provided when calling the macro, a
    /// `Vec<_>` of **exactly** `count` elements would be returned.
    #[macro_export]
    #[doc(hidden)] // `macro_export` puts the macro at the root of the crate.
    macro_rules! __count {
        // [_; N] (infers N by context).
        ($fn:expr) => {
            $crate::utils::nom::__array_count($fn)
        };
        // Vec<_> (it contains **exactly** `$count` elements).
        ($fn:expr, $count:expr) => {
            $crate::utils::nom::multi::count($fn, $count)
        };
    }

    // prevents from conflicting with the actual `nom::multi::count` item.
    pub use __count as count;
}

#[doc(hidden)]
pub fn __array_count<'i, const N: usize, O>(
    f: impl Fn(Input<'i>) -> Result<'i, O>,
) -> impl FnMut(Input<'i>) -> Result<'i, [O; N]> {
    use std::mem::MaybeUninit;

    move |mut input| {
        let mut elements = MaybeUninit::<[O; N]>::uninit().transpose();

        for (idx, elem) in elements.iter_mut().enumerate() {
            match f(input) {
                Ok((__input, output)) => {
                    elem.write(output);
                    input = __input;
                }
                Err(e) => {
                    // Dropping a `MaybeUninit` does nothing, if an error occurs,
                    // already allocated elements should be dropped manually to
                    // prevent memory leaks.
                    for elem in &mut elements[..idx] {
                        // SAFETY: elements.iter_mut().next() was called at least
                        // `idx - 1` times.
                        unsafe { elem.assume_init_drop() }
                    }

                    return Err(e);
                }
            };
        }

        let elements = elements.transpose();

        // SAFETY: elements.iter_mut ensures that `f` was called for each element
        // without failing, which means that every element should be initialized.
        Ok((input, unsafe { MaybeUninit::assume_init(elements) }))
    }
}

/// The input type used through the crate's API.
pub type Input<'a> = &'a [u8];

/// All nom parsers implement this trait.
pub trait Parser<'a, O>: nom::Parser<Input<'a>, O, crate::error::ParseError> {}

// "trait alias"es to an specific impl of `nom::Parser`.
impl<'a, T, O> Parser<'a, O> for T where T: nom::Parser<Input<'a>, O, crate::error::ParseError> {}

/// Holds the result of parsing functions.
pub type Result<'a, O> = nom::IResult<Input<'a>, O, crate::error::ParseError>;
