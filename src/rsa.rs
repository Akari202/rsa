use crate::math::{modular_inverse, new_prime};
use num::BigUint;
use std::ops::Sub;

const BIT_LENGTH: u64 = 256;
const PRIME_BIT_LENGTH: u64 = BIT_LENGTH / 2;

#[derive(Debug)]
pub struct Key {
    exponent: BigUint,
    modulus: BigUint
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
    pub fn new(exponent: BigUint, modulus: BigUint) -> Key {
        Key {
            exponent,
            modulus
        }
    }
}

impl KeySet {
    pub fn new() -> Self {
        let p = new_prime(PRIME_BIT_LENGTH);
        let q = new_prime(PRIME_BIT_LENGTH);
        let n = &p * &q;
        let phi = p.clone().sub(1u8) * q.clone().sub(1u8);
        let e = BigUint::from(2usize.pow(16) + 1);
        let d = modular_inverse(&e, &phi);
        KeySet {
            private_key: Key::new(d, n.clone()),
            public_key: Key::new(e, n.clone()),
            phi,
            primes: (p, q),
            bit_length: 0,
        }
    }
}
