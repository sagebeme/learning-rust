//! Given: the types every task uses.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Station {
    Nairobi,
    Mombasa,
    Kisumu,
    Eldoret,
}

/// One person's record for one qualification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holding {
    pub staff_number: u32,
    pub station: Station,
    pub code: String,
    pub days_left: i64,
}

impl Holding {
    pub fn new(staff_number: u32, station: Station, code: &str, days_left: i64) -> Holding {
        Holding {
            staff_number,
            station,
            code: code.to_string(),
            days_left,
        }
    }
}
