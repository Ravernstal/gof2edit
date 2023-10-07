use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{ErrorKind, Read, Write};

#[derive(Debug)]
pub struct WideString {
    string: String,
}

impl WideString {
    pub fn new(string: impl Into<String>) -> Self {
        Self {
            string: string.into(),
        }
    }

    pub fn get(self) -> String {
        self.string
    }
}

impl BinRead for WideString {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into().map_err(|_| ErrorKind::InvalidData)?;

        let string = (0..length)
            .map(|_| source.read_u16::<O>())
            .collect::<Result<Vec<_>, _>>()?;

        let string = String::from_utf16(&string).map_err(|_| ErrorKind::InvalidData)?;

        Ok(Self::new(string))
    }
}

impl BinWrite for WideString {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        let data = self.string.encode_utf16().collect::<Vec<_>>();
        let length = data.len().try_into().map_err(|_| ErrorKind::InvalidData)?;

        destination.write_u32::<O>(length)?;
        data.iter().try_for_each(|n| destination.write_u16::<O>(*n))
    }
}
