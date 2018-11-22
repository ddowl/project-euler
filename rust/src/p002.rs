use std::collections::HashMap;

fn sum_of_even_fibs(limit: usize) -> usize {
    (0..)
        .map(|x| fib(x))
        .take_while(|f| f < &limit)
        .filter(|f| f % 2 == 0)
        .sum()
}

fn fib(x: usize) -> usize {
    fn fib(x: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if x == 0 || x == 1 {
            return x
        }

        if let Some(val) = cache.get(&x) {
            return *val
        }

        let f1 = fib(x - 1, cache);
        let f2 = fib(x - 2, cache);
        cache.insert(x - 1, f1);
        cache.insert(x - 2, f2);
        f1 + f2
    }
    fib(x, &mut HashMap::new())
}

#[allow(dead_code)]
fn main() {
    println!("{}", sum_of_even_fibs(4_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_10() {
        assert_eq!(sum_of_even_fibs(10), 10);
    }

    #[test]
    fn limit_100() {
        assert_eq!(sum_of_even_fibs(100), 44);
    }
}
