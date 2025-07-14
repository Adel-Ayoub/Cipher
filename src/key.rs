use pbkdf2::pbkdf2_hmac;
use sha3::Sha3_512;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::file::CphFile;

#[derive(Debug)]
pub struct Key {
    bytes: [u8; 32],
    pub rounds: u32,
    pub salt: [u8; 8],
}

impl Key {
    pub fn new(password: &[u8], rounds: u32) -> Self {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut salt = [0u8; 8];
        for i in 0..salt.len() {
            salt[i] = rng.gen();
        }

        let mut bytes = [0u8; 32];

        pbkdf2_hmac::<Sha3_512>(password, &salt, rounds, &mut bytes);

        Key {
            bytes,
            rounds,
            salt,
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn from_file(file: &CphFile, password: &[u8]) -> Self {
        let rounds = file.rounds;
        let salt = file.salt;

        let mut bytes = [0u8; 32];
        pbkdf2_hmac::<Sha3_512>(password, &salt, rounds, &mut bytes);

        Key {
            bytes,
            rounds,
            salt,
        }
    }
}
