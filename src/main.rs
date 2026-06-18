pub mod primality_test;
use crate::primality_test::check_prime_3_mod_4;
use ibig::{ibig, modular::ModuloRing, ubig, UBig};
use rand::prelude::*;
fn main() {
    check_prime_3_mod_4(ubig!(103));
}