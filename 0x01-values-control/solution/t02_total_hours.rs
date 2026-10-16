//! Task 2: total hours worked, without overflowing.
//!
//! Add up the hours. If the total doesn't fit in a `u16` (more than 65,535), return `None`
//! instead of a wrong number. Use `checked_add`.

pub fn total_hours(hours: &[u16]) -> Option<u16> {
    let mut total: u16 = 0;
    for &h in hours {
        total = total.checked_add(h)?;
    }
    Some(total)
}
