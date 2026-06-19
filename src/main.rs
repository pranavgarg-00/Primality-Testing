pub mod primality_test;
use crate::primality_test::{check_prime_3_mod_4, generate_random_prime_3_mod_4, generate_random_ubig_3_mod_4};
use ibig::{ibig, modular::ModuloRing, ubig, UBig};
use rand::prelude::*;
use std::ops::Add;


fn main() {
   println!("{}", generate_random_prime_3_mod_4(0b1000000000000_usize));
}