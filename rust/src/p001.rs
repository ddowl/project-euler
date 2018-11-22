//#[macro_use] extern crate pretty_assertions;
fn not_a_test() {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn works_for_multiple_tests() {
        assert_eq!("Yes", "No")
    }
}
