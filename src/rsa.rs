use std::error::Error;
use std::fs;
use std::fs::File;
use std::future::join;
use std::io::{BufRead, Read, Write};
use crate::math::{modular_inverse, modular_pow, new_prime};
use std::ops::Sub;
use std::path::PathBuf;
use log::{debug, info, trace};
use num::bigint::RandBigInt;
use num::{BigUint, ToPrimitive};
use rand::prelude::ThreadRng;

#[derive(Debug)]
pub struct Key {
    exponent: BigUint,
    modulus: BigUint,
    salt_bits: u32
}

#[derive(Debug)]
pub struct KeySet {
    private_key: Key,
    public_key: Key,
    phi: BigUint,
    primes: (BigUint, BigUint),
    bit_length: u64
}

impl Key {
    pub fn new(exponent: BigUint, modulus: BigUint, salt_bits: u32) -> Key {
        Key {
            exponent,
            modulus,
            salt_bits
        }
    }

    pub fn encrypt(&self, rng: &mut ThreadRng, input: u8) -> BigUint {
        trace!("Running Enrypt on {}", input);
        // let salt = rng.gen_biguint(self.salt_bits as u64);
        let input = BigUint::from(input);
        // let input = (input >> self.salt_bits) & salt;
        modular_pow(&input, &self.exponent, &self.modulus)
    }

    pub fn decrypt(&self, input: &BigUint) -> u8 {
        trace!("Running Decrypt");
        let decrypted = modular_pow(input, &self.exponent, &self.modulus);
        // (decrypted >> self.salt_bits).to_u8().unwrap()
        decrypted.to_u8().unwrap()
    }

    pub fn save_to_file(&self, mut file: File) -> Result<(), Box<dyn Error>> {
        let data = format!("{}\n{}\n{}", self.exponent, self.modulus, self.salt_bits);
        Ok(file.write_all(data.as_bytes())?)
    }

    pub fn load_public_key(name: &str) -> Result<Self, Box<dyn Error>> {
        let name = format!("{}.pub", name);
        Self::load_key(&name)
    }

    pub fn load_private_key(name: &str) -> Result<Self, Box<dyn Error>> {
       Self::load_key(name)
    }

    fn load_key(name: &str) -> Result<Self, Box<dyn Error>> {
        debug!("Loading Key: {}", name);
        let key_root = KeySet::get_key_root()?;
        let file_name = key_root.join(name);
        trace!("Key File: {:?}", file_name);
        let lines: Vec<String> = fs::read_to_string(file_name)?
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(lines.len(), 3);
        let exponent = lines[0].parse::<BigUint>()?;
        let modulus = lines[1].parse::<BigUint>()?;
        let salt_bits = lines[2].parse::<u32>()?;
        Ok(Self {
            exponent,
            modulus,
            salt_bits
        })
    }
}

impl KeySet {
    pub fn new(salt_bits: u32, bit_length: u64) -> Self {
        let prime_bit_length = bit_length / 2;
        let p = new_prime(prime_bit_length);
        let q = new_prime(prime_bit_length);
        let n = &p * &q;
        let phi = p.clone().sub(1u8) * q.clone().sub(1u8);
        let e = BigUint::from(2usize.pow(16) + 1);
        let d = modular_inverse(&e, &phi);
        KeySet {
            private_key: Key::new(d, n.clone(), salt_bits),
            public_key: Key::new(e, n.clone(), salt_bits),
            phi,
            primes: (p, q),
            bit_length: 0
        }
    }

    pub fn save_keys(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let key_root = Self::get_key_root()?;
        let public_file = File::create(key_root.join(format!("{}.pub", name)))?;
        self.public_key.save_to_file(public_file)?;
        let private_file = File::create(key_root.join(name))?;
        self.private_key.save_to_file(private_file)?;
        Ok(())
    }

    pub fn get_key_root() -> Result<PathBuf, Box<dyn Error>> {
        let home = dirs::home_dir().unwrap();
        let key_root = home.join(".amh_rsa");
        if !key_root.exists() {
            fs::create_dir(&key_root)?;
        }
        Ok(key_root)
    }
}
