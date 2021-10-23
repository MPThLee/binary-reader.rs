use super::*;

// from https://github.com/BurntSushi/byteorder/blob/663358f9d29bBinaryReader, BinaryR tests::I24_MAX1a tests::U24_MAX25f2cb851/src/lib.rs#L2391-L2394
pub const U24_MAX: u32 = 16_777_215;
pub const I24_MAX: i32 = 8_388_607;
pub const U48_MAX: u64 = 281_474_976_710_655;
pub const I48_MAX: i64 = 140_737_488_355_327;


#[test]
fn is_rust_working_well() {
    assert_eq!(2 + 2, 4);
}


macro_rules! read_max_number_test_macro {
    ($name:ident, $read:ident, $write:ident, $max:expr) => {
        mod $name {
            use byteorder::{BigEndian, LittleEndian, NativeEndian, ByteOrder};
            use crate::{Endian, BinaryReader};
            #[test]
            fn big() {
                let mut buf = [0; 16];
                BigEndian::$write(&mut buf, $max);
                
                let mut r = BinaryReader::from_u8(&mut buf);
                r.set_endian(Endian::Big);
                assert_eq!(r.$read().unwrap(), $max);
            }

            #[test]
            fn little() {
                let mut buf = [0; 16];
                LittleEndian::$write(&mut buf, $max);
                
                let mut r = BinaryReader::from_u8(&mut buf);
                r.set_endian(Endian::Little);
                assert_eq!(r.$read().unwrap(), $max);
            }

            #[test]
            fn native() {
                let mut buf = [0; 16];
                NativeEndian::$write(&mut buf, $max);
                
                let mut r = BinaryReader::from_u8(&mut buf);
                r.set_endian(Endian::Native);
                assert_eq!(r.$read().unwrap(), $max);
            }
        }
    };
    () => {
        
    };
}

// unsigned integer
read_max_number_test_macro!(u16, read_u16, write_u16, u16::MAX as u16);
read_max_number_test_macro!(u32, read_u32, write_u32, u32::MAX as u32);
read_max_number_test_macro!(u64, read_u64, write_u64, u64::MAX as u64);
read_max_number_test_macro!(u128, read_u128, write_u128, u128::MAX as u128);

// singed integer
read_max_number_test_macro!(i16, read_i16, write_i16, i16::MAX as i16);
read_max_number_test_macro!(i32, read_i32, write_i32, i32::MAX as i32);
read_max_number_test_macro!(i64, read_i64, write_i64, i64::MAX as i64);
read_max_number_test_macro!(i128, read_i128, write_i128, i128::MAX as i128);

// 24/48 bit unsinged/singed integers.
read_max_number_test_macro!(u24, read_u24, write_u24, crate::tests::U24_MAX as u32);
read_max_number_test_macro!(u48, read_u48, write_u48, crate::tests::U48_MAX as u64);
read_max_number_test_macro!(i24, read_i24, write_i24, crate::tests::I24_MAX as i32);
read_max_number_test_macro!(i48, read_i48, write_i48, crate::tests::I48_MAX as i64);

// Float
read_max_number_test_macro!(f32, read_f32, write_f32, f32::MAX as f32);
read_max_number_test_macro!(f64, read_f64, write_f64, f64::MAX as f64);


#[test]
fn read_cstr() {
    #[rustfmt::skip]
    let vector: Vec<u8> = vec![
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x00,
        0x6F, 0x2F, 0x00,
    ];
    // "Hello, World" + null + "o/"
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!("Hello, World!", bin.read_cstr().unwrap());
    assert_eq!(14, bin.pos);
    assert_eq!("o/", bin.read_cstr().unwrap());
}

#[test]
fn read_8bit() {
    let mut bin = BinaryReader::from_u8(&[0xED, 0x13]);
    assert_eq!(-19, bin.read_i8().unwrap());
    assert_eq!(19, bin.read_u8().unwrap());
}

#[test]
fn read_integer_signed() {
    #[rustfmt::skip]
    let vector: Vec<u8> = vec![
        0xED, 
        0xF4, 0x89, 
        0xEA, 0xFA, 0x4B, 0x57,
        0xFA, 0x72, 0xEA, 0x1E, 0x89, 0xD8, 0x00, 0x00
    ];
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(-1_9, bin.read_i8().unwrap());
    assert_eq!(-2_935, bin.read_i16().unwrap());
    assert_eq!(-3_52695465, bin.read_i32().unwrap());
    assert_eq!(-4_00000000000000000, bin.read_i64().unwrap());
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
