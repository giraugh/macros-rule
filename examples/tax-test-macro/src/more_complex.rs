mod old {
    pub fn calculate_tax(income_cents: f64, household_size: usize) -> f64 {
        income_cents * 10.0 + (3.0 * household_size as f64)
    }
}

mod new {
    pub fn calculate_tax(income_cents: f64, household_size: usize) -> f64 {
        income_cents * 13.0 + (2.0 * household_size as f64)
    }
}

#[cfg(test)]
mod tests_no_macro {
    use super::*;

    #[test]
    fn tax_is_higher_now() {
        assert!(
            old::calculate_tax(1000.0, 10) < new::calculate_tax(1000.0, 10),
            "Expected tax to be higher with new routine"
        );
        assert!(
            old::calculate_tax(3000.0, 30) < new::calculate_tax(3000.0, 30),
            "Expected tax to be higher with new routine"
        );
        assert!(
            old::calculate_tax(7000.0, 40) < new::calculate_tax(7000.0, 40),
            "Expected tax to be higher with new routine"
        );
    }
}

#[cfg(test)]
mod tests_macro {
    use super::*;

    macro_rules! assert_new_tax_higher {
        ($income: expr, $household_size: expr) => {
            assert!(
                old::calculate_tax($income, $household_size)
                    < new::calculate_tax($income, $household_size),
                "Expected tax to be higher with new routine"
            )
        };
    }

    macro_rules! assert_returns_lt {
        ($fn_lt: path, $fn_gt: path, $($arg: expr),+) => {{
            let lt_res = $fn_lt($($arg),+);
            let gt_res = $fn_gt($($arg),+);
            assert!(lt_res < gt_res);
        }}
    }

    #[test]
    fn _tax_is_higher_now() {
        assert_new_tax_higher!(1000.0, 10);
        assert_new_tax_higher!(3000.0, 30);
        assert_new_tax_higher!(7000.0, 40);
    }

    #[test]
    fn tax_is_higher_now() {
        assert_returns_lt!(old::calculate_tax, new::calculate_tax, 1000.0, 10);
        assert_returns_lt!(old::calculate_tax, new::calculate_tax, 3000.0, 30);
        assert_returns_lt!(old::calculate_tax, new::calculate_tax, 7000.0, 40);
    }
}
