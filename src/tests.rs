use std::fs;
use num::BigUint;
use crate::rsa::{Key, KeySet};

const INPUT: &str = "./src/rsa.rs";
const KEY_NAME: &str = "test_keys";
const SALT_BITS: u32 = 6;
const BIT_LENGTH: u64 = 128;

#[test]
fn test_rsa() {
    let mut rng = rand::thread_rng();
    let input_plaintext = fs::read_to_string(&INPUT).unwrap();

    // Key generation
    let keyset = KeySet::new(SALT_BITS, BIT_LENGTH);
    keyset.save_keys(&KEY_NAME).unwrap();

    // Encryption
    let public_key = Key::load_public_key(&KEY_NAME).unwrap();
    let ciphertext: Vec<BigUint> = input_plaintext
        .chars()
        .map(|i| {
            public_key.encrypt(&mut rng, i as u8)
        })
        .collect();

    // Decryption
    let private_key = Key::load_private_key(&KEY_NAME).unwrap();
    let plaintext: String = ciphertext
        .iter()
        .map(|i| {
            private_key.decrypt(i).unwrap() as char
        })
        .collect::<String>();

    // Assertions
    assert_eq!(plaintext, input_plaintext)
}

#[test]
fn test_key_loading() {
    let keyset = KeySet::new(SALT_BITS, BIT_LENGTH);
    keyset.save_keys(&KEY_NAME).unwrap();
    let public_key = Key::load_public_key(&KEY_NAME).unwrap();
    let private_key = Key::load_private_key(&KEY_NAME).unwrap();

    assert_eq!(public_key, keyset.get_public_key());
    assert_eq!(private_key, keyset.get_private_key());
}
