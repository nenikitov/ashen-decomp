use std::io::{Read, Seek, SeekFrom};

use binrw::{args, prelude::*, Endian, NullString};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct PackFileEntry {
    #[br(assert(asset_kind == 0))]
    asset_kind: u32,

    offset: u32,

    #[brw(pad_after = 4)]
    size: u32,
}

macro_rules! args_of {
    ($ty:ty) => (
        <$ty as BinRead>::Args<'_>
    )
}

fn offset_args_iter<Reader, T, Arg, Ret, It>(
    it: It,
) -> impl FnOnce(&mut Reader, Endian, ()) -> BinResult<Ret>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    Reader: Read + Seek,
    Arg: Clone,
    Ret: FromIterator<T>,
    It: IntoIterator<Item = (SeekFrom, Arg)>,
{
    move |reader, endian, args| {
        let base_pos = reader.stream_position()?;

        it.into_iter()
            .map(move |(ptr, arg)| {
                match ptr {
                    seek @ SeekFrom::Current(offset) => {
                        if let Some(new_pos) = base_pos.checked_add_signed(offset) {
                            if new_pos != reader.stream_position()? {
                                reader.seek(SeekFrom::Start(new_pos))?;
                            }
                        } else {
                            reader.seek(seek)?;
                        }
                    }
                    seek => {
                        reader.seek(seek)?;
                    }
                }

                T::read_options(reader, endian, arg)
            })
            .collect()
    }
}

pub struct Pointed<T>(T)
where
    T: BinRead + 'static,
    for<'a> T::Args<'a>: Clone;

#[derive(binrw::NamedArgs, Clone)]
pub struct PointedArgs<Inner: Clone> {
    pub offset: SeekFrom,
    pub inner: Inner,
}

impl<T> BinRead for Pointed<T>
where
    T: BinRead + 'static,
    for<'a> T::Args<'a>: Clone,
{
    type Args<'a> = PointedArgs<T::Args<'a>>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        reader.seek(args.offset);
        T::read_options(reader, endian, args.inner).map(Pointed)
    }
}

#[binread]
#[br(little, magic = b"PMAN")]
#[derive(Debug)]
pub struct PackFile {
    #[br(temp)]
    #[bw(try_calc(u32::try_from(_entries.len())))]
    _file_count: u32,

    #[br(pad_size_to = 56)]
    copyright: NullString,

    #[br(temp, count = _file_count)]
    _entries: Vec<PackFileEntry>,

    #[br(
        parse_with = offset_args_iter(_entries.iter().map(|e| -> (_, args_of!(Vec<u8>)) {(
            SeekFrom::Start(e.offset as u64),
            args!{ count: e.size as usize }
        )}))
    )]
    entries: Vec<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use crate::utils::test::*;

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn new_packfile() {
        let mut file =
            File::open(workspace_file_path!("rom/packfile.dat")).expect("ROM file is present");
        let file = PackFile::read(&mut file).unwrap();
        for e in file.entries {
            println!("----");
            println!("head: {:x?}", &e[..e.len().min(10)]);
            println!("len: {}", e.len());
        }
    }
}
