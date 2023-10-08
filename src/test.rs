use crate::prelude::*;
#[test]
pub fn io_varint() {
    let number: u64 = 12345679;
    let mut vec: Vec<u8> = Vec::new();
    vec.write_varint(number).unwrap();
    let (value, _size) = vec.read_varint(0).unwrap();
    assert_eq!(number, value);
}
#[test]
pub fn string() {
    let str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let mut vec: Vec<u8> = Vec::new();
    vec.write_string(str).unwrap();
    let (value, _size) = vec.read_string(0).unwrap();
    assert_eq!(str, value);
}
#[test]
pub fn zigzag32() {
    let i = i32::MIN;
    let mut vec: Vec<u8> = Vec::new();
    vec.write_zigzag32(i).unwrap();
    let (value, _size) = vec.read_zigzag32(0).unwrap();
    assert_eq!(i, value);
}

#[test]
pub fn zigzag64() {
    let i = i64::MIN;
    let mut vec: Vec<u8> = Vec::new();
    vec.write_zigzag64(i).unwrap();
    let (value, _size) = vec.read_zigzag64(0).unwrap();
    assert_eq!(i, value);
}
