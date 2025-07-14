//! This module is for handling cph files, which follow the cipher standard.
//! Its pretty basic, and will be outlined here:
//!
//! 8 bytes - magic number: 0x63 0x70 0x68 0x33 0x72 0x75 0x73 0x74 > ascii: cph3rust
//! 2 bytes - version, currently 0x0001
//! 8 bytes - salt for pbkdf2
//! 4 bytes - rounds used in pbkdf2
//! 12 bytes - nonce of the encrypted blob
//! 8 bytes - length of the encrypted blob
//! --
//! variable length - encrypted blob
//! --
//! 4 bytes - xxh32 checksum

use std::{fs::File, io::{self, Read, Write}, path::{Path, PathBuf}};

use xxhash_rust::xxh32::xxh32;

const MAGIC_NUMBER: [u8; 8] = [0x63, 0x70, 0x68, 0x33, 0x72, 0x75, 0x73, 0x74];
const VERSION: u16 = 0x0001;

#[derive(Debug)]
pub struct CphFile {
    magic_number: [u8; 8],
    version: [u8; 2],
    pub salt: [u8; 8],
    pub rounds: u32,
    pub nonce: [u8; 12],
    length: u64,

    content: Vec<u8>,

    checksum: [u8; 4],
}

impl CphFile {
    pub fn new() -> Self {
        Self {
            magic_number: MAGIC_NUMBER,
            version: VERSION.to_be_bytes(),
            salt: [0u8; 8],
            rounds: 0,
            nonce: [0u8; 12],
            length: 0,
            content: Vec::new(),
            checksum: [0u8; 4]
        }
    }

    pub fn set_content(&mut self, data: Vec<u8>) {
        self.length = data.len() as u64;
        self.checksum = xxh32(&data, self.length as u32).to_be_bytes();
        self.content = data;
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;

        // header
        file.write_all(&self.magic_number)?;
        file.write_all(&self.version)?;
        file.write_all(&self.salt)?;
        file.write_all(&self.rounds.to_be_bytes())?;
        file.write_all(&self.nonce)?;
        file.write_all(&self.length.to_be_bytes())?;

        // body
        file.write_all(&self.content)?;

        // footer
        file.write_all(&self.checksum)?;

        Ok(())
    }

    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;

        let mut buf = [0u8; 8];
        file.read_exact(&mut buf)?;
        if buf != MAGIC_NUMBER {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a CPH file (invalid magic number)"))
        }

        let mut version = [0u8; 2];
        file.read_exact(&mut version)?;

        let mut salt = [0u8; 8];
        file.read_exact(&mut salt)?;

        let mut rounds_bytes = [0u8; 4];
        file.read_exact(&mut rounds_bytes)?;
        let rounds = u32::from_be_bytes(rounds_bytes);

        let mut nonce = [0u8; 12];
        file.read_exact(&mut nonce)?;

        let mut length_bytes = [0u8; 8];
        file.read_exact(&mut length_bytes)?;
        let length = u64::from_be_bytes(length_bytes);

        let mut content = vec![0; length as usize];
        file.read_exact(&mut content)?;

        let mut checksum_bytes = [0u8; 4];
        file.read_exact(&mut checksum_bytes)?;
        if checksum_bytes != xxh32(&content, length as u32).to_be_bytes() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Checksum does not match (integrity failure)"));
        }

        Ok(Self{
            magic_number: MAGIC_NUMBER,
            version,
            salt,
            rounds,
            nonce,
            length,
            content,
            checksum: checksum_bytes,
        })
    }
}

pub fn read_plain(path: PathBuf) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;

    let mut data: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        if byte.is_err() {
            return Err(byte.unwrap_err());
        } else {
            data.push(byte.unwrap());
        }
    }

    Ok(data)
}

pub fn write_plain(path: PathBuf, data: Vec<u8>) -> io::Result<()> {
    let mut file = File::create(path)?;

    let buf = data.as_slice();
    file.write_all(buf)?;

    Ok(())
}
