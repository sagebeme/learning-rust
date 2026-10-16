use traits_dates_testing::t100_countdown::Countdown;

#[test]
fn prints_each_kind_of_countdown() {
    let cases = [
        (14, "in 14 days"),
        (2, "in 2 days"),
        (1, "tomorrow"),
        (0, "today"),
        (-1, "yesterday"),
        (-4, "4 days ago"),
    ];
    for (days, expected) in cases {
        assert_eq!(Countdown { days }.to_string(), expected, "for {days}");
    }
}

#[test]
fn works_inside_format() {
    let line = format!("Fire safety expires {}", Countdown { days: 3 });
    assert_eq!(line, "Fire safety expires in 3 days");
}
