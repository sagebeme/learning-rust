//! Given: the data the server works with, and the state every handler shares.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

pub const STATIONS: [&str; 4] = ["NBO", "MBA", "KIS", "EDL"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Employee {
    pub staff_number: u32,
    pub name: String,
    pub station: String,
    pub department: String,
    /// Qualification codes currently held, sorted.
    pub codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NewEmployee {
    pub staff_number: u32,
    pub name: String,
    pub station: String,
    pub department: String,
}

/// Shared by every request. Cloning it is cheap: it only copies the `Arc` handles.
#[derive(Clone)]
pub struct AppState {
    pub employees: Arc<RwLock<BTreeMap<u32, Employee>>>,
    /// (department, role) → required codes
    pub requirements: Arc<BTreeMap<(String, String), Vec<String>>>,
    /// The bearer token every request except `/health` must send.
    pub token: Arc<str>,
}

impl AppState {
    /// A state with fake sample staff and requirements, for the tests and for trying it out.
    pub fn with_sample_data(token: &str) -> AppState {
        let employee =
            |staff_number, name: &str, station: &str, department: &str, codes: &[&str]| {
                let employee = Employee {
                    staff_number,
                    name: name.to_string(),
                    station: station.to_string(),
                    department: department.to_string(),
                    codes: codes.iter().map(|c| c.to_string()).collect(),
                };
                (staff_number, employee)
            };
        let employees = BTreeMap::from([
            employee(48213, "Amina Wanjiru", "NBO", "Ramp", &["AIRSIDE", "FIRE"]),
            employee(
                10007,
                "Kip Ruto",
                "MBA",
                "Cargo",
                &["AIRSIDE", "DG", "FORKLIFT"],
            ),
            employee(20020, "Zawadi Njeri", "NBO", "Cargo", &["AIRSIDE"]),
            employee(30303, "Baraka Otieno", "KIS", "Ramp", &[]),
        ]);
        let requirement = |department: &str, role: &str, codes: &[&str]| {
            (
                (department.to_string(), role.to_string()),
                codes.iter().map(|c| c.to_string()).collect(),
            )
        };
        let requirements = BTreeMap::from([
            requirement("Cargo", "Handler", &["FORKLIFT", "AIRSIDE", "DG"]),
            requirement("Ramp", "Agent", &["AIRSIDE", "FIRE"]),
        ]);
        AppState {
            employees: Arc::new(RwLock::new(employees)),
            requirements: Arc::new(requirements),
            token: Arc::from(token),
        }
    }
}
