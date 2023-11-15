use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Invalid index ,expected at most {0}")]
    OutOfBounds(usize),
    #[error("varint is too big:{0} > 63")]
    VarIntTooBig(u8),
    #[error("Missing characters in string, at most {0}, found size {1}")]
    MissingCharacters(usize, usize),
    #[error("Failed to parse {0} into bool")]
    FailedIntoBoolean(u8),
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
            fn read_i8(&self, offset: usize) -> i8;
        }
        impl NativeReader for &[u8] {
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
            #[inline]
            fn read_i8(&self, offset: usize) -> i8 {
                self[offset] as i8
            }
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
            #[inline]
            fn read_i8(&self, offset: usize) -> i8 {
                self[offset] as i8
            }
        }
    };
}
native_reader![u16, u32, u64, i16, i32, i64, f32, f64];

pub trait ProtodefReader {
    fn read_varint(&self, offset: usize) -> Result<(u64, usize)>;
    fn read_string(&self, offset: usize) -> Result<(String, usize)>;
    fn read_little_string(&self, offset: usize) -> Result<(String, usize)>;
    fn read_zigzag32(&self, offset: usize) -> Result<(i32, usize)>;
    fn read_zigzag64(&self, offset: usize) -> Result<(i64, usize)>;
    fn read_bool(&self, offset: usize) -> Result<(bool, usize)>;
}

macro_rules! impl_protodef_reader {
    ($($vec:ty),+) => {
        $(
            impl ProtodefReader for $vec {
                fn read_varint(&self, offset: usize) -> Result<(u64, usize)> {
                    let mut value: u64 = 0;
                    let mut shift: u8 = 0;
                    let mut cursor = offset;
                    loop {
                        let byte = self[cursor] as u64;
                        value |= (byte & 0x7f) << shift;
                        cursor += 1;
                        shift += 7;
                        if (byte & 0x80) == 0 {
                            break Ok((value, cursor - offset));
                        } else if shift > 63 {
                            break Err(ReadError::VarIntTooBig(shift).into());
                        } else if (cursor + 1) > self.len() {
                            break Err(ReadError::OutOfBounds(self.len()).into());
                        }
                    }
                }
                fn read_zigzag32(&self, offset: usize) -> Result<(i32, usize)> {
                    let (value, size) = self.read_varint(offset)?;
                    let sign = (-1 * (value & 1) as i32) as u32 as u64;
                    let value = (value >> 1) ^ sign;
                    Ok((value as i32, size))
                }
                fn read_zigzag64(&self, offset: usize) -> Result<(i64, usize)> {
                    let (value, size) = self.read_varint(offset)?;
                    let sign = (-1 * (value & 1) as i64) as u64;
                    let value = (value >> 1) ^ sign;
                    Ok((value as i64, size))
                }
                fn read_bool(&self, offset: usize) -> Result<(bool, usize)> {
                    match self[offset] {
                        n if n == 0 => Ok((false, 1)),
                        n if n == 1 => Ok((true, 1)),
                        _ => Err(ReadError::FailedIntoBoolean(self[offset]).into()),
                    }
                }
                fn read_string(&self, offset: usize) -> Result<(String, usize)> {
                    let mut cursor = offset;
                    let (str_size, size) = self.read_varint(cursor)?;
                    cursor += size;
                    let edge = cursor + str_size as usize;
                    if edge > self.len() {
                        return Err(ReadError::MissingCharacters(self.len(), edge).into());
                    }
                    let str = String::from_utf8(self[cursor..edge].to_vec())?;
                    Ok((str, edge - offset))
                }
                fn read_little_string(&self, offset: usize) -> Result<(String, usize)> {
                    let mut cursor = offset;
                    let str_size = self.read_li32(cursor) as usize;
                    cursor += 4;
                    let edge = cursor + str_size;
                    if edge > self.len() {
                        return Err(ReadError::MissingCharacters(self.len(), edge).into());
                    }
                    let str = String::from_utf8(self[cursor..edge].to_vec())?;
                    Ok((str, 4 + str_size))
                }
            }
        )*
    };
}
impl_protodef_reader!(Vec<u8>,&[u8]);