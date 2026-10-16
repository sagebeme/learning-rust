//! Task 0: TWA's four stations, as an enum.
//!
//! | Variant | Code | City |
//! | --- | --- | --- |
//! | `Nairobi` | `NBO` | Nairobi |
//! | `Mombasa` | `MBA` | Mombasa |
//! | `Kisumu` | `KIS` | Kisumu |
//! | `Eldoret` | `EDL` | Eldoret |
//!
//! `from_code` accepts the code in any case (`"nbo"` works) and returns `None` for anything else.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Station {
    Nairobi,
    Mombasa,
    Kisumu,
    Eldoret,
}

impl Station {
    pub fn code(self) -> &'static str {
        match self {
            Station::Nairobi => "NBO",
            Station::Mombasa => "MBA",
            Station::Kisumu => "KIS",
            Station::Eldoret => "EDL",
        }
    }

    pub fn city(self) -> &'static str {
        match self {
            Station::Nairobi => "Nairobi",
            Station::Mombasa => "Mombasa",
            Station::Kisumu => "Kisumu",
            Station::Eldoret => "Eldoret",
        }
    }

    pub fn from_code(code: &str) -> Option<Station> {
        match code.to_ascii_uppercase().as_str() {
            "NBO" => Some(Station::Nairobi),
            "MBA" => Some(Station::Mombasa),
            "KIS" => Some(Station::Kisumu),
            "EDL" => Some(Station::Eldoret),
            _ => None,
        }
    }
}
