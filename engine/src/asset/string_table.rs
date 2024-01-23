use super::{extension::*, AssetParser};
use crate::utils::nom::*;

pub struct StringTable {
    table: Vec<String>,
}

fn utf_16_string(input: &[u8]) -> Result<String> {
    multi::many_till(number::le_u16, bytes::tag("\0\0"))(input).map(|(input, (bytes, _))| {
        (
            input,
            String::from_utf16(&bytes).expect("valid utf-16 string"),
        )
    })
}

impl AssetParser<Pack> for StringTable {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (input, count) = number::le_u32(input)?;
            // TODO(Unavailable): Find out what the "catholic" characters are.
            let (input, table) = multi::count!(utf_16_string, count as usize)(input)?;

            Ok((input, StringTable { table }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::*;
    use std::cell::LazyCell;

    const STRING_TABLE_DATA: LazyCell<Vec<u8>> = deflated_file!("98-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, string_table) = <StringTable as AssetParser<Pack>>::parser(())(&STRING_TABLE_DATA)?;

        output_file(
            parsed_file_path!("strings/english-uk.txt"),
            string_table.table.join("\n---\n"),
        )?;

        Ok(())
    }
}
