use anyhow::{Ok, Result};
use byteorder::{BigEndian, LittleEndian, ByteOrder};
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadError {
    #[error("Invalid index ,expected at most {0}")]
    OutOfBounds(usize),
    #[error("Missing characters in string, found size is {0}, expected size was {1}")]
    MissingCharacters(usize, u64),
}

pub trait Read {
    fn read_i32(&self, offset: u64) -> i32;
    fn read_li32(&self, offset: u64) -> i32;
    fn read_varint(&self, offset: u64) -> Result<(u64, u64)>;
    fn read_little_string(&self, offset: u64) -> Result<(String, u64)>;
}
impl Read for Vec<u8> {
    fn read_i32(&self, offset: u64) -> i32 {
        BigEndian::read_i32(&self[(offset as usize)..])
    }
    fn read_li32(&self, offset: u64) -> i32 {
        LittleEndian::read_i32(&self[(offset as usize)..])
    }
    fn read_varint(&self, offset: u64)->Result<(u64,u64)> {
        let mut value: u64 = 0;
        let mut shift: u8 = 0;
        let mut cursor = offset;
        loop {
            if (cursor + 1) > self.len() as u64{
                break Err(ReadError::OutOfBounds(self.len()).into());
            }
            let byte = self[cursor as usize];
            value |= (byte as u64 & 0x7f) << shift;
            cursor += 1;
            if (byte as u64 & 0x80) == 0 {
                break Ok((value, cursor - offset));
            }
            shift += 7;
        }
    }
    fn read_little_string(&self, offset: u64) -> Result<(String, u64)> {
        let mut cursor: u64 = offset;
        let value = self.read_li32(offset) as u64;
        cursor += 4;
        if cursor + value > self.len() as u64 {
            return Err(ReadError::MissingCharacters(self.len(), cursor + value).into());
        }
        let edge = cursor + value;
        let str = String::from_utf8(self[(cursor as usize)..(edge as usize)].to_vec()).unwrap();
        let size = 4 + value;
        Ok((str, size))
    }
}