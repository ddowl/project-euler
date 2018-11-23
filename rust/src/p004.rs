fn func(n: usize) -> usize {
    0
}

#[allow(dead_code)]
fn main() {
    println!("{}", func(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(func(2), 0);
    }

}