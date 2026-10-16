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

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Station {
    Nairobi,
    Mombasa,
    Kisumu,
    Eldoret,
}

impl Station {
    pub fn code(self) -> &'static str {
        todo!()
    }

    pub fn city(self) -> &'static str {
        todo!()
    }

    pub fn from_code(code: &str) -> Option<Station> {
        todo!()
    }
}
