//! Task 1: a staff number's check digit.
//!
//! Add up the digits of the staff number and return the last digit of that sum.
//! `check_digit(48213)`: 4 + 8 + 2 + 1 + 3 = 18, so the answer is `8`.
//!
//! Work with the number itself (`% 10` and `/ 10`), not by turning it into text.

pub fn check_digit(staff_number: u32) -> u32 {
    let mut remaining = staff_number;
    let mut sum = 0;
    while remaining > 0 {
        sum += remaining % 10;
        remaining /= 10;
    }
    sum % 10
}
