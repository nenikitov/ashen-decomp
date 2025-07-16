use std::io::{Read, Seek, Write};

use derive_more::{Deref, From, Into};
use glam::Vec3;

use super::*;
use crate::utils::math::IntoFromNormalizedF32;

#[derive(Debug, Deref, Clone, From, Into)]
pub struct ColorU16(Vec3);

impl BinRead for ColorU16 {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        let color = u16::read_options(reader, endian, ())?;
        if color > 0xFFF {
            return Err(Error::AssertFail {
                pos,
                message: "12 bit color outside the range".to_string(),
            });
        }

        macro_rules! isolate {
            ($offset: expr) => {
                ((color & (0xFu16 << 4 * $offset)) >> 4 * $offset)
                    .into_normalized_f32_between(0, 15, true)
            };
        }

        Ok(Self(Vec3::new(isolate!(2), isolate!(1), isolate!(0))))
    }
}

impl BinWrite for ColorU16 {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        macro_rules! isolate {
            ($channel: ident, $offset: expr) => {
                (u16::from_normalized_f32_between(self.$channel, 0, 15, true) << 4 * $offset)
            };
        }

        (isolate!(x, 2) | isolate!(y, 1) | isolate!(z, 0)).write_options(writer, endian, ())
    }
}
