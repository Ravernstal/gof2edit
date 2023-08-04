use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::{ErrorKind, Read, Write};

pub mod read;
pub mod write;

impl BinRead for String {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        let length = source.read_u16::<O>()?;
        let length = length.try_into().map_err(|_| ErrorKind::InvalidData)?;
        let mut string = vec![0u8; length];
        source.read_exact(&mut string)?;

        Ok(String::from_utf8(string).map_err(|_| ErrorKind::InvalidData)?)
    }
}

impl BinWrite for String {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        let data = self.as_bytes();
        let length = data.len().try_into().map_err(|_| ErrorKind::InvalidData)?;

        destination.write_u16::<O>(length)?;
        destination.write_all(data)
    }
}
