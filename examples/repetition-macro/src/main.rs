#[derive(Debug)]
struct MyStruct(isize);

macro_rules! tuple_of {
    ($($x:expr),*) => {
        ($($x, 0),*)
    };
    ($mapper:path; $($x: expr),+) => {
        ( $( $mapper($x) ),+ )
    };
}

#[allow(unused)]
#[test]
fn test_tuples() {
    let t = tuple_of![];
    let t = tuple_of![1, 2, 3];
    let t = tuple_of![Some; 1, 2, 3];
}

fn main() {
    let t = tuple_of![MyStruct; 1, 2, 3];
    dbg!(t);
}
