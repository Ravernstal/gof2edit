use crate::result::Result;
use byteorder::ByteOrder;
use std::io::Read;

pub trait BinRead: Sized {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self>;
}

pub trait BinReader<T> {
    fn read_bin<O: ByteOrder>(&mut self) -> Result<T>;
}

impl<R, T> BinReader<T> for R
where
    R: Read,
    T: BinRead,
{
    fn read_bin<O: ByteOrder>(&mut self) -> Result<T> {
        T::read_bin::<O>(self)
    }
}
