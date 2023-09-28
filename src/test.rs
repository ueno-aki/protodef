use crate::prelude::*;
#[test]
pub fn io_varint() {
    let number:u64 = 12345679;
    let mut vec:Vec<u8> = Vec::new();
    vec.write_var_int(number).unwrap();
    let (value,_size) = vec.read_varint(0).unwrap();
    assert_eq!(number,value);
}
#[test]
pub fn string() {
    let str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let mut vec:Vec<u8> = Vec::new();
    vec.write_string(str).unwrap();
    let (value,_size) = vec.read_string(0).unwrap();
    assert_eq!(str,value);
}