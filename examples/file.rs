extern crate binary_reader;

use std::fs::File;

use binary_reader::BinaryReader;

// `file.bin` struct
// c-string: Hello, world!
// i64: -1
// u32: 123456789
// f64: 3.141592653589793
// f32: 1234.56

fn main() {
    let mut file = File::open("file.bin").unwrap();

    let mut bin = BinaryReader::from_file(&mut file);

    assert_eq!("Hello, world!", bin.read_cstr().unwrap());
    assert_eq!(-1, bin.read_i64().unwrap());
    assert_eq!(123456789, bin.read_u32().unwrap());
    assert_eq!(3.141592653589793, bin.read_f64().unwrap());
    assert_eq!(1234.56, bin.read_f32().unwrap());
}
