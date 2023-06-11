macro_rules! assert_lt {
    ($x: expr, $y: expr) => {
        assert!($x < $y, "assertion failed: expected {} < {}", $x, $y)
    };
}

#[test]
fn my_test() {
    assert!(1 < 10, "Expected 1 to be less than 10");
    assert!(10 < 100, "Expected 10 to be less than 100");
    assert!(5 < 500, "Expected 5 to be less than 500");
}

#[test]
fn my_test_two() {
    assert_lt!(1, 10);
    assert_lt!(10, 100);
    assert_lt!(5, 500);
}

fn main() {}
