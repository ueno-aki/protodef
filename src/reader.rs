use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Invalid index ,expected at most {0}")]
    OutOfBounds(usize),
    #[error("Missing characters in string, found size is {0}, expected size was {1}")]
    MissingCharacters(usize, usize),
    #[error("Failed to parse {0} into bool")]
    FailedIntoBoolean(u8)
}
macro_rules! native_reader {
    ($($native:ty),*) => {
        use byteorder::{BigEndian, LittleEndian, ByteOrder};
        pub trait NativeReader {
            $(
                paste::paste! {
                    fn [<read_ $native>](&self, offset: usize) -> $native;
                    fn [<read_l $native>](&self, offset: usize) -> $native;
                }
            )*
            fn read_u8(&self, offset: usize) -> u8;
        }
        impl NativeReader for Vec<u8> {
            $(
                paste::paste! {
                    #[inline]
                    fn [<read_ $native>](&self, offset: usize) -> $native {
                        BigEndian::[<read_$native>](&self[offset..])
                    }
                    #[inline]
                    fn [<read_l $native>](&self, offset: usize) -> $native {
                        LittleEndian::[<read_$native>](&self[offset..])
                    }
                }
            )*
            #[inline]
            fn read_u8(&self, offset: usize) -> u8 {
                self[offset]
            }
        }
    };
}
native_reader![u16,u32,u64,i16,i32,i64,f32,f64];

pub trait ProtodefReader {
    fn read_varint(&self, offset: usize) -> Result<(u64, usize)>;
    fn read_bool(&self, offset: usize) -> Result<(bool,usize)>;
    fn read_little_string(&self, offset: usize) -> Result<(String, usize)>;
}
impl ProtodefReader for Vec<u8> {
    #[inline]
    fn read_varint(&self, offset: usize)->Result<(u64,usize)> {
        let mut value: u64 = 0;
        let mut shift: u64 = 0;
        let mut cursor = offset;
        loop {
            if (cursor + 1) > self.len(){
                break Err(ReadError::OutOfBounds(self.len()).into());
            }
            let byte = self[cursor] as u64;
            value |= (byte & 0x7f) << shift;
            cursor += 1;
            if (byte & 0x80) == 0 {
                break Ok((value, cursor - offset));
            }
            shift += 7;
        }
    }
    #[inline]
    fn read_bool(&self, offset: usize) -> Result<(bool,usize)> {
        match self[offset] {
            n if n == 0 => Ok((false,1)),
            n if n == 1 => Ok((true,1)),
            _ => Err(ReadError::FailedIntoBoolean(self[offset]).into())
        }
    }
    #[inline]
    fn read_little_string(&self, offset: usize) -> Result<(String, usize)> {
        let mut cursor = offset;
        let str_size = self.read_li32(offset) as usize;
        cursor += 4;
        let edge = cursor + str_size;
        if edge > self.len() {
            return Err(ReadError::MissingCharacters(self.len(), edge).into());
        }
        let str = String::from_utf8(self[cursor..edge].to_vec())?;
        cursor += str_size;
        Ok((str, cursor))
    }
}