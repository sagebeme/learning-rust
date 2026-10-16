//! Task 100 (advanced): split a roster into teams, without copying any names.
//!
//! `chunk_roster(&["A", "B", "C"], 2)` returns `[["A", "B"], ["C"]]`. A team size of 0 returns an
//! empty list instead of crashing.

pub fn chunk_roster<'a>(staff: &[&'a str], team_size: usize) -> Vec<Vec<&'a str>> {
    if team_size == 0 {
        return Vec::new();
    }
    staff.chunks(team_size).map(<[&str]>::to_vec).collect()
}
