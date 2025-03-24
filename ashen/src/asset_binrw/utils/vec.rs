use std::io::{Read, Seek, Write};

use super::*;

pub fn map_vec_parse<F, Reader, T, O, Arg>(
    map: F,
) -> impl Fn(&mut Reader, Endian, VecArgs<Arg>) -> BinResult<Vec<O>>
where
    F: Fn(T) -> O,
    Reader: Read + Seek,
    T: for<'a> BinRead<Args<'a> = Arg>,
    Arg: Clone,
{
    move |reader, endian, VecArgs { count, inner }| {
        (0..count)
            .map(|_| Ok(map(T::read_options(reader, endian, inner.clone())?)))
            .collect()
    }
}

pub fn map_vec2_parse<F, Reader, T, O, Arg>(
    map: F,
) -> impl Fn(&mut Reader, Endian, VecArgs<VecArgs<Arg>>) -> BinResult<Vec<Vec<O>>>
where
    F: Copy + Fn(T) -> O,
    Reader: Read + Seek,
    T: for<'a> BinRead<Args<'a> = Arg>,
    Arg: Clone,
{
    move |reader, endian, VecArgs { count, inner }| {
        (0..count)
            .map(|_| map_vec_parse(map)(reader, endian, inner.clone()))
            .collect()
    }
}

pub fn map_vec_write<F, Writer, T, O>(
    map: F,
) -> impl Copy + Fn(&Vec<T>, &mut Writer, Endian, ()) -> BinResult<()>
where
    F: Copy + Fn(&T) -> O,
    Writer: Write + Seek,
    O: for<'a> BinWrite<Args<'a> = ()>,
{
    move |vec, writer, endian, ()| {
        vec.iter()
            .try_for_each(|e| map(e).write_options(writer, endian, ()))
    }
}

pub fn map_vec2_write<F, Writer, T, O>(
    map: F,
) -> impl Copy + Fn(&Vec<Vec<T>>, &mut Writer, Endian, ()) -> BinResult<()>
where
    F: Copy + Fn(&T) -> O,
    Writer: Write + Seek,
    O: for<'a> BinWrite<Args<'a> = ()>,
{
    move |vec, writer, endian, ()| {
        vec.iter()
            .try_for_each(|e| map_vec_write(map)(e, writer, endian, ()))
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
        itertools::zip_eq(elems.iter(), it)
            .try_for_each(|(e, arg)| e.write_options(writer, endian, arg))
    }
}
