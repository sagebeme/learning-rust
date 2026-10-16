//! Task 3: a qualification type that can't be created in a broken state.
//!
//! `QualificationType::new` checks its inputs and refuses bad ones:
//! - a blank name (only spaces counts as blank) → `TypeError::EmptyName`
//! - a validity of 0 days → `TypeError::ZeroValidity`
//! - a grace period as long as the validity or longer → `TypeError::GraceTooLong`
//!
//! Store the name trimmed. The fields are private, so the only way to build one is through `new`;
//! give read access with the getter methods.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeError {
    EmptyName,
    ZeroValidity,
    GraceTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationType {
    name: String,
    validity_days: u32,
    grace_days: u32,
    recency_days: Option<u32>,
}

impl QualificationType {
    pub fn new(
        name: &str,
        validity_days: u32,
        grace_days: u32,
        recency_days: Option<u32>,
    ) -> Result<QualificationType, TypeError> {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn validity_days(&self) -> u32 {
        todo!()
    }

    pub fn grace_days(&self) -> u32 {
        todo!()
    }

    pub fn recency_days(&self) -> Option<u32> {
        todo!()
    }
}
