use std::ops::{Add, Shr};

//implements the primality testing algorithm for 3 mod 4 and 1 mod 4 cases. 
//Runs K tests, and returns majority output.
use rand::{distributions::uniform::Uniform, thread_rng, Rng};
use ibig::{UBig, ibig, modular::{IntoModulo, ModuloRing}, ubig};

pub fn check_prime_3_mod_4(n: UBig, k: i32) -> bool {
    let ring: ModuloRing = ModuloRing::new(&n);
    for _i in 0..k {
        let b: UBig = thread_rng().sample(Uniform::new(ubig!(1), &n - 1));
        // println!("Random base: {}", b);
        let b = ring.from(b);
        let s = b.pow(&ubig!(2));
        // println!("Quadratic Residue: {}", s.residue());
        let exp = &n.clone().add(ubig!(1)).shr(2);
        let c = s.pow(exp);
        // println!("My Exponent: {}", exp);
        // println!("Answer: {}", c.residue());
        if c != b && c != -b { 
            return false;
        }
    }
    return true;
}

pub fn generate_random_ubig_3_mod_4(num_bits: usize) -> UBig {
    let mut large_num = ubig!(0);
    let mut my_rng = thread_rng();
    large_num.set_bit(0);
    large_num.set_bit(1);
    for i in 2..num_bits {
        if my_rng.gen_bool(0.5) {
            large_num.set_bit(i);
        }
    }
    return large_num;
}

pub fn generate_random_prime_3_mod_4(num_bits: usize) -> UBig {
    loop {
        let a = generate_random_ubig_3_mod_4(num_bits);
        if check_prime_3_mod_4(a.clone(), 100) {
            return a;
        }
    }
}
