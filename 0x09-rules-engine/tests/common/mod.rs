//! Shared test data: a small, fake catalogue and requirements matrix.
#![allow(dead_code)]

use chrono::NaiveDate;
use rules_engine::model::{Catalogue, Holding, QualificationType, RequirementsMatrix};

pub fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

/// The day every test is checked on.
pub fn today() -> NaiveDate {
    d(2026, 10, 16)
}

pub fn qualification(
    code: &str,
    validity_months: u32,
    grace_days: u32,
    warn_days: u32,
    recency_days: Option<u32>,
    prerequisites: &[&str],
    equivalents: &[&str],
) -> QualificationType {
    QualificationType {
        code: code.to_string(),
        name: format!("{code} (fake)"),
        validity_months,
        grace_days,
        warn_days,
        recency_days,
        prerequisites: prerequisites.iter().map(|s| s.to_string()).collect(),
        equivalents: equivalents.iter().map(|s| s.to_string()).collect(),
    }
}

pub fn catalogue_types() -> Vec<QualificationType> {
    vec![
        qualification("AIRSIDE", 24, 0, 30, None, &[], &[]),
        qualification("DG", 24, 30, 60, None, &["AIRSIDE"], &["DG-ICAO"]),
        qualification("DG-ICAO", 24, 0, 60, None, &[], &[]),
        qualification("FORKLIFT", 12, 14, 30, Some(90), &["AIRSIDE"], &[]),
        qualification("FIRE", 12, 0, 30, None, &[], &[]),
    ]
}

pub fn catalogue() -> Catalogue {
    Catalogue::from_types(catalogue_types())
}

pub fn matrix() -> RequirementsMatrix {
    let mut matrix = RequirementsMatrix::default();
    matrix.roles.entry("Cargo".into()).or_default().insert(
        "Handler".into(),
        vec!["FORKLIFT".into(), "AIRSIDE".into(), "DG".into()],
    );
    matrix
        .roles
        .entry("Ramp".into())
        .or_default()
        .insert("Agent".into(), vec!["AIRSIDE".into(), "FIRE".into()]);
    matrix
}

pub fn holding(code: &str, completed_on: NaiveDate) -> Holding {
    Holding {
        code: code.to_string(),
        completed_on,
        last_used_on: None,
    }
}

pub fn used(code: &str, completed_on: NaiveDate, last_used_on: NaiveDate) -> Holding {
    Holding {
        code: code.to_string(),
        completed_on,
        last_used_on: Some(last_used_on),
    }
}
