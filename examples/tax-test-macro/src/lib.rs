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

    // fn new_tax_is_higher_s() {
    //     let household_a = Household {
    //         people: 2,
    //         income: 3000,
    //     };
    //     assert!(new_tax(household_a) > old_tax(household_a));

    //     let household_b = Household {
    //         people: 5,
    //         income: 5000,
    //     };
    //     assert!(new_tax(household_b) > old_tax(household_b));

    //     let business_a = Business { income: 5000 };
    //     assert!(new_tax(business_a) > old_tax(business_a));
    // }

    // fn new_tax_is_higher_s_macros() {
    //     assert!(new_tax(building!(house, 2, 3000)) > old_tax(building!()));

    //     let village = Village::new(&[household_a, household_b, business_a]);

    //     let household_a = Building::Household {
    //         people: 2,
    //         income: 3000,
    //     };
    //     assert!(new_tax(household_a) > old_tax(household_a));

    //     let household_b = Building::Household {
    //         people: 5,
    //         income: 5000,
    //     };
    //     assert!(new_tax(household_b) > old_tax(household_b));

    //     let business_a = Building::Business { income: 5000 };
    //     assert!(new_tax(business_a) > old_tax(business_a));
    // }

    #[test]
    fn new_tax_is_higher_macros() {
        assert_gt!(new_tax(5), old_tax(5));
        assert_gt!(new_tax(10), old_tax(10));
        // ...
    }
}
