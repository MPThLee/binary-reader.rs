//! A Binary reader for step by step.
//!
//! # Example
//!
//! ```
//! extern crate binary_reader;
//!
//! use binary_reader::{Endian, BinaryReader};
//!
//! fn main() {
//!     let vector: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x00, 0x0B, 0x77];
//!     let mut binary = BinaryReader::from_vec(&vector);
//!     binary.set_endian(Endian::Big);
//!
//!     assert_eq!("Hello, World!", binary.read_cstr().unwrap());
//!     assert_eq!(2_935, binary.read_i16().unwrap());
//! }
//! ```

extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{prelude::*, Error, ErrorKind};

/// An Enums for set Endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    Little,
    //Native
}

pub struct BinaryReader {
    pub data: Vec<u8>,
    pub pos: usize,
    pub length: usize,
    pub endian: Endian,
}

impl BinaryReader {
    #[doc(hidden)]
    fn initialize() -> BinaryReader {
        BinaryReader {
            data: Vec::new(),
            pos: 0,
            length: 0,
            endian: Endian::Big,
        }
    }

    #[doc(hidden)]
    fn read_bytes(&mut self, bytes: usize) -> std::io::Result<&[u8]> {
        let data = self.data.get(self.pos..self.pos + bytes).ok_or_else(|| {
            Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to read {} bytes from offset {}", bytes, self.pos),
            )
        })?;
        self.pos += bytes;

        Ok(data)
    }

    /// Initialize BinaryReader from u8 slice.
    pub fn from_u8(get: &[u8]) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = get.to_vec();
        a.length = get.len();
        a
    }

    #[allow(clippy::ptr_arg)]
    /// Initialize BinaryReader from u8 Vector.
    pub fn from_vec(vec: &Vec<u8>) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = vec.to_vec();
        a.length = vec.len();
        a
    }

    /// Initialize BinaryReader from `std::fs::File`.
    pub fn from_file(file: &mut std::fs::File) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        let mut v: Vec<u8> = Vec::new();
        a.length = file.read_to_end(&mut v).unwrap();
        a.data = v;
        a
    }

    /// Set endian for read method.
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian
    }

    /// Jump position.
    pub fn jmp(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn adv(&mut self, size: usize) {
        self.pos += size
    }

    pub fn align(&mut self, size: usize) {
        self.pos = (self.pos + size - 1) / size * size
    }

    /// Read length size bytes.
    pub fn read(&mut self, size: usize) -> Option<&[u8]> {
        let data = self.data.get(self.pos..self.pos + size);
        self.pos += size;
        data
    }

    /// Read cstr.
    /// Read String(s) until `null`(aka `0x00`).
    pub fn read_cstr(&mut self) -> std::io::Result<String> {
        // "abc" "null" "def"
        let mut data = self
            .data
            .clone()
            .get(self.pos..self.length)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnexpectedEof,
                    format!(
                        "failed to read {} bytes from offset {}",
                        self.length - self.pos,
                        self.pos
                    ),
                )
            })?
            .to_vec();
        data.reverse();
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let a = data.pop().unwrap();
            if a == 0x00 {
                self.pos += vec.len() + 1;
                return String::from_utf8(vec).map_err(|err| {
                    Error::new(
                        ErrorKind::UnexpectedEof,
                        format!("failed to convert to string: {:?}", err),
                    )
                });
            } else {
                vec.push(a);
            }
        }
    }

    /// Read signed 8 bit integer
    pub fn read_i8(&mut self) -> std::io::Result<i8> {
        let mut data = self.read_bytes(1)?;
        data.read_i8()
    }

    /// Read signed 16 bit integer
    pub fn read_i16(&mut self) -> std::io::Result<i16> {
        let endianness = self.endian;
        let mut data = self.read_bytes(2)?;
        match endianness {
            Endian::Big => data.read_i16::<BigEndian>(),
            Endian::Little => data.read_i16::<LittleEndian>(),
        }
    }

    /// Read signed 32 bit integer
    pub fn read_i32(&mut self) -> std::io::Result<i32> {
        let endianness = self.endian;
        let mut data = self.read_bytes(4)?;
        match endianness {
            Endian::Big => data.read_i32::<BigEndian>(),
            Endian::Little => data.read_i32::<LittleEndian>(),
        }
    }

    /// Read signed 64 bit integer
    pub fn read_i64(&mut self) -> std::io::Result<i64> {
        let endianness = self.endian;
        let mut data = self.read_bytes(8)?;
        match endianness {
            Endian::Big => data.read_i64::<BigEndian>(),
            Endian::Little => data.read_i64::<LittleEndian>(),
        }
    }

    /// Read 32 bit float
    pub fn read_f32(&mut self) -> std::io::Result<f32> {
        let endianness = self.endian;
        let mut data = self.read_bytes(4)?;
        match endianness {
            Endian::Big => data.read_f32::<BigEndian>(),
            Endian::Little => data.read_f32::<LittleEndian>(),
        }
    }

    /// Read 64 bit float
    pub fn read_f64(&mut self) -> std::io::Result<f64> {
        let endianness = self.endian;
        let mut data = self.read_bytes(8)?;
        match endianness {
            Endian::Big => data.read_f64::<BigEndian>(),
            Endian::Little => data.read_f64::<LittleEndian>(),
        }
    }

    /// Read unsigned 8 bit integer
    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut data = self.read_bytes(1)?;
        data.read_u8()
    }

    /// Read unsigned 16 bit integer
    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        let endianness = self.endian;
        let mut data = self.read_bytes(2)?;
        match endianness {
            Endian::Big => data.read_u16::<BigEndian>(),
            Endian::Little => data.read_u16::<LittleEndian>(),
        }
    }

    /// Read unsigned 32 bit integer
    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        let endianness = self.endian;
        let mut data = self.read_bytes(4)?;
        match endianness {
            Endian::Big => data.read_u32::<BigEndian>(),
            Endian::Little => data.read_u32::<LittleEndian>(),
        }
    }

    /// Read unsigned 64 bit integer
    pub fn read_u64(&mut self) -> std::io::Result<u64> {
        let endianness = self.endian;
        let mut data = self.read_bytes(8)?;
        match endianness {
            Endian::Big => data.read_u64::<BigEndian>(),
            Endian::Little => data.read_u64::<LittleEndian>(),
        }
    }
}

#[cfg(test)]
mod tests;
