struct MyStruct(isize);

macro_rules! tuple_of {
    ($($x:expr),*) => {
        ($($x, 0),*)
    };
    ($mapper: path; $($x: expr),+) => {
        ( $( $mapper($x) ),+ )
    };
}

#[test]
fn test_tuples() {
    let t = tuple_of![];
    let t = tuple_of![1, 2, 3];
    let t = tuple_of![Some; 1, 2, 3];
}

// macro_rules! count_foos {
//     (foo) => {
//         1
//     };
//     (foo, foo) => {
//         2
//     };
// }

// macro_rules! count_foos {
//     (foo) => { 1 };
//     (foo, $($f:ident),+) => {
//         1 + count_foos!($($f),+)
//     };
// }

// #[test]
// fn test() {
//     let r = count_foos!(foo, foo, foo, foo);
//     dbg!(r);
//     assert!(false);
// }

fn main() {
    println!("Hello, world!");
}
