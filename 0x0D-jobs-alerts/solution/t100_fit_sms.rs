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

fn units(text: &str) -> usize {
    text.encode_utf16().count()
}

pub fn fit_sms(message: &str) -> String {
    let plain = message.chars().all(|c| (' '..='~').contains(&c));
    let limit = if plain { 160 } else { 70 };
    if units(message) <= limit {
        return message.to_string();
    }

    // The longest prefix that still leaves room for "...", ending on a character boundary.
    let room = limit - 3;
    let mut end = 0;
    let mut used = 0;
    for (index, c) in message.char_indices() {
        if used + c.len_utf16() > room {
            break;
        }
        used += c.len_utf16();
        end = index + c.len_utf8();
    }

    let cut = &message[..end];
    let cut = match cut.rfind(' ') {
        Some(space) if space > 0 => cut[..space].trim_end(),
        _ => cut,
    };
    format!("{cut}...")
}
