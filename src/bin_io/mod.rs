use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

pub mod read;
pub mod write;

impl BinRead for String {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u16::<O>()?;
        let length = length.into();
        let mut string = vec![0u8; length];
        source.read_exact(&mut string)?;

        Ok(String::from_utf8(string)?)
    }
}

impl BinWrite for String {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let data = self.as_bytes();
        let length = data.len().try_into()?;

        destination.write_u16::<O>(length)?;
        destination.write_all(data)?;

        Ok(())
    }
}

impl BinRead for Vec<Vec<i32>> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        (0..length)
            .map(|_| source.read_bin::<O>())
            .collect::<Result<Vec<_>>>()
    }
}

impl BinWrite for Vec<Vec<i32>> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_bin::<O>(x))
    }
}

impl BinRead for Vec<bool> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let bytes = Vec::<u8>::read_bin::<O>(source)?;

        Ok(bytes.into_iter().map(|x| x != 0).collect())
    }
}

impl BinWrite for Vec<bool> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let bytes = self.iter().map(|x| u8::from(*x)).collect::<Vec<_>>();

        destination.write_bin::<O>(&bytes)
    }
}

impl BinRead for Vec<u8> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        let list = (0..length)
            .map(|_| source.read_u8())
            .collect::<io::Result<Vec<_>>>()?;

        Ok(list)
    }
}

impl BinWrite for Vec<u8> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_u8(*x))?;

        Ok(())
    }
}

impl BinRead for Vec<i32> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        let list = (0..length)
            .map(|_| source.read_i32::<O>())
            .collect::<io::Result<Vec<_>>>()?;

        Ok(list)
    }
}

impl BinWrite for Vec<i32> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|x| destination.write_i32::<O>(*x))?;

        Ok(())
    }
}

impl BinRead for Vec<u32> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        let list = (0..length)
            .map(|_| source.read_u32::<O>())
            .collect::<io::Result<Vec<_>>>()?;

        Ok(list)
    }
}

impl BinWrite for Vec<u32> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|x| destination.write_u32::<O>(*x))?;

        Ok(())
    }
}
