use std::{
    cell::Cell,
    io::{Read, Seek, Write},
};

use derive_more::{Debug, Deref};
use itertools::Itertools;
use num::{Bounded, NumCast, Zero};

use crate::utils::math::IntoFromNormalizedF32;

#[rustfmt::skip]
#[allow(unused_imports)]
pub use binrw::{*, helpers::*};

#[binrw]
#[derive(Debug, Deref)]
pub struct NormalizedF32<T>(
    #[deref]
    #[br(map = |x: T| x.into_normalized_f32())]
    #[bw(map = |x| T::from_normalized_f32(*x))]
    f32,
    #[debug(ignore)]
    #[ignore]
    std::marker::PhantomData<T>,
)
where
    T: Bounded
        + NumCast
        + Zero
        + PartialOrd
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>;

#[derive(NamedArgs, Clone)]
pub struct PaddedNullStringArgs {
    len: usize,
}

#[derive(Debug, Deref)]
pub struct PaddedNullString(String);

impl BinRead for PaddedNullString {
    type Args<'a> = PaddedNullStringArgs;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let value: Vec<u8> =
            binrw::helpers::count_with(args.len, u8::read_options)(reader, endian, ())?;

        let value = String::from_utf8_lossy(&value)
            .trim_end_matches('\0')
            .to_string();

        Ok(Self(value))
    }
}

impl BinWrite for PaddedNullString {
    type Args<'a> = PaddedNullStringArgs;

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let bytes = self.0.as_bytes();

        if bytes.len() >= args.len {
            return Err(binrw::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "string too large to fit into specified length",
            )));
        }

        bytes.write_options(writer, endian, ())?;
        std::iter::repeat(0u8)
            .take(args.len - bytes.len())
            .collect_vec()
            .write_options(writer, endian, ())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PosMarker<T> {
    pub pos: Cell<u64>,
    pub value: T,
}

impl<T> BinRead for PosMarker<T>
where
    T: BinRead,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        T::read_options(reader, endian, args).map(|value| Self {
            pos: Cell::new(pos),
            value,
        })
    }
}

impl<T> BinWrite for PosMarker<T>
where
    T: BinWrite<Args<'static> = ()> + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.pos.set(writer.stream_position()?);
        T::default().write_options(writer, endian, args)
    }
}

pub fn args_iter_write<'a, T, Writer, Arg, It>(
    it: It,
) -> impl Copy + FnOnce(&Vec<T>, &mut Writer, Endian, ()) -> BinResult<()>
where
    T: BinWrite<Args<'a> = Arg>,
    Writer: Write + Seek,
    It: IntoIterator<Item = Arg> + Copy,
{
    move |elems, writer, endian, ()| {
        itertools::zip_eq(elems.iter(), it.into_iter())
            .map(|(e, arg)| e.write_options(writer, endian, arg))
            .collect()
    }
}
