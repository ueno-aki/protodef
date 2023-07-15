use anyhow::Result;
use byteorder::WriteBytesExt;

pub trait Write {
    fn write_var_int(&mut self, value: u64) -> Result<u64>;
}
impl Write for Vec<u8> {
    fn write_var_int(&mut self, value: u64) -> Result<u64> {
        let mut cursor: u64 = 0;
        let mut v: u64 = value;
        while (v & !0x7f) != 0 {
            self.write_u8(((v & 0xff) | 0x80) as u8)?;
            cursor += 1;
            v >>= 7;
        }
        self.write_u8(v as u8)?;
        cursor += 1;
        Ok(cursor)
    }
}
