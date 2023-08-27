use crate::bin_io::read::BinRead;
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::index::Index;
use byteorder::BigEndian;
use std::io;
use std::io::{Read, Write};

pub mod bin_io;
pub mod data;
pub mod index;

pub fn read_object_list<T: BinRead>(source: &mut impl Read) -> io::Result<Vec<T>> {
    let mut objects = vec![];

    while let Ok(object) = T::read_bin::<BigEndian>(source) {
        objects.push(object);
    }

    Ok(objects)
}

pub fn read_object_list_indexed<T: BinRead + Index>(source: &mut impl Read) -> io::Result<Vec<T>> {
    let mut objects = vec![];
    let mut index = 0;

    while let Ok(mut object) = T::read_bin::<BigEndian>(source) {
        object.set_index(index);
        objects.push(object);
        index += 1;
    }

    Ok(objects)
}

pub fn write_object_list<T: BinWrite + Index>(
    destination: &mut impl Write,
    mut objects: Vec<T>,
) -> io::Result<usize> {
    objects.sort_unstable_by_key(|object| object.index());

    objects
        .iter()
        .try_for_each(|object| destination.write_bin::<BigEndian>(object))?;

    Ok(objects.len())
}
