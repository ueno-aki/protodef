use anyhow::{Result, Ok};
use byteorder::{BigEndian, LittleEndian,WriteBytesExt};

pub trait Write {
    fn write_u8(&mut self, value: u8) -> Result<()>;
    fn write_i8(&mut self, value: i8) -> Result<()>;
    fn write_i16(&mut self, value: i16) -> Result<()>;
    fn write_li16(&mut self, value: i16) -> Result<()>;
    fn write_u16(&mut self, value: u16) -> Result<()>;
    fn write_lu16(&mut self, value: u16) -> Result<()>;
    fn write_u32(&mut self, value: u32) -> Result<()>;
    fn write_lu32(&mut self, value: u32) -> Result<()>;
    fn write_i32(&mut self, value: i32) -> Result<()>;
    fn write_u64(&mut self, value: u64) -> Result<()>;
    fn write_lu64(&mut self, value: u64) -> Result<()>;
    fn write_lf32(&mut self, value: f32) -> Result<()>;
    fn write_var_int(&mut self, value: u64) -> Result<u64>;
    fn write_bool(&mut self, value: bool) -> Result<()>;
    fn write_string(&mut self, value: &str) -> Result<u64>;
}
impl Write for Vec<u8> {
    #[inline]
    fn write_u8(&mut self, value: u8) -> Result<()> {
        WriteBytesExt::write_u8(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_i8(&mut self, value: i8) -> Result<()> {
        WriteBytesExt::write_i8(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_i16(&mut self, value: i16) -> Result<()> {
        WriteBytesExt::write_i16::<BigEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_li16(&mut self, value: i16) -> Result<()> {
        WriteBytesExt::write_i16::<LittleEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_u16(&mut self, value: u16) -> Result<()> {
        WriteBytesExt::write_u16::<BigEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_lu16(&mut self, value: u16) -> Result<()> {
        WriteBytesExt::write_u16::<LittleEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_u32(&mut self, value: u32) -> Result<()> {
        WriteBytesExt::write_u32::<BigEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_lu32(&mut self, value: u32) -> Result<()> {
        WriteBytesExt::write_u32::<LittleEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_i32(&mut self, value: i32) -> Result<()> {
        WriteBytesExt::write_i32::<BigEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_u64(&mut self, value: u64) -> Result<()> {
        WriteBytesExt::write_u64::<BigEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_lu64(&mut self, value: u64) -> Result<()> {
        WriteBytesExt::write_u64::<LittleEndian>(self, value)?;
        Ok(())
    }
    #[inline]
    fn write_lf32(&mut self, value: f32) -> Result<()> {
        WriteBytesExt::write_f32::<LittleEndian>(self, value)?;
        Ok(())
    }
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
