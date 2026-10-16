//! Task 100 (advanced): make a message fit in one SMS.
//!
//! One SMS holds 160 characters of plain text, but only 70 if the message contains anything
//! outside basic English characters (an emoji, `é`, `ñ`), because the whole message switches to a
//! wider encoding. Count the wider encoding in UTF-16 units: an emoji counts as 2.
//! - plain text: everything is printable ASCII (`' '` to `'~'`) → limit 160 characters
//! - otherwise → limit 70 UTF-16 units
//!
//! If the message fits, return it unchanged. If not, cut it at the last space that leaves room
//! for `"..."` (or exactly at the limit, if there's no space), and add `"..."`. The result must
//! always fit.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

pub fn fit_sms(message: &str) -> String {
    todo!()
}
