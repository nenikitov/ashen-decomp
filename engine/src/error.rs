use crate::utils::nom::Input;
use nom::error::ErrorKind as NomErrorKind;
use std::num::NonZeroUsize;

// TODO(Unavailable): implement Error and Display.
//
// I really don't know how the output should look like, can be done later. When
// implementing `Error` don't forget to add nom as `source`.

/// An error that could be encountered when calling the [`Asset::parse`]
/// function.
///
/// [`Asset::parse`]: crate::asset::Asset::parse
#[derive(Debug, Clone)]
pub struct ParseError {
    bytes: Box<[u8]>,
    kind: ErrorKind,
    /// A custom message that should be used when `Display`ing the error,
    /// instead of the default one.
    ///
    /// This should be used if context would be better than a generic default
    /// message. This might be removed if backtrace/context based error handling
    /// is implement with nom (see `append()` method below).
    custom: Option<Box<str>>,
}

impl ParseError {
    /// Creates a new `ParseError` with a kind of `UnsupportedExtension`.
    pub(crate) fn unsupported_extension<B, I>(bytes: B, unsupported: I) -> Self
    where
        I: Into<String>,
        B: AsRef<[u8]>,
    {
        Self {
            bytes: bytes.as_ref().to_vec().into_boxed_slice(),
            kind: ErrorKind::UnsupportedExtension {
                unsupported: Into::into(unsupported),
            },
            custom: None,
        }
    }

    /// The bytes that caused this error.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The kind of error encountered.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// The requested extension is not supported.
    UnsupportedExtension {
        unsupported: String,
        /* TODO(Unavailable): supported: Box<[Extension]> */
    },
    /// Not enough bytes where provided to parse the `Asset`.
    ///
    /// `Incomplete` will only be returned if the `Asset`'s size is fixed,
    /// otherwise `ErrorKind::Nom::Eof` would be returned instead.
    Incomplete(NonZeroUsize),
    /// Generic nom error.
    Nom(NomErrorKind),
}

impl nom::error::ParseError<Input<'_>> for ParseError {
    fn from_error_kind(input: Input<'_>, kind: NomErrorKind) -> Self {
        // kinda bad, but solves problems with lifetimes; also, _most_ of the
        // time we are panicking out if an error is encountered.
        let input = input.to_vec().into_boxed_slice();
        Self {
            bytes: input,
            kind: ErrorKind::Nom(kind),
            custom: None,
        }
    }

    // TODO(Unavailable): could be used to build a backtrace of errors.
    fn append(_: Input<'_>, _: NomErrorKind, other: Self) -> Self {
        other
    }
}

/// Automatically converts `Result<..., ParseError>` -> `IResult<..., ..., ParseError>`.
impl From<ParseError> for nom::Err<ParseError> {
    fn from(value: ParseError) -> Self {
        nom::Err::Error(value)
    }
}

// helpers

/// Checks if `bytes` is at least as long as `length`.
pub(crate) fn ensure_bytes_length<B, S>(
    bytes: B,
    length: usize,
    message: S,
) -> Result<(), ParseError>
where
    B: AsRef<[u8]>,
    S: Into<Box<str>>,
{
    let bytes = bytes.as_ref();

    if bytes.len() < length {
        let missing = NonZeroUsize::new(length - bytes.len()).unwrap();
        return Err(ParseError {
            bytes: bytes.to_vec().into_boxed_slice(),
            kind: ErrorKind::Incomplete(missing),
            custom: Some(message.into()),
        });
    };

    Ok(())
}
