use super::*;

// from https://github.com/BurntSushi/byteorder/blob/663358f9d29bddadc1a8e84290ec96925f2cb851/src/lib.rs#L2391-L2394
pub const U24_MAX: u32 = 16_777_215;
pub const I24_MAX: i32 = 8_388_607;
pub const U48_MAX: u64 = 281_474_976_710_655;
pub const I48_MAX: i64 = 140_737_488_355_327;

pub const U24_MIN: u32 = 0;
pub const I24_MIN: i32 = -8_388_608;
pub const U48_MIN: u64 = 0;
pub const I48_MIN: i64 = -140_737_488_355_328;

#[test]
fn is_rust_working_well() {
    assert_eq!(2 + 2, 4);
}
mod integer_read {
    macro_rules! read_min_max_test_macro {
        ($name:ident, $bytes:expr, $read:ident, $write:ident, $max:expr, $min:expr) => {
            mod $name {
                use crate::{BinaryReader, Endian};
                use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian};
                #[test]
                fn big_endian() {
                    let mut buf = [0; 32];
                    BigEndian::$write(&mut buf, $max);
                    BigEndian::$write(&mut buf[$bytes..], $min);

                    let mut r = BinaryReader::from_u8(&mut buf);
                    r.set_endian(Endian::Big);
                    assert_eq!(r.$read().unwrap(), $max);
                    assert_eq!(r.$read().unwrap(), $min);
                }

                #[test]
                fn little_endian() {
                    let mut buf = [0; 32];
                    LittleEndian::$write(&mut buf, $max);
                    LittleEndian::$write(&mut buf[$bytes..], $min);

                    let mut r = BinaryReader::from_u8(&mut buf);
                    r.set_endian(Endian::Little);
                    assert_eq!(r.$read().unwrap(), $max);
                    assert_eq!(r.$read().unwrap(), $min);
                }

                #[test]
                fn native_endian() {
                    let mut buf = [0; 32];
                    NativeEndian::$write(&mut buf, $max);
                    NativeEndian::$write(&mut buf[$bytes..], $min);

                    let mut r = BinaryReader::from_u8(&mut buf);
                    r.set_endian(Endian::Native);
                    assert_eq!(r.$read().unwrap(), $max);
                    assert_eq!(r.$read().unwrap(), $min);
                }
            }
        };
        () => {};
    }

    // unsigned integer
    read_min_max_test_macro!(
        u16,
        2,
        read_u16,
        write_u16,
        ::std::u16::MAX,
        ::std::u16::MIN
    );
    read_min_max_test_macro!(
        u32,
        4,
        read_u32,
        write_u32,
        ::std::u32::MAX,
        ::std::u32::MIN
    );
    read_min_max_test_macro!(
        u64,
        8,
        read_u64,
        write_u64,
        ::std::u64::MAX,
        ::std::u64::MIN
    );
    read_min_max_test_macro!(
        u128,
        16,
        read_u128,
        write_u128,
        ::std::u128::MAX,
        ::std::u128::MIN
    );

    // singed integer
    read_min_max_test_macro!(
        i16,
        2,
        read_i16,
        write_i16,
        ::std::i16::MAX,
        ::std::i16::MIN
    );
    read_min_max_test_macro!(
        i32,
        4,
        read_i32,
        write_i32,
        ::std::i32::MAX,
        ::std::i32::MIN
    );
    read_min_max_test_macro!(
        i64,
        8,
        read_i64,
        write_i64,
        ::std::i64::MAX,
        ::std::i64::MIN
    );
    read_min_max_test_macro!(
        i128,
        16,
        read_i128,
        write_i128,
        ::std::i128::MAX,
        ::std::i128::MIN
    );

    // 24/48 bit unsinged/singed integers.
    read_min_max_test_macro!(
        u24,
        3,
        read_u24,
        write_u24,
        crate::tests::U24_MAX,
        crate::tests::U24_MIN
    );
    read_min_max_test_macro!(
        u48,
        6,
        read_u48,
        write_u48,
        crate::tests::U48_MAX,
        crate::tests::U48_MIN
    );
    read_min_max_test_macro!(
        i24,
        3,
        read_i24,
        write_i24,
        crate::tests::I24_MAX,
        crate::tests::I24_MIN
    );
    read_min_max_test_macro!(
        i48,
        6,
        read_i48,
        write_i48,
        crate::tests::I48_MAX,
        crate::tests::I48_MIN
    );

    // Float
    read_min_max_test_macro!(
        f32,
        4,
        read_f32,
        write_f32,
        ::std::f32::MAX,
        ::std::f32::MIN
    );
    read_min_max_test_macro!(
        f64,
        8,
        read_f64,
        write_f64,
        ::std::f64::MAX,
        ::std::f64::MIN
    );
}

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

#[test]
fn adv_and_bool() {
    let vector: Vec<u8> = vec![0x00, 0x00, 0x01, 0x00];
    let mut bin = BinaryReader::from_vec(&vector);
    assert_eq!(false, bin.read_bool().unwrap());
    bin.adv(1);
    assert_eq!(2, bin.pos);
    assert_eq!(true, bin.read_bool().unwrap());
    assert_eq!(false, bin.read_bool().unwrap());
}
