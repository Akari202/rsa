use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use log::{info, trace};
use num::BigUint;
use crate::rsa::{Key, KeySet};

#[test]
fn test_rsa() {
    let key_name = "test_keys";
    let input = "./src/rsa.rs";

    let mut rng = rand::thread_rng();
    let input_plaintext = fs::read_to_string(input).unwrap();

    // Key generation
    let keyset = KeySet::new(6, 128);
    keyset.save_keys(key_name).unwrap();

    // Encryption
    let public_key = Key::load_public_key(key_name).unwrap();
    let ciphertext: Vec<BigUint> = input_plaintext
        .chars()
        .map(|i| {
            public_key.encrypt(&mut rng, i as u8)
        })
        .collect();

    // Decryption
    let private_key = Key::load_private_key(key_name).unwrap();
    let plaintext: String = ciphertext
        .iter()
        .map(|i| {
            private_key.decrypt(i) as char
        })
        .collect::<String>();

    // Assertions
    assert_eq!(plaintext, input_plaintext)
}