use std::{cell::Cell, fmt::Display, io::SeekFrom};

use super::*;

#[derive(Debug, Clone, Default)]
pub struct Marker<M> {
    pub pos: Cell<u64>,
    pub value: M,
}

impl<M> BinRead for Marker<M>
where
    M: BinRead,
{
    type Args<'a> = M::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        M::read_options(reader, endian, args).map(|value| Self {
            pos: Cell::new(pos),
            value,
        })
    }
}

impl<M> BinWrite for Marker<M>
where
    M: BinWrite + Default,
{
    type Args<'a> = M::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.pos.set(writer.stream_position()?);
        M::default().write_options(writer, endian, args)
    }
}

pub struct MarkerMetadata<'m, M, T> {
    marker: &'m Marker<M>,
    value: T,
    metadata: M,
}

impl<'m, M, T> BinWrite for MarkerMetadata<'m, M, T>
where
    T: BinWrite,
    for<'a> M: BinWrite<Args<'a> = ()>,
{
    type Args<'a> = T::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        // Write value
        self.value.write_options(writer, endian, args)?;
        let pos_after = writer.stream_position()?;

        // Write metadata at marker
        writer.seek(SeekFrom::Start(self.marker.pos.get()))?;
        self.metadata.write_options(writer, endian, ())?;

        // Return back to after value
        writer.seek(SeekFrom::Start(pos_after))?;
        Ok(())
    }
}

pub struct MarkerOffset<'m, M, T> {
    marker: &'m Marker<M>,
    value: T,
}

impl<'m, M, T> AsRef<T> for MarkerOffset<'m, M, T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<'m, M, T> BinWrite for MarkerOffset<'m, M, T>
where
    T: BinWrite,
    for<'a> M: BinWrite<Args<'a> = ()> + TryFrom<u64>,
    <M as TryFrom<u64>>::Error: Display,
{
    type Args<'a> = T::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        // Write value
        let pos_before = writer.stream_position()?;
        self.value.write_options(writer, endian, args)?;
        let pos_after = writer.stream_position()?;

        // Write position at marker
        let pos_marker = self.marker.pos.get();
        writer.seek(SeekFrom::Start(pos_marker))?;
        M::try_from(pos_before)
            .map_err(|e| Error::Custom {
                pos: pos_marker,
                err: Box::new(format!(
                    "Offset {pos_before:x} can't be stored in a marker: {e}"
                )),
            })?
            .write_options(writer, endian, ());

        // Return back to after value
        writer.seek(SeekFrom::Start(pos_after))?;
        Ok(())
    }
}

pub struct MarkerSize<'m, M, T> {
    marker: &'m Marker<M>,
    value: T,
}

impl<'m, M, T> BinWrite for MarkerSize<'m, M, T>
where
    T: BinWrite,
    for<'a> M: BinWrite<Args<'a> = ()> + TryFrom<u64>,
    <M as TryFrom<u64>>::Error: Display,
{
    type Args<'a> = T::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        // Write value
        let pos_before = writer.stream_position()?;
        self.value.write_options(writer, endian, args)?;
        let pos_after = writer.stream_position()?;
        let size = pos_after - pos_before;

        // Write size at marker
        let pos_marker = self.marker.pos.get();
        writer.seek(SeekFrom::Start(pos_marker))?;
        M::try_from(size)
            .map_err(|e| Error::Custom {
                pos: pos_marker,
                err: Box::new(format!("Size {size:x} can't be stored in a marker: {e}")),
            })?
            .write_options(writer, endian, ());

        // Return back to after value
        writer.seek(SeekFrom::Start(pos_after))?;
        Ok(())
    }
}

pub trait StoreAtMarker
where
    Self: Sized,
{
    fn store_offset<'m, M>(self, marker: &'m Marker<M>) -> MarkerOffset<'m, M, Self> {
        MarkerOffset {
            marker,
            value: self,
        }
    }

    fn store_size<'m, M>(self, marker: &'m Marker<M>) -> MarkerSize<'m, M, Self> {
        MarkerSize {
            marker,
            value: self,
        }
    }

    fn store_metadata<'m, M>(
        self,
        marker: &'m Marker<M>,
        metadata: M,
    ) -> MarkerMetadata<'m, M, Self> {
        MarkerMetadata {
            marker,
            value: self,
            metadata,
        }
    }
}

impl<T> StoreAtMarker for T where T: BinWrite {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn marker_read_stores_position() -> eyre::Result<()> {
        #[binread]
        struct Data {
            before: [u8; 2],
            marker: Marker<u8>,
            after: [u8; 3],
        }

        let data = Data::read_le(&mut Cursor::new([
            // Some data before
            111, 186, // Marker
            71,  // Some data after
            222, 104, 235,
        ]))?;

        assert_eq!(data.marker.pos.get(), 2);
        assert_eq!(data.marker.value, 71);

        Ok(())
    }

    #[test]
    fn marker_write_stores_position() -> eyre::Result<()> {
        #[binwrite]
        struct Data {
            before: [u8; 2],
            marker: Marker<u8>,
            after: [u8; 3],
        }

        let data = Data {
            before: [111, 186],
            marker: Default::default(),
            after: [222, 104, 235],
        };

        let mut output = Cursor::new(vec![]);
        data.write_le(&mut output)?;

        assert_eq!(output.into_inner(), [111, 186, 0, 222, 104, 235]);
        assert_eq!(data.marker.pos.get(), 2);

        Ok(())
    }

    #[test]
    fn store_at_marker_stores_metadata() -> eyre::Result<()> {
        #[binwrite]
        struct Data {
            before: [u8; 2],
            marker: Marker<u8>,
            after: [u8; 3],
            #[bw(map = |d| d.store_metadata(marker, 60))]
            marked_data: [u8; 4],
        }

        let data = Data {
            before: [111, 186],
            marker: Default::default(),
            after: [222, 104, 235],
            marked_data: [61, 78, 150, 240],
        };

        let mut output = Cursor::new(vec![]);
        data.write_le(&mut output)?;

        assert_eq!(
            output.into_inner(),
            [111, 186, 60, 222, 104, 235, 61, 78, 150, 240]
        );

        Ok(())
    }

    #[test]
    fn store_at_marker_stores_offset() -> eyre::Result<()> {
        #[binwrite]
        struct Data {
            before: [u8; 2],
            marker: Marker<u8>,
            after: [u8; 3],
            #[bw(map = |d| d.store_offset(marker))]
            marked_data: [u8; 4],
        }

        let data = Data {
            before: [111, 186],
            marker: Default::default(),
            after: [222, 104, 235],
            marked_data: [61, 78, 150, 240],
        };

        let mut output = Cursor::new(vec![]);
        data.write_le(&mut output)?;

        assert_eq!(
            output.into_inner(),
            [111, 186, 6, 222, 104, 235, 61, 78, 150, 240]
        );

        Ok(())
    }

    #[test]
    fn store_at_marker_stores_size() -> eyre::Result<()> {
        #[binwrite]
        struct Data {
            before: [u8; 2],
            marker: Marker<u8>,
            after: [u8; 3],
            #[bw(map = |d| d.store_size(marker))]
            marked_data: [u8; 4],
        }

        let data = Data {
            before: [111, 186],
            marker: Default::default(),
            after: [222, 104, 235],
            marked_data: [61, 78, 150, 240],
        };

        let mut output = Cursor::new(vec![]);
        data.write_le(&mut output)?;

        assert_eq!(
            output.into_inner(),
            [111, 186, 4, 222, 104, 235, 61, 78, 150, 240]
        );

        Ok(())
    }
}
