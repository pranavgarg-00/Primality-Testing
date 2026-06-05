# Primality-Testing
Manuel Blum Primality Tester using Berlekamp-Rabin quadratic residues.

I read this algorithm (seems kinda elegant) in Goldreich's Foundations of Cryptography. Couldn't find any implementation online. 

Using facts:
1) A quadratic residue s modulo a prime has only 2 unique roots (+r and -r), where r^2 = s mod(p).
2) A quadratic residue s modulo a non-integer prime power composite has at least 4 unique roots.

If we have a SQRT algorithm that finds the root of a quadratic residue, we can use it to build a primality tester as follows.

Algorithm:
Given an integer n to test for prime. We run the following experiment many times:
  1) Choose a random $r \in [1, n-1]$.
  2) Calculate $s = r^2 (mod n)$.
  3) Check if SQRT($s$) = $\pm r$.

The implementation of SQRT is easy for primes that are 3 (mod 4). For the 1 (mod 4) case, we use the Berlekamp-Rabin algorithm. We stress that the above experiment would return correct $r$ for composites at most half the time, whereas for primes it would return correct $r$ always for the 3 (mod 4) case, and still more often for than composites for the 1 (mod 4) case. 
Another note is that, even though prime powers only have 2 unique roots, the SQRT algorithm that would work on primes would certainly fail on them prime powers with a significant probability.

I'll add a large prime generator once the primality tester starts working.
