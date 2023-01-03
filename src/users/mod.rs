use serde::{Deserialize, Serialize};

pub mod display_names;
pub mod users;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub description: String,
    pub created: String,
    pub isBanned: bool,
    pub externalAppDisplayName: Option<String>,
    pub hasVerifiedBadge: bool,
    pub id: usize,
    pub name: String,
    pub displayName: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialUser {
    pub id: usize,
    pub name: String,
    pub displayName: String,
}
