use std::{collections::BTreeSet, str::FromStr};

use crate::context::oauth::error::DomainError;

pub struct Scope(BTreeSet<String>);

impl Scope {
    pub fn new(set: BTreeSet<String>) -> Self {
        Scope(set)
    }
    pub fn parse(s: String) -> Self {
        if s.trim().is_empty() {
            return Scope(BTreeSet::new());
        }
        let set = s.split_whitespace().map(|s| s.to_string()).collect();
        Scope(set)
    }
    pub fn contains(&self, permission: &str) -> bool {
        self.0.contains(permission)
    }
    
    pub fn is_set_of(&self, permissions: &Scope) -> bool {
        self.0.is_subset(&permissions.0)
    }
    pub fn union(&self, other: &Scope) -> Scope {
        let mut new_set = self.0.clone();
        new_set.extend(other.0.iter().cloned());
        Scope(new_set)
    }
}

impl FromStr for Scope {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Scope::parse(s.to_string()))
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.iter().cloned().collect::<Vec<String>>().join(" ");
        write!(f, "{}", s)
    }
}