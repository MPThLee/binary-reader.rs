use super::*;

#[test]
fn is_rust_working_well() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn read_cstr() {
    let vector: Vec<u8> = vec![
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x00, 0x6F,
        0x2F, 0x00,
    ];
    // "Hello, World" + null + "o/"
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!("Hello, World!", bin.read_cstr());
    assert_eq!(14, bin.pos);
    assert_eq!("o/", bin.read_cstr());
}

#[test]
fn read_integer_signed() {
    #[rustfmt::skip]
    let vector: Vec<u8> = vec![
        0x13, 
        0x0B, 0x77, 
        0x15, 0x05, 0xB4, 0xA9,
        0x05, 0x8D, 0x15, 0xE1, 0x76, 0x28, 0x00, 0x00
    ];
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(1_9, bin.read_i8().unwrap());
    assert_eq!(2_935, bin.read_i16().unwrap());
    assert_eq!(3_52695465, bin.read_i32().unwrap());
    assert_eq!(4_00000000000000000, bin.read_i64().unwrap());
}

#[test]
fn read_float_32() {
    let initial_float = 255.254_f32;
    let vector: Vec<u8> = initial_float.to_be_bytes().to_vec();

    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(255.254_f32, bin.read_f32().unwrap());
}

#[test]
fn read_float_64() {
    let initial_float = 255.254_f64;
    let vector: Vec<u8> = initial_float.to_be_bytes().to_vec();

    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(255.254_f64, bin.read_f64().unwrap());
}

#[test]
fn integer_pos_jump_and_endian_parsing() {
    let vector: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];

    let mut bin = BinaryReader::from_vec(&vector);

    bin.set_endian(Endian::Big);
    assert_eq!(291, bin.read_i16().unwrap());
    assert_eq!(1164413355, bin.read_i32().unwrap());
    assert_eq!(-12817, bin.read_i16().unwrap());
    bin.jmp(0);
    assert_eq!(81985529216486895, bin.read_i64().unwrap());

    bin.jmp(0);
    bin.set_endian(Endian::Little);

    assert_eq!(8961, bin.read_i16().unwrap());
    assert_eq!(-1417058491, bin.read_i32().unwrap());
    assert_eq!(-4147, bin.read_i16().unwrap());
    bin.jmp(0);
    assert_eq!(-1167088121787636991, bin.read_i64().unwrap());
}
