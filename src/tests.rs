use super::*;

#[test]
fn is_rust_working_well() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn read_cstr() {
    let vector: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x00, 0x6F, 0x2F, 0x00];
    // "Hello, World" + null + "o/"
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!("Hello, World!", bin.cstr());
    assert_eq!(14, bin.pos);
    assert_eq!("o/", bin.cstr());
}

#[test]
fn read_interger_signed() {
    let vector: Vec<u8> = vec![
        0x13, 
        0x0B, 0x77, 
        0x15, 0x05, 0xB4, 0xA9,
        0x05, 0x8D, 0x15, 0xE1, 0x76, 0x28, 0x00, 0x00];
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(1_9, bin.i8());
    assert_eq!(2_935, bin.i16());
    assert_eq!(3_52695465, bin.i32());
    assert_eq!(4_00000000000000000, bin.i64());
}