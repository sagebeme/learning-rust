//! Task 0: the longest name, borrowed.
//!
//! Return the longest name in the list, counted in characters (not bytes), or `None` for an empty
//! list. If two names tie, return the first. Return a `&str` that borrows from the list: don't copy.

pub fn longest_name(names: &[String]) -> Option<&str> {
    let mut longest: Option<&str> = None;
    for name in names {
        let is_longer = match longest {
            None => true,
            Some(current) => name.chars().count() > current.chars().count(),
        };
        if is_longer {
            longest = Some(name);
        }
    }
    longest
}
