//! Task 0: requirement rules as a tree, with `Box`.
//!
//! A rule is either "holds this qualification", or a combination of other rules. Write:
//! - `is_met(&self, held)`: does someone holding the codes in `held` satisfy the rule?
//! - `conditions(&self)`: how many `Has` leaves the rule contains
//!
//! `Rule::all(Rule::has("FIRE"), Rule::any(Rule::has("DG"), Rule::has("AVSEC")))` means
//! "FIRE, and also DG or AVSEC".

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
        match self {
            Rule::Has(code) => held.contains(&code.as_str()),
            Rule::All(left, right) => left.is_met(held) && right.is_met(held),
            Rule::Any(left, right) => left.is_met(held) || right.is_met(held),
            Rule::Not(rule) => !rule.is_met(held),
        }
    }

    pub fn conditions(&self) -> usize {
        match self {
            Rule::Has(_) => 1,
            Rule::All(left, right) | Rule::Any(left, right) => {
                left.conditions() + right.conditions()
            }
            Rule::Not(rule) => rule.conditions(),
        }
    }
}
