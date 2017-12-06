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
//!     assert_eq!("Hello, World!", binary.cstr());
//!     assert_eq!(2_935, binary.i16());
//! }
//! ```

extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::prelude::*;

/// An Enums for set Endian.
pub enum Endian {
    Big,
    Little,
    //Native
}


pub struct BinaryReader {
    pub data: Vec<u8>,
    pub pos: usize,
    pub length: usize,
    pub endian: Endian
}

impl BinaryReader {
    #[doc(hidden)]
    fn initialize() -> BinaryReader {
        BinaryReader {
            data: Vec::new(),
            pos: 0,
            length: 0,
            endian: Endian::Big
        }
    }

    /// Initialize BinaryReader from u8 slice.
    pub fn from_u8(get: &[u8]) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = get.to_vec();
        a.length = get.len();
        a
    }

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

    /// jump position.
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
    pub fn cstr(&mut self) -> String { // "abc" "null" "def"
        let mut data = self.data.clone().get(self.pos..self.length).unwrap().to_vec();
        data.reverse();
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let a = data.pop().unwrap();
            if a == 0x00 { self.pos += vec.len() + 1; return String::from_utf8(vec).unwrap() }
            else { vec.push(a); }
        }
    }

    /// read signed 8 bit interger
    pub fn i8(&mut self) -> i8 {
        let mut data = self.data.get(self.pos..self.pos + 1).unwrap();
        self.pos += 1;
        data.read_i8().unwrap()
    }

    /// read signed 16 bit interger
    pub fn i16(&mut self) -> i16 {
        let mut data = self.data.get(self.pos..self.pos + 2).unwrap();
        self.pos += 2;
        match self.endian {
            Endian::Big =>  data.read_i16::<BigEndian>().unwrap(),
            Endian::Little => data.read_i16::<LittleEndian>().unwrap()
        }
    }

    /// read signed 32 bit interger
    pub fn i32(&mut self) -> i32 {
        let mut data = self.data.get(self.pos..self.pos + 4).unwrap();
        self.pos += 4;
        match self.endian {
            Endian::Big =>  data.read_i32::<BigEndian>().unwrap(),
            Endian::Little => data.read_i32::<LittleEndian>().unwrap()
        }
    }

    /// read signed 64 bit interger
    pub fn i64(&mut self) -> i64 {
        let mut data = self.data.get(self.pos..self.pos + 8).unwrap();
        self.pos += 8;
        match self.endian {
            Endian::Big =>  data.read_i64::<BigEndian>().unwrap(),
            Endian::Little => data.read_i64::<LittleEndian>().unwrap()
        }
    }

    /// read unsigned 8 bit interger
    pub fn u8(&mut self) -> u8 {
        let mut data = self.data.get(self.pos..self.pos + 1).unwrap();
        self.pos += 1;
        data.read_u8().unwrap()
    }

    /// read unsigned 16 bit interger
    pub fn u16(&mut self) -> u16 {
        let mut data = self.data.get(self.pos..self.pos + 2).unwrap();
        self.pos += 2;
        match self.endian {
            Endian::Big =>  data.read_u16::<BigEndian>().unwrap(),
            Endian::Little => data.read_u16::<LittleEndian>().unwrap()
        }
    }

    /// read unsigned 32 bit interger
    pub fn u32(&mut self) -> u32 {
        let mut data = self.data.get(self.pos..self.pos + 4).unwrap();
        self.pos += 4;
        match self.endian {
            Endian::Big =>  data.read_u32::<BigEndian>().unwrap(),
            Endian::Little => data.read_u32::<LittleEndian>().unwrap()
        }
    }

    /// read unsigned 64 bit interger
    pub fn u64(&mut self) -> u64 {
        let mut data = self.data.get(self.pos..self.pos + 8).unwrap();
        self.pos += 8;
        match self.endian {
            Endian::Big =>  data.read_u64::<BigEndian>().unwrap(),
            Endian::Little => data.read_u64::<LittleEndian>().unwrap()
        }
    }
}


#[cfg(test)]
mod tests;