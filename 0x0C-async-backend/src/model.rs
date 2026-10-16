//! Given: the data the server works with, and the state every handler shares.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use twa_engine::model::{Catalogue, Holding, RequirementsMatrix};

pub const STATIONS: [&str; 4] = ["NBO", "MBA", "KIS", "EDL"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Employee {
    pub staff_number: u32,
    pub name: String,
    pub station: String,
    pub department: String,
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
    /// staff number → their qualifications, in the engine's own type. (In the real server these
    /// come from the 0x0B database.)
    pub holdings: Arc<RwLock<BTreeMap<u32, Vec<Holding>>>>,
    pub catalogue: Arc<Catalogue>,
    pub matrix: Arc<RequirementsMatrix>,
    /// The bearer token every request except `/health` must send.
    pub token: Arc<str>,
}

impl AppState {
    /// The course's sample staff, holdings, catalogue and matrix (all fake), from the engine.
    pub fn with_sample_data(token: &str) -> AppState {
        let employees = twa_engine::sample::staff()
            .into_iter()
            .map(|s| {
                let employee = Employee {
                    staff_number: s.staff_number,
                    name: s.name.to_string(),
                    station: s.station.to_string(),
                    department: s.department.to_string(),
                };
                (s.staff_number, employee)
            })
            .collect();
        AppState {
            employees: Arc::new(RwLock::new(employees)),
            holdings: Arc::new(RwLock::new(twa_engine::sample::holdings())),
            catalogue: Arc::new(twa_engine::sample::catalogue()),
            matrix: Arc::new(twa_engine::sample::matrix()),
            token: Arc::from(token),
        }
    }
}
