use anyhow::Result;
use crate::prelude::*;
#[test]
pub fn hello() -> Result<()>{
    println!("helloworld");
    Ok(())
}
#[test]
pub fn io_varint() {
    let number:u64 = 12345679;
    let mut vec:Vec<u8> = Vec::new();
    vec.write_var_int(number).unwrap();
    let (value,_size) = vec.read_varint(0).unwrap();
    assert_eq!(number,value);
}