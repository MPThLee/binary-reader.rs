//! A Binary reader for step by step.
//! It's a minimal [`byteorder`] wrapper for read bytes.
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

use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use std::io::{prelude::*, Error, ErrorKind};

/// An Enums for set Endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// See [`byteorder::BigEndian`].
    Big,
    /// See [`byteorder::LittleEndian`].
    Little,
    /// See [`byteorder::NativeEndian`].
    /// This endian varies with the platform.
    Native,
    /// See [`byteorder::NetworkEndian`].
    /// This endian is alias of [`BigEndian`](byteorder::BigEndian).
    Network,
}

#[doc(hidden)]
/// This macro does a generate read functions.
/// As there's too much duplicate code, I put it in a macro.
macro_rules! read_number {
    ($name:ident, $ty: ty, $bytes: expr, $doc: expr) => {
        #[doc = $doc]
        pub fn $name(&mut self) -> std::io::Result<$ty> {
            let endianness = self.endian;
            let mut data = self.read_bytes($bytes)?;
            match endianness {
                Endian::Big | Endian::Network => data.$name::<BigEndian>(),
                Endian::Little => data.$name::<LittleEndian>(),
                Endian::Native => data.$name::<NativeEndian>(),
            }
        }
    };

    ($name:ident, $ty: ty, $doc: expr) => {
        #[doc = $doc]
        pub fn $name(&mut self) -> std::io::Result<$ty> {
            let mut data = self.read_bytes(1)?;
            data.$name()
        }
    };
}
/// Binary reader.
#[derive(Debug, Clone)]
pub struct BinaryReader {
    /// The buffer data.
    pub data: Vec<u8>,
    /// The current position.
    pub pos: usize,
    /// The length of the buffer.
    pub length: usize,
    /// The endian of the buffer.
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

    /// Initialize BinaryReader from [`u8`] slice.
    pub fn from_u8(get: &[u8]) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = get.to_vec();
        a.length = get.len();
        a
    }

    #[allow(clippy::ptr_arg)] // leave this for bypass clippy warning.
    /// Initialize BinaryReader from [`u8`] [`Vector`](std::vec::Vec).
    pub fn from_vec(vec: &Vec<u8>) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = vec.to_vec();
        a.length = vec.len();
        a
    }

    /// Initialize BinaryReader from [`std::fs::File`].
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

    /// Read provided length size bytes.
    pub fn read(&mut self, size: usize) -> Option<&[u8]> {
        let data = self.data.get(self.pos..self.pos + size);
        self.pos += size;
        data
    }

    /// Read provided length size bytes.
    /// Similar to [`BinaryReader::read`] but this returns [`std::io::Result`] instead of [`Option`].
    pub fn read_bytes(&mut self, bytes: usize) -> std::io::Result<&[u8]> {
        let data = self.data.get(self.pos..self.pos + bytes).ok_or_else(|| {
            Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to read {} bytes from offset {}", bytes, self.pos),
            )
        })?;
        self.pos += bytes;

        Ok(data)
    }

    #[doc(hidden)]
    /// Read to end of data from current offset and Return reversed data.
    /// This is used for cstr read functions.
    fn read_cstr_post(&self) -> std::io::Result<Vec<u8>> {
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
        Ok(data)
    }

    /// Read cstr strings until `null`(aka `0x00`) using [`std::string::String::from_utf8`].
    pub fn read_cstr(&mut self) -> std::io::Result<String> {
        let mut data = self.read_cstr_post()?;
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let a = data.pop().unwrap();
            if a == 0x00 {
                self.pos += vec.len() + 1;
                return String::from_utf8(vec).map_err(|err| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("failed to convert to string: {:?}", err),
                    )
                });
            } else {
                vec.push(a);
            }
        }
    }

    /// Read cstr strings until `null`(aka `0x00`) using [`std::string::String::from_utf8_lossy`].
    pub fn read_cstr_lossy(&mut self) -> std::io::Result<String> {
        let mut data = self.read_cstr_post()?;
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let a = data.pop().unwrap();
            if a == 0x00 {
                self.pos += vec.len() + 1;
                return Ok(String::from_utf8_lossy(&vec).to_string());
            } else {
                vec.push(a);
            }
        }
    }

    /// Read boolean.
    /// Note that anything other than `0x00` is considered `true`.
    pub fn read_bool(&mut self) -> std::io::Result<bool> {
        let data = self.read_bytes(1)?;
        Ok(data[0] != 0)
    }

    // Signed integers
    read_number!(read_i8, i8, "Read signed 8 bit integer.");
    read_number!(read_i16, i16, 2, "Read signed 16 bit integer.");
    read_number!(
        read_i24,
        i32,
        3,
        "Read signed 24 bit integer. Stored in i32."
    );
    read_number!(read_i32, i32, 4, "Read signed 32 bit integer.");
    read_number!(
        read_i48,
        i64,
        6,
        "Read signed 48 bit integer. Stored in i64."
    );
    read_number!(read_i64, i64, 8, "Read signed 64 bit integer.");
    read_number!(read_i128, i128, 16, "Read signed 128 bit integer.");

    // Unsigned integers
    read_number!(read_u8, u8, "Read unsigned 8 bit integer.");
    read_number!(read_u16, u16, 2, "Read unsigned 16 bit integer.");
    read_number!(
        read_u24,
        u32,
        3,
        "Read unsigned 24 bit integer. Stored in u32."
    );
    read_number!(read_u32, u32, 4, "Read unsigned 32 bit integer.");
    read_number!(
        read_u48,
        u64,
        6,
        "Read unsigned 48 bit integer. Stored in u64."
    );
    read_number!(read_u64, u64, 8, "Read unsigned 64 bit integer.");
    read_number!(read_u128, u128, 16, "Read unsigned 128 bit integer.");

    // Floating point
    read_number!(read_f32, f32, 4, "Read 32 bit floating point number.");
    read_number!(read_f64, f64, 8, "Read 64 bit floating point number.");
}

#[cfg(test)]
mod tests;
