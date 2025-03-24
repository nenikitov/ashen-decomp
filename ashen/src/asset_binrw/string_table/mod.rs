use super::utils::*;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct StringTable {
    #[br(temp)]
    #[bw(calc = strings.len() as u32)]
    _strings_len: u32,

    #[br(
        count = _strings_len,
        parse_with = map_vec_parse(|x: NullWideString| x.to_string())
    )]
    #[bw(write_with = map_vec_write(|x: &String| NullWideString::from(x.clone())))]
    strings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const STRING_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("98-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let string_table = StringTable::read(&mut Cursor::new(STRING_TABLE_DATA.as_slice()))?;
        Ok(())
    }
}
