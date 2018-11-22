fn largest_prime(n: usize) -> usize {
    *primes(n).iter().max().expect(&format!("No primes found for {}!", n))
}

fn primes(n: usize) -> Vec<usize> {
    vec!()
}


#[allow(dead_code)]
fn main() {
    println!("{}", largest_prime(600_851_475_143));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_of_15() {
        assert_eq!(largest_prime(15), 5);
    }

    #[test]
    fn largest_prime_of_13195() {
        assert_eq!(largest_prime(13195), 29);
    }
}