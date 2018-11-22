use std::collections::HashSet;

// Try trial division first
fn largest_prime(n: usize) -> usize {
    *factors(&n).iter()
        .filter(|f| is_prime(*f))
        .max().expect(&format!("No prime factors found for {}!", n))
}

fn factors(n: &usize) -> HashSet<usize> {
    HashSet::new()
}

fn is_prime(n: &usize) -> bool {
    false
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