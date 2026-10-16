use hello_cargo::t02_type_sizes::type_sizes;

#[test]
fn lists_the_types_in_order() {
    let names: Vec<&str> = type_sizes().iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        ["u8", "i32", "i64", "f64", "bool", "char", "&str", "String"]
    );
}

#[test]
fn reports_the_fixed_size_types() {
    let sizes = type_sizes();
    assert_eq!(
        &sizes[..6],
        &[
            ("u8", 1),
            ("i32", 4),
            ("i64", 8),
            ("f64", 8),
            ("bool", 1),
            ("char", 4)
        ]
    );
}

// On a 64-bit machine a pointer is 8 bytes. A &str is a pointer + a length (16 bytes); a String is
// a pointer + a length + a capacity (24 bytes). The text itself isn't counted: it lives on the heap.
#[cfg(target_pointer_width = "64")]
#[test]
fn reports_strings_as_pointer_sized_handles() {
    let sizes = type_sizes();
    assert_eq!(&sizes[6..], &[("&str", 16), ("String", 24)]);
}
