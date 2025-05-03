use super::Parser;
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

impl Parser for StringTable {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
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
    use std::cell::LazyCell;

    use super::*;
    use crate::utils::test::*;

    const STRING_TABLES: LazyCell<Vec<(&str, Vec<u8>)>> = LazyCell::new(|| {
        vec![
            ("english-uk", deflated_file!("98-deflated.dat")),
            ("english-us", deflated_file!("99-deflated.dat")),
            ("french", deflated_file!("9A-deflated.dat")),
            ("italian", deflated_file!("9B-deflated.dat")),
            ("german", deflated_file!("9C-deflated.dat")),
            ("spanish", deflated_file!("9D-deflated.dat")),
        ]
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_string_tables() -> eyre::Result<()> {
        for (name, data) in STRING_TABLES.iter() {
            let (_, string_table) = StringTable::parser(())(data)?;

            output_file(
                PARSED_PATH.join(format!("string/{name}.txt")),
                string_table.table.join("\n---\n"),
            )?;
        }

        Ok(())
    }
}
