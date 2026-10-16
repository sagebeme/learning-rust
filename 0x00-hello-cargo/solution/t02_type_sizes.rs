//! Task 2: how big is each type?
//!
//! Return the name and size in bytes of each of these types, in this order:
//! `u8`, `i32`, `i64`, `f64`, `bool`, `char`, `&str`, `String`.
//!
//! Use `std::mem::size_of::<T>()` for every size. Don't type the numbers in by hand.

use std::mem::size_of;

pub fn type_sizes() -> Vec<(&'static str, usize)> {
    vec![
        ("u8", size_of::<u8>()),
        ("i32", size_of::<i32>()),
        ("i64", size_of::<i64>()),
        ("f64", size_of::<f64>()),
        ("bool", size_of::<bool>()),
        ("char", size_of::<char>()),
        ("&str", size_of::<&str>()),
        ("String", size_of::<String>()),
    ]
}
