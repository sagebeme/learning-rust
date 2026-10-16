//! Task 2: a week's pay, with overtime.
//!
//! The first 40 hours are paid at `rate_cents` per hour. Every hour after that is paid at one and
//! a half times the rate, rounded down to a whole cent. Money is in cents, so there are no
//! fractions to go wrong.
//!
//! Split the work: write a private helper `fn overtime_hours(hours: u32) -> u32` and use it.

const STANDARD_WEEK_HOURS: u32 = 40;

pub fn weekly_pay_cents(hours: u32, rate_cents: u64) -> u64 {
    let overtime = overtime_hours(hours);
    let standard = hours - overtime;
    u64::from(standard) * rate_cents + u64::from(overtime) * rate_cents * 3 / 2
}

fn overtime_hours(hours: u32) -> u32 {
    hours.saturating_sub(STANDARD_WEEK_HOURS)
}

#[cfg(test)]
mod tests {
    use super::overtime_hours;

    #[test]
    fn no_overtime_up_to_forty_hours() {
        assert_eq!(overtime_hours(0), 0);
        assert_eq!(overtime_hours(40), 0);
    }

    #[test]
    fn counts_hours_past_forty() {
        assert_eq!(overtime_hours(45), 5);
    }
}
