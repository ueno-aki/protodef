use anyhow::{Result, anyhow};

macro_rules! native_writer {
    ($($native:ty),*) => {
        use paste::paste;
        use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
        pub trait NativeWriter {
            $(
                paste! {
                    fn [<write_ $native>](&mut self, value: $native) -> Result<()>;
                    fn [<write_l $native>](&mut self, value: $native) -> Result<()>;
                }
            )*
            fn write_u8(&mut self, value: u8) -> Result<()>;
            fn write_i8(&mut self, value: i8) -> Result<()>;
        }
        impl NativeWriter for Vec<u8> {
            $(
                paste! {
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
native_writer!(u16,u32,u64,i16,i32,i64,f32,f64);

pub trait ProtodefWriter {
    fn write_var_int(&mut self, value: u64) -> Result<u64>;
    fn write_bool(&mut self, value: bool) -> Result<()>;
    fn write_string(&mut self, value: &str) -> Result<u64>;
}
impl ProtodefWriter for Vec<u8> {
    #[inline]
    fn write_var_int(&mut self, value: u64) -> Result<u64> {
        let mut cursor: u64 = 0;
        let mut v: u64 = value;
        while (v & !0x7f) != 0 {
            WriteBytesExt::write_u8(self, ((v & 0xff) | 0x80) as u8)?;
            cursor += 1;
            v >>= 7;
        }
        WriteBytesExt::write_u8(self, v as u8)?;
        cursor += 1;
        Ok(cursor)
    }
    #[inline]
    fn write_bool(&mut self, value: bool) -> Result<()> {
        WriteBytesExt::write_i8(self, value.into())?;
        Ok(())
    }
    #[inline]
    fn write_string(&mut self, value: &str) -> Result<u64> {
        let mut cursor = 0;
        let len = value.as_bytes().len() as u64;
        cursor += self.write_var_int(len)?;
        self.append(&mut value.as_bytes().to_vec());
        Ok(cursor + len)
    }
}
