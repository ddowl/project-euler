extern crate primal;

// http://huonw.github.io/primal/primal/#indexing-primes
fn nth_prime(n: usize) -> usize {
    primal::Primes::all().nth(n - 1).unwrap()
}

#[allow(dead_code)]
fn main() {
    println!("{}", nth_prime(10_001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixth_prime() {
        assert_eq!(nth_prime(6), 13);
    }
}