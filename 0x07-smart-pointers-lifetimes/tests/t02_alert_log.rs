use smart_pointers_lifetimes::t02_alert_log::AlertLog;

#[test]
fn records_messages_in_order() {
    let log = AlertLog::new();
    log.record("DG expires in 30 days: 48213");
    log.record("FIRE expired: 10007");
    assert_eq!(
        log.entries(),
        ["DG expires in 30 days: 48213", "FIRE expired: 10007"]
    );
}

#[test]
fn every_handle_sees_the_same_log() {
    let log = AlertLog::new();
    let from_job = log.handle();
    let from_screen = log.handle();

    from_job.record("from the daily job");
    from_screen.record("from the screen");

    assert_eq!(log.entries(), ["from the daily job", "from the screen"]);
    assert_eq!(from_job.entries(), log.entries());
}

#[test]
fn counts_live_handles() {
    let log = AlertLog::new();
    assert_eq!(log.handles(), 1);
    let second = log.handle();
    assert_eq!(log.handles(), 2);
    drop(second);
    assert_eq!(log.handles(), 1);
}

#[test]
fn entries_is_a_copy() {
    let log = AlertLog::new();
    log.record("one");
    let mut snapshot = log.entries();
    snapshot.push("not in the log".to_string());
    assert_eq!(log.entries(), ["one"]);
}
