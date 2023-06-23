macro_rules! assert_gt {
    ($a:expr, $b:expr) => {
        assert!($a > $b, "Expected {} to be greater than {}", $a, $b)
    };

    ($a:expr, $b:expr, $($c: expr),+) => {{
        assert!($a > $b, "Expected {} to be greater than {}", $a, $b);
        assert_gt!($b, $($c),+);
    }};
}

macro_rules! print_lots {
    ($x: expr) => {
        println!("{}", $x);
    };

    ($x: expr, $($y: expr),+) => {{
        print_lots($x);
        print_lots($($y),+);
    }}
}

#[test]
fn test_variadic_assert_gt() {
    assert_gt!(3, 2, 1, 0)
}
