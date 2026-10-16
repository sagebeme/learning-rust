//! Task 0: requirement rules as a tree, with `Box`.
//!
//! A rule is either "holds this qualification", or a combination of other rules. Write:
//! - `is_met(&self, held)`: does someone holding the codes in `held` satisfy the rule?
//! - `conditions(&self)`: how many `Has` leaves the rule contains
//!
//! `Rule::all(Rule::has("FIRE"), Rule::any(Rule::has("DG"), Rule::has("AVSEC")))` means
//! "FIRE, and also DG or AVSEC".

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Has(String),
    All(Box<Rule>, Box<Rule>),
    Any(Box<Rule>, Box<Rule>),
    Not(Box<Rule>),
}

// Given: shorter ways to build rules.
impl Rule {
    pub fn has(code: &str) -> Rule {
        Rule::Has(code.to_string())
    }

    pub fn all(left: Rule, right: Rule) -> Rule {
        Rule::All(Box::new(left), Box::new(right))
    }

    pub fn any(left: Rule, right: Rule) -> Rule {
        Rule::Any(Box::new(left), Box::new(right))
    }

    pub fn negate(rule: Rule) -> Rule {
        Rule::Not(Box::new(rule))
    }
}

impl Rule {
    pub fn is_met(&self, held: &[&str]) -> bool {
        todo!()
    }

    pub fn conditions(&self) -> usize {
        todo!()
    }
}
