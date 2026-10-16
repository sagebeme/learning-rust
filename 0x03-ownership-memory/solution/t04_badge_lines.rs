//! Task 4: turn names into badge lines, taking ownership.
//!
//! This function *owns* the list it's given. Turn each name into `"BADGE: <NAME IN CAPITALS>"`,
//! reusing the strings instead of allocating new ones where you can.
//! `badge_lines(vec!["Amina".into()])` returns `["BADGE: AMINA"]`.

pub fn badge_lines(names: Vec<String>) -> Vec<String> {
    names
        .into_iter()
        .map(|mut name| {
            name.make_ascii_uppercase();
            name.insert_str(0, "BADGE: ");
            name
        })
        .collect()
}
