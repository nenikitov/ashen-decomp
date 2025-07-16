use std::{
    cell::Cell,
    io::{Read, Seek, SeekFrom, Write},
};

use super::*;

#[derive(Debug, Clone, Default)]
pub struct PosMarker<P> {
    pub pos: Cell<u64>,
    pub value: P,
}

impl<P> PosMarker<P> {
    pub fn store_offset<'p, T>(&'p self) -> impl FnOnce(T) -> AtPosMarker<'p, P, T> {
        move |obj| AtPosMarker { pos: self, obj }
    }
}

impl<P> BinRead for PosMarker<P>
where
    P: BinRead,
{
    type Args<'a> = P::Args<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        P::read_options(reader, endian, args).map(|value| Self {
            pos: Cell::new(pos),
            value,
        })
    }
}

impl<P> BinWrite for PosMarker<P>
where
    P: BinWrite<Args<'static> = ()> + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.pos.set(writer.stream_position()?);
        P::default().write_options(writer, endian, args)
    }
}

pub struct AtPosMarker<'p, P, T> {
    pos: &'p PosMarker<P>,
    obj: T,
}

impl<'p, P, T> BinWrite for AtPosMarker<'p, P, T>
where
    T: BinWrite,
    P: TryFrom<u64> + for<'a> BinWrite<Args<'a> = ()>,
{
    type Args<'a> = T::Args<'a>;

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let pos_before = writer.stream_position()?;
        self.obj.write_options(writer, endian, args)?;
        let pos_after = writer.stream_position()?;

        let pos_marker = self.pos.pos.get();
        writer.seek(SeekFrom::Start(pos_marker))?;
        P::try_from(pos_before)
            .map_err(|e| Error::Custom {
                pos: pos_marker,
                err: Box::new(format!(
                    "Address {pos_before:x} too large to be stored in a pointer"
                )),
            })?
            .write_options(writer, endian, ())?;
        writer.seek(SeekFrom::Start(pos_after))?;

        Ok(())
    }
}
