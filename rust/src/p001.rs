use std::collections::HashSet;

fn sum_of_multiples_below(factors: HashSet<u32>, limit: u32) -> u32 {
    (2..)
        .take_while(|n| n < &limit)
        .filter(|n| factors.iter().any(|f| n % f == 0))
        .sum()
}

fn given_factors() -> HashSet<u32> {
    [3, 5].iter().cloned().collect()
}

fn main() {
    println!("{}", sum_of_multiples_below(given_factors(), 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_multiples_below_10() {
        let limit = 10;
        let sum = sum_of_multiples_below(given_factors(), limit);
        assert_eq!(sum, 3 + 5 + 6 + 9);
    }

    #[test]
    fn sum_of_multiples_below_14() {
        let limit = 14;
        let sum = sum_of_multiples_below(given_factors(), limit);
        assert_eq!(sum, 3 + 5 + 6 + 9 + 10 + 12);
    }
}
