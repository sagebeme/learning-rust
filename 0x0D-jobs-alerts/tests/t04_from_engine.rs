mod common;

use std::collections::BTreeMap;

use common::d;
use jobs_alerts::model::{Channel, Contact};
use jobs_alerts::t00_plan_alerts::plan_alerts;
use jobs_alerts::t04_from_engine::expiring_items;
use twa_engine::model::Holding;

fn holding(code: &str, completed_on: chrono::NaiveDate) -> Holding {
    Holding {
        code: code.to_string(),
        completed_on,
        last_used_on: None,
    }
}

#[test]
fn amina_s_items_come_from_the_engine() {
    let items = expiring_items(
        &twa_engine::sample::catalogue(),
        &twa_engine::sample::holdings(),
    );
    let amina: Vec<(&str, chrono::NaiveDate)> = items
        .iter()
        .filter(|i| i.staff_number == 48213)
        .map(|i| (i.code.as_str(), i.valid_until))
        .collect();
    assert_eq!(
        amina,
        [
            ("AIRSIDE", d(2027, 3, 10)),
            ("DG-ICAO", d(2026, 11, 1)),
            ("FIRE", d(2027, 3, 1)),
        ]
    );
}

#[test]
fn sorted_by_staff_number_then_code() {
    let items = expiring_items(
        &twa_engine::sample::catalogue(),
        &twa_engine::sample::holdings(),
    );
    let keys: Vec<(u32, &str)> = items
        .iter()
        .map(|i| (i.staff_number, i.code.as_str()))
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
    assert_eq!(keys.first(), Some(&(10007, "AIRSIDE")));
}

#[test]
fn a_renewal_replaces_the_old_certificate() {
    let holdings = BTreeMap::from([(
        1,
        vec![
            holding("FIRE", d(2025, 1, 10)), // the old certificate
            holding("FIRE", d(2026, 1, 10)), // the renewal
        ],
    )]);
    let items = expiring_items(&twa_engine::sample::catalogue(), &holdings);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].valid_until, d(2027, 1, 10));
}

#[test]
fn codes_outside_the_catalogue_are_skipped() {
    let holdings = BTreeMap::from([(1, vec![holding("PILOT", d(2026, 1, 1))])]);
    assert!(expiring_items(&twa_engine::sample::catalogue(), &holdings).is_empty());
}

#[test]
fn on_25_october_amina_is_reminded_about_her_dg_certificate() {
    // The whole pipeline: engine → items → planned alerts.
    let items = expiring_items(
        &twa_engine::sample::catalogue(),
        &twa_engine::sample::holdings(),
    );
    let contacts = [Contact {
        staff_number: 48213,
        email: None,
        phone: Some("+254700000001".to_string()),
        whatsapp_opt_in: true,
        push_token: None,
    }];
    let alerts = plan_alerts(d(2026, 10, 25), &items, &contacts);
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].code, "DG-ICAO");
    assert_eq!(alerts[0].channel, Channel::WhatsApp);
    assert_eq!(alerts[0].message, "Your DG-ICAO expires in 7 days.");
}
