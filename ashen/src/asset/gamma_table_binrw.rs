use std::ops::Deref;

use binrw::{BinRead, BinWrite, binrw};

const LEN_ROWS: usize = 256;
const LEN_COLS: usize = 101;

#[binrw]
#[derive(Debug)]
pub struct NormalizedF32From<T>(
    #[br(map = |x: T| x.to_f32().unwrap() / T::max_value().to_f32().unwrap())]
    #[bw(map = |x| T::from((x * T::max_value().to_f32().unwrap()).floor()))]
    f32,
    #[ignore] std::marker::PhantomData<T>,
)
where
    T: num::Bounded
        + num::NumCast
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>;

impl<T> Deref for NormalizedF32From<T>
where
    T: num::Bounded
        + num::NumCast
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>,
{
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct GammaTable {
    #[br(args {
        count: LEN_ROWS,
        inner: binrw::args! { count: LEN_COLS, inner: () },
    })]
    #[bw()]
    pub gamma: Vec<Vec<NormalizedF32From<u8>>>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const GAMMA_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("00.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let gamma_table = GammaTable::read(&mut Cursor::new(GAMMA_TABLE_DATA.as_slice()))?;

        dbg!(gamma_table);

        Ok(())
    }
}
