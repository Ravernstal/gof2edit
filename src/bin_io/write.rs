use crate::result::Result;
use byteorder::ByteOrder;
use std::io::Write;

pub trait BinWrite {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()>;
}

pub trait BinWriter<T>: Sized {
    fn write_bin<O: ByteOrder>(&mut self, data: &T) -> Result<()>;
}

impl<W, T> BinWriter<T> for W
where
    W: Write,
    T: BinWrite,
{
    fn write_bin<O: ByteOrder>(&mut self, data: &T) -> Result<()> {
        T::write_bin::<O>(data, self)
    }
}
