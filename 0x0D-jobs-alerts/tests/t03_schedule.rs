use chrono::{DateTime, NaiveTime, Utc};
use jobs_alerts::t03_schedule::{nairobi, next_run_after};

fn utc(text: &str) -> DateTime<Utc> {
    text.parse().expect("an RFC 3339 time")
}

fn six_am() -> NaiveTime {
    NaiveTime::from_hms_opt(6, 0, 0).expect("a real time")
}

#[test]
fn nairobi_is_three_hours_ahead() {
    assert_eq!(nairobi().local_minus_utc(), 3 * 60 * 60);
}

#[test]
fn later_today_when_the_time_has_not_come_yet() {
    // 02:00 UTC is 05:00 in Nairobi: the job runs at 06:00 Nairobi, 03:00 UTC, the same day.
    assert_eq!(
        next_run_after(utc("2026-10-16T02:00:00Z"), six_am()),
        utc("2026-10-16T03:00:00Z")
    );
}

#[test]
fn tomorrow_when_it_has_passed() {
    assert_eq!(
        next_run_after(utc("2026-10-16T09:30:00Z"), six_am()),
        utc("2026-10-17T03:00:00Z")
    );
}

#[test]
fn exactly_at_the_time_means_tomorrow() {
    assert_eq!(
        next_run_after(utc("2026-10-16T03:00:00Z"), six_am()),
        utc("2026-10-17T03:00:00Z")
    );
}

#[test]
fn uses_the_nairobi_date_not_the_utc_date() {
    // 22:30 UTC on the 16th is already 01:30 on the 17th in Nairobi.
    assert_eq!(
        next_run_after(utc("2026-10-16T22:30:00Z"), six_am()),
        utc("2026-10-17T03:00:00Z")
    );
}

#[test]
fn across_the_end_of_the_year() {
    assert_eq!(
        next_run_after(utc("2026-12-31T05:00:00Z"), six_am()),
        utc("2027-01-01T03:00:00Z")
    );
}
