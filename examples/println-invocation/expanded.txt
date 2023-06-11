#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let name = "Ewan";
    {
        ::std::io::_print(format_args!("Hello {0}\n", name));
    };
}
