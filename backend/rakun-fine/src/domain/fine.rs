use crate::domain::Institution;
use chrono::{DateTime, Utc};

/// The identification fine
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FineId(pub String);

/// Auto Fine
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Fine {
    pub id: Option<FineId>,
    /// The infraction notice.
    pub ait: u32,
    /// The motivation of the fine.
    pub description: Option<String>,
    /// The provider fine.
    pub institution: Option<String>,
    /// The location where the fine occurred, usually the r
    pub location: Option<Institution>,
    /// The code CTB.
    pub code: Option<String>,
    /// The federation unit where the fine ocurred.
    pub uf: Option<String>,
    /// The city where the fine ocurred.
    pub city: Option<String>,
    /// The amount of the fine.
    pub amount: Option<u32>,
    /// The post date.
    pub posted_at: DateTime<Utc>,
}

impl Fine {
    pub fn new(ait: u32, posted_at: DateTime<Utc>) -> Self {
        Self {
            id: None,
            ait,
            description: None,
            institution: None,
            location: None,
            code: None,
            uf: None,
            city: None,
            amount: None,
            posted_at,
        }
    }

    pub fn new_with_id(id: Option<FineId>, ait: u32, posted_at: DateTime<Utc>) -> Self {
        Self {
            id,
            ait,
            description: None,
            institution: None,
            location: None,
            code: None,
            uf: None,
            city: None,
            amount: None,
            posted_at,
        }
    }
}
