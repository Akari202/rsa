mod constants;

use log::trace;
use num::{BigInt, BigUint, Integer, Signed, ToPrimitive};
use num::bigint::{RandBigInt, ToBigInt};
use rand::prelude::ThreadRng;
use crate::math::constants::{MILLER_RABIN_ROUNDS, SMALL_PRIMES};

pub fn modular_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let mut exponent = exponent.clone();
    if modulus == &BigUint::from(1u8) {
        BigUint::ZERO
    }
    else {
        let mut result = BigUint::from(1u8);
        let mut base = base % modulus;
        let one = BigUint::from(1u8);
        let two = &BigUint::from(2u8);
        while exponent > BigUint::ZERO {
            if &exponent % two == one {
                result = result * &base % modulus;
            }
            exponent = exponent >> 1;
            base = &base * &base % modulus;
        }
        result
    }
}

pub fn xgcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut a = a.clone();
    let mut b = b.clone();
    let mut x0 = BigInt::from(1u8);
    let mut x1 = BigInt::ZERO;
    let mut y0 = BigInt::ZERO;
    let mut y1 = BigInt::from(1u8);
    loop {
        let q = &a / &b;
        let a_old = a.clone();
        a = b.clone();
        b = a_old % b;
        let x0_old = x0;
        x0 = x1.clone();
        x1 = x0_old - &q * x1;
        let y0_old = y0;
        y0 = y1.clone();
        y1 = y0_old - &q * y1;
        if b == BigInt::ZERO {
            return (a.clone(), x0.clone(), y0.clone())
        }
    }
}

pub fn modular_inverse(e: &BigUint, phi: &BigUint) -> BigUint {
    let phi = phi.to_bigint().unwrap();
    let (_, x, _) = xgcd(&e.to_bigint().unwrap(), &phi);
    let mut inverse = x % &phi;
    while inverse.is_negative() {
        inverse += &phi;
    }
    inverse.to_biguint().unwrap()
}

pub fn new_prime(bit_length: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let candidate: BigUint = rng.gen_biguint(bit_length);
        if is_prime(&candidate) {
            trace!("Successful Prime: {:?}", candidate);
            return candidate;
        }
        trace!("Failed Prime: {:?}", candidate);
    }
}

pub fn is_prime(candidate: &BigUint) -> bool {
    let mut rng = rand::thread_rng();
    if candidate == &BigUint::ZERO || candidate == &BigUint::from(2u8) {
        false
    }
    else if candidate.is_even() {
        false
    }
    else if !divide_small_primes(candidate) {
        false
    }
    else if !fermat(&mut rng, candidate) {
        false
    }
    else if !miller_rabin(&mut rng, candidate, MILLER_RABIN_ROUNDS) {
        false
    } else {
        true
    }
}

fn divide_small_primes(number: &BigUint) -> bool {
    for i in SMALL_PRIMES.iter() {
        if number % &BigUint::from(*i) == BigUint::ZERO {
            return false
        }
    }
    true
}

fn fermat(rng: &mut ThreadRng, candidate: &BigUint) -> bool {
    let random = rng.gen_biguint_below(candidate);
    let exponent = candidate - BigUint::from(1u8);
    let result = modular_pow(&random, &exponent, candidate);
    result == BigUint::from(1u8)
}

// needs to be fixed
fn miller_rabin(rng: &mut ThreadRng, candidate: &BigUint, limit: usize) -> bool {
    let (d, s) = rewrite(&candidate);
    let step = (s - &BigUint::from(1u8)).to_usize().unwrap();

    for _ in 0..limit {
        let a = rng.gen_biguint_range(&BigUint::from(2u8), &(candidate - &BigUint::from(1u8)));

        // Reference Implementation
        // Pretty sure `sample_range()` has an inclusive end
        //let basis = Int::sample_range(&two, &(candidate-&two));

        // (a^d mod n)
        let mut x = modular_pow(&a, &d, &candidate);

        // Reference Implementation
        //let mut y = Int::modpow(&basis, &d, candidate);

        if x == BigUint::from(1u8) || x == (candidate - &BigUint::from(1u8)) {
            continue
            // return true
        }
        else {

            let mut break_early = false;
            // Issue #1 | Changed one_usize to zero_usize; step (s-1) was equal to iterations-1 and therefore needed an extra iteration
            for _ in 0..step {
                x = modular_pow(&x, &BigUint::from(2u8), candidate);

                if x == BigUint::from(1u8) {
                    return false
                }
                else if x == (candidate - &BigUint::from(1u8)) {
                    break_early = true;
                    break;
                }

            }

            if !break_early {
                return false
            }
        }
    }
    return true
}

fn rewrite(n: &BigUint) -> (BigUint,BigUint) {
    let one: BigUint = BigUint::from(1u8);
    let mut s: BigUint = BigUint::ZERO;
    let mut d: BigUint = n - &one;

    while d.is_even() == true {
        d = d.div_floor(&BigUint::from(2u8));
        s += &one;
    }

    (d.clone(), s)
}