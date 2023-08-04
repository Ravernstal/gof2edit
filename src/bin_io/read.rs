use byteorder::ByteOrder;
use std::io;
use std::io::Read;

pub trait BinRead: Sized {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self>;
}

pub trait BinReader<T> {
    fn read_bin<O: ByteOrder>(&mut self) -> io::Result<T>;
}

impl<R, T> BinReader<T> for R
where
    R: Read,
    T: BinRead,
{
    fn read_bin<O: ByteOrder>(&mut self) -> io::Result<T> {
        T::read_bin::<O>(self)
    }
}
