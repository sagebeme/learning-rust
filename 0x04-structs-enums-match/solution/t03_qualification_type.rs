//! Task 3: a qualification type that can't be created in a broken state.
//!
//! `QualificationType::new` checks its inputs and refuses bad ones:
//! - a blank name (only spaces counts as blank) → `TypeError::EmptyName`
//! - a validity of 0 days → `TypeError::ZeroValidity`
//! - a grace period as long as the validity or longer → `TypeError::GraceTooLong`
//!
//! Store the name trimmed. The fields are private, so the only way to build one is through `new`;
//! give read access with the getter methods.

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
        let name = name.trim();
        if name.is_empty() {
            return Err(TypeError::EmptyName);
        }
        if validity_days == 0 {
            return Err(TypeError::ZeroValidity);
        }
        if grace_days >= validity_days {
            return Err(TypeError::GraceTooLong);
        }
        Ok(QualificationType {
            name: name.to_string(),
            validity_days,
            grace_days,
            recency_days,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn validity_days(&self) -> u32 {
        self.validity_days
    }

    pub fn grace_days(&self) -> u32 {
        self.grace_days
    }

    pub fn recency_days(&self) -> Option<u32> {
        self.recency_days
    }
}
