use serde_json::json;
use tauri_core::t100_events::{EXPIRING_EVENT, expiring_payload, is_valid_event_name};

#[test]
fn builds_the_message_for_each_case() {
    let cases = [
        (3, "DG expires in 3 days"),
        (1, "DG expires tomorrow"),
        (0, "DG expires today"),
        (-1, "DG expired yesterday"),
        (-4, "DG expired 4 days ago"),
    ];
    for (days_left, message) in cases {
        assert_eq!(expiring_payload(48213, "DG", days_left).message, message);
    }
}

#[test]
fn the_payload_reaches_the_screen_in_camel_case() {
    assert_eq!(
        serde_json::to_value(expiring_payload(48213, "FIRE", 7)).unwrap(),
        json!({
            "staffNumber": 48213,
            "code": "FIRE",
            "daysLeft": 7,
            "message": "FIRE expires in 7 days"
        })
    );
}

#[test]
fn checks_event_names() {
    assert!(is_valid_event_name(EXPIRING_EVENT));
    assert!(is_valid_event_name("sync/finished:station_NBO"));
    assert!(!is_valid_event_name(""));
    assert!(!is_valid_event_name("qualification expiring"));
    assert!(!is_valid_event_name("expiring!"));
    assert!(!is_valid_event_name("muda-umeisha-ñ"));
}
