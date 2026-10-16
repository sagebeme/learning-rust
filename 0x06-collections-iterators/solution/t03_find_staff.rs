//! Task 3: binary search, written by hand, without the ALX bug.
//!
//! `sorted` is in ascending order. Return the index of `target`, or `None`. Write the search
//! yourself (no `slice::binary_search`), and make sure it never panics: an empty list, a target
//! smaller than the first number, and a target bigger than the last must all just return `None`.

pub fn find_staff(sorted: &[u32], target: u32) -> Option<usize> {
    // Search the half-open range low..high. When it's empty, the target isn't there. Because
    // `high` is one past the end, nothing ever has to go below zero.
    let mut low = 0;
    let mut high = sorted.len();
    while low < high {
        let middle = low + (high - low) / 2;
        match sorted[middle].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Less => low = middle + 1,
            std::cmp::Ordering::Greater => high = middle,
        }
    }
    None
}
