mod constants;

use num::{BigUint, Integer, One, ToPrimitive, Zero};
use num::bigint::RandBigInt;
use rand::ThreadRng;
use rand::Rng;
use crate::math::constants::{MILLER_RABIN_ROUNDS, SMALL_PRIMES};

pub fn modular_pow(base: usize, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let exponent = exponent;
    if modulus == &BigUint::from(1u8) {
        BigUint::ZERO
    } else {
        let mut result = BigUint::from(1u8);
        let mut base = base % modulus;
        loop {
            if exponent % &BigUint::from(2u8) == BigUint::from(1u8) {
                result = result * &base % modulus;
            }
            let exponent = &(exponent >> 1);
            base = &base * &base % modulus;
            if exponent <= &BigUint::ZERO {
                break
            }
        }
        result
    }
}

pub fn xgcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
    let mut a = a.clone();
    let mut b = b.clone();
    let mut x0 = BigUint::from(1u8);
    let mut x1 = BigUint::from(0u8);
    let mut y0 = BigUint::from(0u8);
    let mut y1 = BigUint::from(1u8);
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
        if b == BigUint::ZERO {
            return (a.clone(), x0.clone(), y0.clone())
        }
    }
}

pub fn modular_inverse(e: &BigUint, phi: &BigUint) -> BigUint {
    let (_, x, _) = xgcd(e, phi);
    x % phi
}

pub fn new_prime(bit_length: u64) -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        let candidate: BigUint = rng.gen_biguint(bit_length);
        if is_prime(&candidate) {
            return candidate;
        }
    }
}

// fn generate_random_of_length(rng: &mut ThreadRng, bit_size: usize) -> BigUint {
//     let (digits, rem) = bit_size.div_rem(&32usize);
//     let mut data = vec![u32::default(); digits + (rem > 0) as usize];
//     rng.fill_bytes(data[..].as_byte_slice_mut());
//     data.to_le();
//     if rem > 0 {
//         data[digits] >>= 32 - rem;
//     }
//     BigUint::new(data)
// }

pub fn is_prime(candidate: &BigUint) -> bool {
    let mut rng = rand::thread_rng();
    if candidate == &BigUint::ZERO {
        false
    }
    else if candidate == &BigUint::from(2u8) {
        true
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
        let mut x = a.modpow(&d, &candidate);

        // Reference Implementation
        //let mut y = Int::modpow(&basis, &d, candidate);

        if x == &BigUint::from(1u8) || x == (candidate - &BigUint::from(1u8)) {
            continue
            // return true
        }
        else {

            let mut break_early = false;
            // Issue #1 | Changed one_usize to zero_usize; step (s-1) was equal to iterations-1 and therefore needed an extra iteration
            for _ in 0..step {
                x = modular_pow(x, &BigUint::from(2u8), candidate);

                // Reference Implementation
                //y = Int::modpow(&y, &two, candidate);

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
    let one: BigUint = BigUint::one();
    let two: BigUint = BigUint::one() + BigUint::one();
    let mut s: BigUint = BigUint::zero();




    // (n-1) becomes even number
    let mut d: BigUint = n - &one;

    // The Main Loop That Checks Whether The Number is even and then divides by 2 and stores a counter

    while d.is_even() == true {
        d = d.div_floor(&two);
        s += &one;
    }

    return (d.clone(),s)
}