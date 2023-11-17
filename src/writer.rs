use std::io::Write;

use anyhow::{anyhow, Result};

macro_rules! native_writer {
    ($($native:ty),*) => {
        use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
        pub trait NativeWriter {
            $(
                paste::paste! {
                    fn [<write_ $native>](&mut self, value: $native) -> Result<()>;
                    fn [<write_l $native>](&mut self, value: $native) -> Result<()>;
                }
            )*
            fn write_u8(&mut self, value: u8) -> Result<()>;
            fn write_i8(&mut self, value: i8) -> Result<()>;
        }
        impl NativeWriter for Vec<u8> {
            $(
                paste::paste! {
                    #[inline]
                    fn [<write_ $native>](&mut self, value: $native) -> Result<()> {
                        WriteBytesExt::[<write_ $native>]::<BigEndian>(self, value).map_err(|e|anyhow!(e))
                    }
                    #[inline]
                    fn [<write_l $native>](&mut self, value: $native) -> Result<()> {
                        WriteBytesExt::[<write_ $native>]::<LittleEndian>(self, value).map_err(|e|anyhow!(e))
                    }
                }
            )*
            #[inline]
            fn write_u8(&mut self, value: u8) -> Result<()> {
                WriteBytesExt::write_u8(self, value).map_err(|e|anyhow!(e))
            }
            #[inline]
            fn write_i8(&mut self, value: i8) -> Result<()> {
                WriteBytesExt::write_i8(self, value).map_err(|e|anyhow!(e))
            }
        }
    };
}
native_writer![u16, u32, u64, i16, i32, i64, f32, f64];

pub trait ProtodefWriter {
    fn write_varint(& mut self, value: u64) -> Result<usize>;
    fn write_string(& mut self, value: &str) -> Result<usize>;
    fn write_short_string(&mut self, value: &str) -> Result<usize>;
    fn write_zigzag32(& mut self, value: i32) -> Result<usize>;
    fn write_zigzag64(& mut self, value: i64) -> Result<usize>;
    fn write_bool(& mut self, value: bool) -> Result<()>;
}

impl ProtodefWriter for Vec<u8> {
    fn write_varint(&mut self, value: u64) -> Result<usize> {
        let mut cursor: usize = 0;
        let mut v = value;
        while (v & !0x7f) != 0 {
            WriteBytesExt::write_u8(self, (v & 0xff | 0x80) as u8)?;
            cursor += 1;
            v >>= 7;
        }
        WriteBytesExt::write_u8(self, v as u8)?;
        cursor += 1;
        Ok(cursor)
    }
    fn write_string(&mut self, value: &str) -> Result<usize> {
        let mut cursor = 0;
        let len = value.as_bytes().len();
        cursor += self.write_varint(len as u64)?;
        self.write(&value.as_bytes().to_vec())?;
        Ok(cursor + len)
    }
    fn write_short_string(&mut self, value: &str) -> Result<usize> {
        let len = value.as_bytes().len();
        self.write_li16(len as i16)?;
        self.write(&value.as_bytes().to_vec())?;
        Ok(len + 2)
    }
    #[inline]
    /// 32bit Signed VarInt
    fn write_zigzag32(&mut self, value: i32) -> Result<usize> {
        let v = (value >> 31) ^ (value << 1);
        Ok(self.write_varint(v as u32 as u64)?)
    }
    #[inline]
    /// 64bit Signed VarInt
    fn write_zigzag64(&mut self, value: i64) -> Result<usize> {
        let v = (value >> 63) ^ (value << 1);
        Ok(self.write_varint(v as u64)?)
    }
    #[inline]
    fn write_bool(&mut self, value: bool) -> Result<()> {
        WriteBytesExt::write_i8(self, value as i8)?;
        Ok(())
    }
}