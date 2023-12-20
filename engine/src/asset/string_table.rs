use super::{Asset, Extension, Kind};
use crate::{error, utils::nom::*};

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

impl Asset for StringTable {
    fn kind() -> Kind {
        Kind::StringTable
    }

    fn parse(input: &[u8], extension: super::Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (input, count) = number::le_u32(input)?;
                // TODO(Unavailable): Find out what the "catholic" characters are.
                let (input, table) = multi::count!(utf_16_string, count as usize)(input)?;

                Ok((input, StringTable { table }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[ignore = "uses files that are local"]
    fn parse_works() -> eyre::Result<()> {
        let bytes = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../output/deflated/DA5B9C.dat"
        ))?;

        let (_, string_table) = StringTable::parse(&bytes, Extension::Dat)?;

        fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../output/strings/DA5B9C.txt"),
            string_table.table.join("\n"),
        )?;

        Ok(())
    }
}
