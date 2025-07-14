use chacha20poly1305::{
    aead::{Aead, KeyInit, self},
    ChaCha20Poly1305,
};
use rand_chacha::ChaCha20Rng;
use rand::prelude::*;

use crate::key::Key;

#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub data: Vec<u8>,
    pub nonce: [u8; 12],
}

pub fn encrypt(key: &Key, data: &[u8]) -> EncryptedData {
    // can't panic, since Key.bytes are 32 bytes
    // initiating crypt-algorithm with the key
    let cipher = ChaCha20Poly1305::new_from_slice(key.get_bytes()).unwrap();

    // creating a new csprng and filling the nonce
    let mut rng = ChaCha20Rng::from_entropy();
    let nonce: [u8; 12] = rng.gen();

    // encrypting the plaintext and returning
    let crypted = cipher.encrypt(&nonce.into(), data).unwrap();
    EncryptedData { data: crypted, nonce }
}

pub fn decrypt(key: &Key, data: EncryptedData) -> aead::Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.get_bytes().into());
    cipher.decrypt(&data.nonce.into(), &*data.data)
}
