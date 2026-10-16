//! Given: the course's sample data. A fake catalogue, a fake requirements matrix, and the fake
//! staff in the course's running story. Nobody here is real.
//!
//! The story: **Amina Wanjiru** (staff number 48213) is a ramp agent at the Nairobi station. She
//! wants to move to cargo handling. Can she start, and who needs to be told what? Weeks 4 to 8
//! answer that, each from a different part of TWA.

use std::collections::BTreeMap;

use chrono::NaiveDate;

use crate::model::{Catalogue, Holding, QualificationType, RequirementsMatrix};

// One argument per field keeps each sample type on one readable line below.
#[allow(clippy::too_many_arguments)]
fn qualification(
    code: &str,
    name: &str,
    validity_months: u32,
    grace_days: u32,
    warn_days: u32,
    recency_days: Option<u32>,
    prerequisites: &[&str],
    equivalents: &[&str],
) -> QualificationType {
    QualificationType {
        code: code.to_string(),
        name: name.to_string(),
        validity_months,
        grace_days,
        warn_days,
        recency_days,
        prerequisites: prerequisites.iter().map(|s| s.to_string()).collect(),
        equivalents: equivalents.iter().map(|s| s.to_string()).collect(),
    }
}

/// Five qualification types, with every kind of rule the engine supports.
pub fn catalogue() -> Catalogue {
    Catalogue::from_types(vec![
        qualification("AIRSIDE", "Airside safety", 24, 0, 30, None, &[], &[]),
        qualification(
            "DG",
            "Dangerous goods",
            24,
            30,
            60,
            None,
            &["AIRSIDE"],
            &["DG-ICAO"],
        ),
        qualification(
            "DG-ICAO",
            "Dangerous goods (external ICAO certificate)",
            24,
            0,
            60,
            None,
            &[],
            &[],
        ),
        qualification(
            "FORKLIFT",
            "Forklift licence",
            12,
            14,
            30,
            Some(90),
            &["AIRSIDE"],
            &[],
        ),
        qualification("FIRE", "Fire safety", 12, 0, 30, None, &[], &[]),
    ])
}

/// What two roles require.
pub fn matrix() -> RequirementsMatrix {
    let codes = |list: &[&str]| list.iter().map(|c| c.to_string()).collect::<Vec<_>>();
    let mut matrix = RequirementsMatrix::default();
    matrix
        .roles
        .entry("Cargo".to_string())
        .or_default()
        .insert("Handler".to_string(), codes(&["AIRSIDE", "DG", "FORKLIFT"]));
    matrix
        .roles
        .entry("Ramp".to_string())
        .or_default()
        .insert("Agent".to_string(), codes(&["AIRSIDE", "FIRE"]));
    matrix
}

/// A sample staff member: staff number, name, station code, department.
pub struct SampleStaff {
    pub staff_number: u32,
    pub name: &'static str,
    pub station: &'static str,
    pub department: &'static str,
}

/// The four people in the story.
pub fn staff() -> Vec<SampleStaff> {
    vec![
        SampleStaff {
            staff_number: 48213,
            name: "Amina Wanjiru",
            station: "NBO",
            department: "Ramp",
        },
        SampleStaff {
            staff_number: 10007,
            name: "Kip Ruto",
            station: "MBA",
            department: "Cargo",
        },
        SampleStaff {
            staff_number: 20020,
            name: "Zawadi Njeri",
            station: "NBO",
            department: "Cargo",
        },
        SampleStaff {
            staff_number: 30303,
            name: "Baraka Otieno",
            station: "KIS",
            department: "Ramp",
        },
    ]
}

fn on(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).expect("the sample dates are real dates")
}

fn holding(code: &str, completed_on: NaiveDate) -> Holding {
    Holding {
        code: code.to_string(),
        completed_on,
        last_used_on: None,
    }
}

/// What each person holds. Amina has airside safety and fire safety, and an external DG
/// certificate that expires on 1 November 2026, but no forklift licence.
pub fn holdings() -> BTreeMap<u32, Vec<Holding>> {
    BTreeMap::from([
        (
            48213,
            vec![
                holding("AIRSIDE", on(2025, 3, 10)),
                holding("FIRE", on(2026, 3, 1)),
                holding("DG-ICAO", on(2024, 11, 1)),
            ],
        ),
        (
            10007,
            vec![
                holding("AIRSIDE", on(2026, 1, 1)),
                holding("DG", on(2026, 1, 1)),
                holding("FORKLIFT", on(2026, 9, 1)),
            ],
        ),
        (20020, vec![holding("AIRSIDE", on(2025, 6, 1))]),
        (30303, vec![]),
    ])
}
