mod more_complex;
mod variadic;

fn old_tax(household_size: usize) -> usize {
    return 10 * household_size;
}

fn new_tax(household_size: usize) -> usize {
    return 10 * household_size;
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_gt {
        ($a:expr, $b:expr) => {
            assert!($a > $b, "Expected {} to be greater than {}", $a, $b)
        };
    }

    #[test]
    fn new_tax_is_higher() {
        assert!(new_tax(5) > old_tax(5));
        assert!(new_tax(10) > old_tax(10));
        // ...
    }

    #[test]
    fn new_tax_is_higher_macros() {
        assert_gt!(new_tax(5), old_tax(5));
        assert_gt!(new_tax(10), old_tax(10));
        // ...
    }
}
