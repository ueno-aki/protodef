use anyhow::{Ok, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadError {
    #[error("Invalid index ,expected at most {0}")]
    OutOfBounds(usize),
    #[error("Missing characters in string, found size is {0}, expected size was {1}")]
    MissingCharacters(usize, u64),
}

pub trait Read {
    fn read_varint(&self, offset: u64)->Result<(u64,u64)>;
}
impl Read for Vec<u8> {
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
}