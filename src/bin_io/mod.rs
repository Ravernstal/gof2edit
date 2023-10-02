use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{ErrorKind, Read, Write};

pub mod read;
pub mod write;

impl BinRead for String {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let length = source.read_u16::<O>()?;
        let length = length.try_into().map_err(|_| ErrorKind::InvalidData)?;
        let mut string = vec![0u8; length];
        source.read_exact(&mut string)?;

        Ok(String::from_utf8(string).map_err(|_| ErrorKind::InvalidData)?)
    }
}

impl BinWrite for String {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        let data = self.as_bytes();
        let length = data.len().try_into().map_err(|_| ErrorKind::InvalidData)?;

        destination.write_u16::<O>(length)?;
        destination.write_all(data)
    }
}

impl BinRead for Vec<u8> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into().map_err(|_| ErrorKind::InvalidData)?;

        (0..length)
            .map(|_| source.read_u8())
            .collect::<io::Result<Vec<_>>>()
    }
}

impl BinWrite for Vec<u8> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        let length = self.len().try_into().map_err(|_| ErrorKind::InvalidData)?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_u8(*x))
    }
}

impl BinRead for Vec<u32> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into().map_err(|_| ErrorKind::InvalidData)?;

        (0..length)
            .map(|_| source.read_u32::<O>())
            .collect::<io::Result<Vec<_>>>()
    }
}

impl BinWrite for Vec<u32> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        let length = self.len().try_into().map_err(|_| ErrorKind::InvalidData)?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_u32::<O>(*x))
    }
}
