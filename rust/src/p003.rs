extern crate primal;
use std::collections::HashSet;

fn largest_prime_primal(n: usize) -> usize {
    let sieve = primal::Sieve::new((n as f64).sqrt().ceil() as usize);
    let factors = sieve.factor(n).unwrap();
    *factors.iter()
        .map(|(p, _)| p)
        .max().unwrap()
}

#[allow(unused)]
fn largest_prime_slow(n: usize) -> usize {
    let n_sqrt = (n as f64).sqrt().ceil() as usize;
    let sieve = sieve(n);
    (1..n)
        .filter(|i| n % i == 0 && sieve.contains(i))
        .max().expect(&format!("No prime factors found for {}!", n))
}

// Trial division
#[allow(unused)]
fn is_prime_td(n: usize) -> bool {
    if n <= 2 { return true; }

    (2..(n as f64).sqrt().ceil() as usize)
        .all(|i| n % i != 0)
}

fn sieve(upper_bound: usize) -> HashSet<usize> {
    let mut sieve: Vec<bool> = vec![false; upper_bound];
    sieve[0] = true;
    sieve[1] = true;
    let mut primes = HashSet::new();

    let mut nth_prime = 2;
    primes.insert(nth_prime);

    // Finding the ith prime
    while nth_prime < upper_bound {

        // fill in sieve
        for idx in (nth_prime..upper_bound).step_by(nth_prime) {
            sieve[idx] = true;
        }

        // find first open space
        nth_prime = first_open_space(&sieve, nth_prime);
        primes.insert(nth_prime);
    }

    primes
}

fn first_open_space(sieve: &Vec<bool>, at_least: usize) -> usize {
    let mut idx = at_least;
    while idx < sieve.len() && sieve[idx] { idx += 1; }
    idx
}

#[allow(dead_code)]
fn main() {
    println!("{}", largest_prime_primal(600_851_475_143));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_is_prime_with_trial_division() {
        assert_eq!(is_prime_td(2), true);
    }

    #[test]
    fn thirteen_is_prime_with_trial_division() {
        assert_eq!(is_prime_td(13), true);
    }

    #[test]
    fn thirty_is_not_prime_with_trial_division() {
        assert_eq!(is_prime_td(30), false);
    }

    #[test]
    fn largest_prime_of_15() {
        assert_eq!(largest_prime_primal(15), 5);
    }

    #[test]
    fn largest_prime_of_13195() {
        assert_eq!(largest_prime_primal(13195), 29);
    }

    #[test]
    fn largest_prime_of_475_143() {
        assert_eq!(largest_prime_primal(475_143), 631);
    }

    #[test]
    fn largest_prime_of_1_475_143() {
        assert_eq!(largest_prime_primal(1_475_143), 50867);
    }

    #[test]
    fn largest_prime_of_51_475_143() {
        assert_eq!(largest_prime_primal(51_475_143), 12497);
    }

    #[test]
    fn largest_prime_of_851_475_143() {
        assert_eq!(largest_prime_primal(851_475_143), 851475143);
    }

    #[test]
    fn largest_prime_of_1_000_000_000() {
        assert_eq!(largest_prime_primal(1_000_000_000), 5);
    }
}