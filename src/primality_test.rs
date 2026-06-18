use std::ops::{Add, Shr};

//implements the primality testing algorithm for 3 mod 4 and 1 mod 4 cases. 
//Runs K tests, and returns majority output.
use rand::{distributions::uniform::Uniform, thread_rng, Rng};
use ibig::{UBig, ibig, modular::{IntoModulo, ModuloRing}, ubig};

pub fn check_prime_3_mod_4(n: UBig) {
    let ring = ModuloRing::new(&n);
    let b: UBig = thread_rng().sample(Uniform::new(ubig!(1), &n - 1));
    println!("Random base: {}", b);
    let b = ring.from(b);
    let s = b.pow(&ubig!(2));
    println!("Quadratic Residue: {}", s.residue());
    let exp = &n.add(ubig!(1)).shr(2);
    let c = s.pow(exp);
    println!("My Exponent: {}", exp);
    println!("Answer: {}", c.residue());
}