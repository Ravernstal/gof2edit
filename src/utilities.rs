use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Seek, SeekFrom};

pub(crate) fn read_u32_list(source: &mut impl ReadBytesExt) -> io::Result<Vec<u32>> {
    let count = source.read_u32::<BigEndian>()?;
    let count = count.try_into().map_err(|_| ErrorKind::InvalidData)?;
    let mut list = Vec::with_capacity(count);

    for _ in 0..count {
        list.push(source.read_u32::<BigEndian>()?);
    }

    Ok(list)
}

pub(crate) fn write_u32_list(destination: &mut impl WriteBytesExt, list: &[u32]) -> io::Result<()> {
    let length = list.len().try_into().map_err(|_| ErrorKind::InvalidData)?;
    destination.write_u32::<BigEndian>(length)?;

    list.iter()
        .try_for_each(|x| destination.write_u32::<BigEndian>(*x))?;

    Ok(())
}

pub fn set_byte(file: &mut File, address: u64, byte: u8) -> io::Result<()> {
    file.seek(SeekFrom::Start(address))?;
    file.write_u8(byte)
}
